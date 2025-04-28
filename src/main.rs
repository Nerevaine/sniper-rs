use clap::Parser;
use futures::{sink::SinkExt, stream::StreamExt};
use log::info;
use std::env;
use std::collections::HashMap;
use tokio::time::{interval, Duration};
use tonic::transport::channel::ClientTlsConfig;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestPing, SubscribeUpdatePong, SubscribeRequestFilterAccounts,
};
use solana_sdk::pubkey::Pubkey;
mod market;
mod common {
    pub mod layout;
}
mod dex {
    pub mod pump;
    pub mod raydium_lp_v4;
    pub mod raydium_cpmm;
    pub mod raydium_clmm;
}

/// 命令行参数结构体
#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Service endpoint
    #[clap(short, long, default_value_t = String::from("https://solana-yellowstone-grpc.publicnode.com"))]
    endpoint: String,

    /// 可选的 x_token
    #[clap(long)]
    x_token: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 设置日志等级，初始化日志，然后输出一条“bot启动中”的日志，方便你在终端看到程序启动了
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();
    log::info!("bot启动中");

    // 解析命令行参数
    let args = Args::parse();

    // 构建 Yellowstone gRPC 客户端
    let mut client = GeyserGrpcClient::build_from_shared(args.endpoint)?
        .x_token(args.x_token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    // 账户过滤器映射类型定义
    type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;
    let mut accounts: AccountFilterMap = HashMap::new();

    // 需要监听的账户 owner
    let pump = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string();
    let raydium_lp_v4 = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string();
    let raydium_cpmm = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C".to_string();
    let raydium_clmm = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string();
    
    accounts.insert(
        "client".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![pump.clone(), raydium_lp_v4.clone(), raydium_cpmm.clone(), raydium_clmm.clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    // 并发执行：发送订阅请求和处理消息流
    futures::try_join!(
        // 发送订阅请求和定时 ping
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
                // 定时发送 ping 保持连接
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
        // 处理订阅消息流
        async move {
            while let Some(message) = stream.next().await {
                match message?.update_oneof.expect("valid message") {
                    UpdateOneof::Ping(_msg) => {
                        info!("ping received");
                    }
                    UpdateOneof::Pong(SubscribeUpdatePong { id }) => {
                        info!("pong received: id#{id}");
                    }
                    UpdateOneof::Account(_msg) => {
                        // 解析账户数据
                        let account = _msg.account.ok_or(anyhow::anyhow!("no account in the message"))?;
                        let ammkey = Pubkey::try_from(account.pubkey).map_err(|_| anyhow::anyhow!("invalid account pubkey"))?.to_string();
                        let owner = Pubkey::try_from(account.owner).map_err(|_| anyhow::anyhow!("invalid account owner"))?.to_string();

                        let buffer = account.data.clone();  
                        // 根据 owner 类型分别处理
                        // if owner == pump {
                        //     market::pump(ammkey.clone(), buffer.clone())
                        // } 
                        // if owner == raydium_lp_v4 {
                        //     market::raydium_lp_v4(ammkey.clone(), buffer.clone())
                        // }  
                        // if owner == raydium_cpmm {
                        //         market::raydium_cpmm(ammkey.clone(), buffer.clone())
                        // } 
                        if owner == raydium_clmm {
                                market::raydium_clmm(ammkey, buffer)
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