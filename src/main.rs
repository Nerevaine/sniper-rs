use {
    clap::{Parser},
    futures::{sink::SinkExt, stream::StreamExt},
    log::info,
    std::env,
    std::{
        collections::HashMap
    },
    tokio::time::{interval, Duration},
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::GeyserGrpcClient,
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestPing, SubscribeUpdatePong,SubscribeRequestFilterAccounts,
        SubscribeUpdateSlot, SubscribeUpdateAccountInfo
    },
    serde_json::{json, Value},
    solana_sdk::pubkey::Pubkey,
    bs58,
    hex,
};
// Define a struct to parse command-line arguments using the `clap` crate.
#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Service endpoint
    #[clap(short, long, default_value_t = String::from("https://solana-yellowstone-grpc.publicnode.com"))]
    endpoint: String,

    // Optional x-token for authentication
    #[clap(long)]
    x_token: Option<String>,
}

// Main asynchronous entry point of the application
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up environment variables and initialize the logger
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();

    // Parse command-line arguments
    let args = Args::parse();

    // Build and connect the gRPC client
    let mut client = GeyserGrpcClient::build_from_shared(args.endpoint)?
        .x_token(args.x_token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    // Define a type alias for account filters
    type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;
    let mut accounts: AccountFilterMap = HashMap::new();

    // Define account owners to filter
    let pump = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string();
    let raydium = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string();
    accounts.insert(
        "client".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![pump.clone(), raydium],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    // Run two asynchronous tasks concurrently
    futures::try_join!(
        // Task to send subscription requests and periodic pings
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
        // Task to process incoming messages from the stream
        async move {
            while let Some(message) = stream.next().await {
                match message?.update_oneof.expect("valid message") {
                    UpdateOneof::Slot(SubscribeUpdateSlot { slot, .. }) => {
                        info!("slot received: {slot}");
                    }
                    UpdateOneof::Ping(_msg) => {
                        info!("ping received");
                    }
                    UpdateOneof::Pong(SubscribeUpdatePong { id }) => {
                        info!("pong received: id#{id}");
                    }
                    UpdateOneof::Account(_msg) => {
                        let account = _msg.account.ok_or(anyhow::anyhow!("no account in the message"))?;
                        info!("account received");
                        let ammkey = Pubkey::try_from(account.pubkey).map_err(|_| anyhow::anyhow!("invalid account pubkey"))?.to_string();
                        info!("ammkey {}", ammkey);
                        let owner = Pubkey::try_from(account.owner).map_err(|_| anyhow::anyhow!("invalid account owner"))?.to_string();
                        info!("owner {}", owner);

                        if owner == pump {
                            // Deserialize PumpLayout structure
                            #[derive(Debug)]
                            struct PumpLayout {
                                discriminator: u64,
                                pool_bump: u8,
                                index: u16,
                                creator: [u8; 32],
                                base_mint: [u8; 32],
                                quote_mint: [u8; 32],
                                lp_mint: [u8; 32],
                                base_vault: [u8; 32],
                                quote_vault: [u8; 32],
                            }
                            
                            let pump_data: PumpLayout = unsafe {
                                std::ptr::read(account.data.as_ptr() as *const _)
                            };
                            
                            info!("PumpLayout data:");
                            info!("  discriminator: {}", pump_data.discriminator);
                            info!("  pool_bump: {}", pump_data.pool_bump);
                            info!("  index: {}", pump_data.index);
                            info!("  creator: {}", bs58::encode(pump_data.creator).into_string());
                            info!("  base_mint: {}", bs58::encode(pump_data.base_mint).into_string());
                            info!("  quote_mint: {}", bs58::encode(pump_data.quote_mint).into_string());
                            info!("  lp_mint: {}", bs58::encode(pump_data.lp_mint).into_string());
                            info!("  base_vault: {}", bs58::encode(pump_data.base_vault).into_string());
                            info!("  quote_vault: {}", bs58::encode(pump_data.quote_vault).into_string());
                        }
                    }
                    msg => anyhow::bail!("received unexpected message: {msg:?}"),
                }
            }
            Ok::<(), anyhow::Error>(())
        }
    )?;

    Ok(())
}