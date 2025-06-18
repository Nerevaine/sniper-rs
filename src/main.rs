// src/main.rs
use clap::Parser;
use futures::{sink::SinkExt, stream::StreamExt};
use log::info;
use std::{collections::HashMap, env, str::FromStr};
use tokio::time::{interval, Duration};
use tonic::transport::channel::ClientTlsConfig;

use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_request_filter_accounts_filter::Filter as SubscribeFilterKind,
        subscribe_request_filter_accounts_filter_memcmp::Data as MemcmpData,
        SubscribeRequestFilterAccountsFilter, SubscribeRequestFilterAccountsFilterMemcmp,
    },
    prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterAccounts, SubscribeRequestPing,
    },
};

mod common;
mod dex;
mod dex_processor; // your existing parsers/printers
mod instruction; // contains LIQUIDITY_STATE_LAYOUT_V4 + decode()

// ─────────── constants ───────────────────────────────
const RAYDIUM_PROGRAM_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";
const OPENBOOK_PROGRAM: &str = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX";

// ─── byte offsets inside LIQUIDITY_STATE_LAYOUT_V4 ───────────
use instruction::decoder::LIQUIDITY_STATE_LAYOUT_V4;
use memoffset::offset_of;

const OFFSET_QUOTE_MINT:     u64 = 432;  // TS: quoteMint@432
const OFFSET_MARKET_PROGRAM: u64 = 560;  // TS: marketProg@560
const OFFSET_SWAP_QUOTE_IN:  u64 = 664;  // TS: swapQ@664
const OFFSET_SWAP_BASE_OUT:  u64 = 688;  // TS: swapB@688

// ───────── CLI args ──────────────────────────────────────
#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    /// gRPC endpoint
    #[clap(short, long,
           default_value_t = String::from("https://solana-yellowstone-grpc.publicnode.com"))]
    endpoint: String,

    /// optional x-token header
    #[clap(long)]
    x_token: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    /* ───── logging & args ───── */
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();
    info!("bot");

    let args = Args::parse();

    /* ───── gRPC connection ───── */
    let mut client = GeyserGrpcClient::build_from_shared(args.endpoint)?
        .x_token(args.x_token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;
    let (mut tx, mut stream) = client.subscribe().await?;

    /* ───── build 4 filters ───── */
    let zero = vec![0u8];
    let f_quote = memcmp_filter(OFFSET_QUOTE_MINT, Pubkey::from_str(WSOL_MINT)?);
    let f_market = memcmp_filter(OFFSET_MARKET_PROGRAM, Pubkey::from_str(OPENBOOK_PROGRAM)?);
    let f_swap_q = byte_zero_filter(OFFSET_SWAP_QUOTE_IN);
    let f_swap_b = byte_zero_filter(OFFSET_SWAP_BASE_OUT);

    info!(
        "using filters → quote@{}  market@{}  swapQ@{}  swapB@{}",
        OFFSET_QUOTE_MINT, OFFSET_MARKET_PROGRAM, OFFSET_SWAP_QUOTE_IN, OFFSET_SWAP_BASE_OUT
    );
    /* ───── accounts map ───── */
    let mut accounts: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    accounts.insert(
        "new_lp_v4".into(),
        SubscribeRequestFilterAccounts {
            owner: vec![RAYDIUM_PROGRAM_V4.into()],
            filters: vec![f_quote, f_market, f_swap_q, f_swap_b],
            ..Default::default()
        },
    );

    futures::try_join!(
        /* TX task */
        async move {
            tx.send(SubscribeRequest {
                accounts,
                commitment: Some(CommitmentLevel::Processed as i32),
                ..Default::default()
            })
            .await?;

            let mut ticker = interval(Duration::from_secs(3));
            let mut id: i32 = 0;
            loop {
                ticker.tick().await;
                id += 1;
                tx.send(SubscribeRequest {
                    ping: Some(SubscribeRequestPing { id }),
                    ..Default::default()
                })
                .await?;
            }
            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        },
        async move {
            while let Some(msg) = stream.next().await {
                let msg = msg?; // SubscribeUpdate
                // log every incoming account message
                if let Some(UpdateOneof::Account(acc)) = &msg.update_oneof {
                    if let Some(acct) = &acc.account {
                        let key = Pubkey::try_from(&acct.pubkey[..])?.to_string();
                        let data = &acct.data;
                        info!("→ got {} bytes for account {}", data.len(), key);
        
                        // now attempt decode V4 layout
                        let mut slice: &[u8] = data;
                        match LIQUIDITY_STATE_LAYOUT_V4::decode(&mut slice) {
                            Ok(state) => {
                                info!(
                                    "counters → swapQuoteIn={}  swapBaseOut={}",
                                    state.swapQuoteInAmount,
                                    state.swapBaseOutAmount,
                                );
                                let pass_q = state.swapQuoteInAmount == 0;
                                let pass_b = state.swapBaseOutAmount == 0;
                                if pass_q && pass_b {
                                    info!("NEW LP-V4 POOL: {}", key);
                                    info!("  baseMint:     {}", state.baseMint);
                                    info!("  lpMint:       {}", state.lpMint);
                                    info!("  marketId:     {}", state.marketId);
                                    info!("  poolOpenTime: {}", state.poolOpenTime);
                                    dex_processor::raydium_lp_v4(key, acct.data.clone());
                                } else {
                                    info!(
                                        "skipped {}  pass_q={}  pass_b={}",
                                        key, pass_q, pass_b
                                    );
                                }
                            }
                            Err(err) => {
                                info!("⚠ V4 decode failed for {}: {}", key, err);
                            }
                        }
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        }
    )?;

    Ok(())
}

// ─── helper constructors ───────────────────────────────────
fn memcmp_filter(offset: u64, key: Pubkey) -> SubscribeRequestFilterAccountsFilter {
    // pubkeys → BASE-58 (same as JS)
    SubscribeRequestFilterAccountsFilter {
        filter: Some(SubscribeFilterKind::Memcmp(
            SubscribeRequestFilterAccountsFilterMemcmp {
                offset,
                data: Some(MemcmpData::Base58(key.to_string())),
            },
        )),
    }
}

fn byte_zero_filter(offset: u64) -> SubscribeRequestFilterAccountsFilter {
    // zero hacks → raw BYTES (exactly like the JS Uint8Array)
    SubscribeRequestFilterAccountsFilter {
        filter: Some(SubscribeFilterKind::Memcmp(
            SubscribeRequestFilterAccountsFilterMemcmp {
                offset,
                data: Some(MemcmpData::Bytes(vec![0u8])),   // <── one single zero byte
            },
        )),
    }
}