// main.rs ---------------------------------------------------------------
use clap::Parser;
use futures::{sink::SinkExt, stream::StreamExt};
use log::info;
use memoffset::offset_of;
use mevbot_ws_rust::dex_processor;
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, env, str::FromStr};
use tokio::time::{interval, Duration};
use tonic::transport::channel::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_request_filter_accounts_filter::Filter as SubscribeFilterKind,
        subscribe_request_filter_accounts_filter_memcmp::Data as MemcmpData,
        SubscribeRequestFilterAccountsFilter,
        SubscribeRequestFilterAccountsFilterMemcmp,
    },
    prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterAccounts, SubscribeRequestPing, SubscribeUpdatePong,
    },
};

// 💧 tu layout -----------------------------------------------------------
mod instruction;             
use instruction::decoder::LIQUIDITY_STATE_LAYOUT_V4;

// ────────────────────────────  CONSTANTES  ────────────────────────────────
// ► Se obtienen del struct para no volver a fallar nunca.
const DATASIZE_LIQUIDITY_V4: u64 =
    std::mem::size_of::<LIQUIDITY_STATE_LAYOUT_V4>() as u64;
const OFFSET_QUOTE_MINT:     u64 =
    offset_of!(LIQUIDITY_STATE_LAYOUT_V4, quoteMint)        as u64;
const OFFSET_MARKET_PROGRAM: u64 =
    offset_of!(LIQUIDITY_STATE_LAYOUT_V4, marketProgramId)  as u64;

// WSOL y OpenBook
const WSOL_MINT: &str        = "So11111111111111111111111111111111111111112";
const OPENBOOK_PROGRAM: &str = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX";

// Program IDs de Raydium
const RAYDIUM_AMM_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const RAYDIUM_CPMM:   &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";
const RAYDIUM_CLMM:   &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";

// ¿Ignorar el snapshot inicial?
const SKIP_STARTUP: bool = true;

/// Argumentos CLI
#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Endpoint gRPC
    #[clap(short, long,
           default_value_t = String::from("https://solana-yellowstone-grpc.publicnode.com"))]
    endpoint: String,

    /// Token opcional x-token
    #[clap(long)]
    x_token: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();
    info!("bot启动中");

    let args = Args::parse();

    let mut client = GeyserGrpcClient::build_from_shared(args.endpoint)?
        .x_token(args.x_token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    // ───── Filtros
    let filter_datasize = SubscribeRequestFilterAccountsFilter {
        filter: Some(SubscribeFilterKind::Datasize(DATASIZE_LIQUIDITY_V4)),
    };

    let filter_quote = SubscribeRequestFilterAccountsFilter {
        filter: Some(SubscribeFilterKind::Memcmp(
            SubscribeRequestFilterAccountsFilterMemcmp {
                offset: OFFSET_QUOTE_MINT,
                data: Some(MemcmpData::Bytes(
                    Pubkey::from_str(WSOL_MINT)?.to_bytes().to_vec(),
                )),
            },
        )),
    };

    let filter_market = SubscribeRequestFilterAccountsFilter {
        filter: Some(SubscribeFilterKind::Memcmp(
            SubscribeRequestFilterAccountsFilterMemcmp {
                offset: OFFSET_MARKET_PROGRAM,
                data: Some(MemcmpData::Bytes(
                    Pubkey::from_str(OPENBOOK_PROGRAM)?.to_bytes().to_vec(),
                )),
            },
        )),
    };

    let mut accounts: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    accounts.insert(
        "raydium".into(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![
                RAYDIUM_AMM_V4.into(),
                RAYDIUM_CPMM.into(),
                RAYDIUM_CLMM.into(),
            ],
            filters: vec![filter_datasize, filter_quote, filter_market],
            nonempty_txn_signature: None,
        },
    );

    // ───── Subscripción + pings
    futures::try_join!(
        // TX
        async move {
            subscribe_tx
                .send(SubscribeRequest {
                    accounts,
                    commitment: Some(CommitmentLevel::Processed as i32),
                    ..Default::default()
                })
                .await?;

            let mut timer = interval(Duration::from_secs(3));
            let mut id = 0;
            loop {
                timer.tick().await;
                id += 1;
                subscribe_tx
                    .send(SubscribeRequest {
                        ping: Some(SubscribeRequestPing { id }),
                        ..Default::default()
                    })
                    .await?;
            }
            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        },
        // RX
        async move {
            while let Some(message) = stream.next().await {
                let update = message?.update_oneof.expect("mensaje válido");
                match update {
                    UpdateOneof::Ping(_) => {
                        // info!("ping received");
                    }
                    UpdateOneof::Pong(SubscribeUpdatePong { id }) =>
                        info!("pong received: id#{id}"),

                        UpdateOneof::Account(acc_msg) => {
                            // if you still want to ignore the *initial* snapshot, keep this:
                            if SKIP_STARTUP && acc_msg.is_startup {
                                continue;
                            }
                        
                            let account = acc_msg
                                .account
                                .as_ref()
                                .ok_or_else(|| anyhow::anyhow!("Account message missing"))?;
                        
                            let ammkey = Pubkey::try_from(account.pubkey.as_slice())?.to_string();
                            let owner  = Pubkey::try_from(account.owner.as_slice())?.to_string();
                            let buffer = &account.data;
                        
                            // now attempt decode on *every* incoming update:
                            let mut slice: &[u8] = buffer;
                            match LIQUIDITY_STATE_LAYOUT_V4::decode(&mut slice) {
                                Ok(state) => {
                                    info!(
                                        "► POOL {}  owner={}  status={}  openTime={}  quoteMint={}  marketProg={}",
                                        ammkey,
                                        owner,
                                        state.status,
                                        state.poolOpenTime,
                                        state.quoteMint,
                                        state.marketProgramId,
                                    );
                        
                                    // dispatch to your processors
                                    match owner.as_str() {
                                        RAYDIUM_AMM_V4 => dex_processor::raydium_lp_v4(ammkey.clone(), buffer.clone()),
                                        RAYDIUM_CPMM  => dex_processor::raydium_cpmm (ammkey.clone(), buffer.clone()),
                                        RAYDIUM_CLMM  => dex_processor::raydium_clmm (ammkey.clone(), buffer.clone()),
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    info!("⚠ decode error for {}: {:#?}", ammkey, e);
                                }
                            }
                        }                        

                    msg => anyhow::bail!("mensaje inesperado: {msg:?}"),
                }
            }
            Ok::<(), anyhow::Error>(())
        }
    )?;

    Ok(())
}
