use tonic::transport::Channel;
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeUpdate};
use yellowstone_grpc_proto::prelude::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterAccountsFilter,
    subscribe_request_filter_accounts_filter::Filter as AccountsFilter,
};
use futures::StreamExt;
use dotenvy::dotenv; // 导入 dotenvy
use std::env;        // 导入标准库的 env 模块

#[tokio::main] // 使用 tokio 的 main 宏来设置异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 在程序启动时加载 .env 文件中的环境变量
    dotenv().ok(); // .ok() 会忽略加载 .env 文件时可能发生的错误（比如文件不存在）

    println!("项目初始化完成，准备连接 Yellowstone gRPC...");

    // 尝试读取环境变量（仅用于测试）
    let api_key = env::var("HELIUS_API_KEY").expect("需要设置 HELIUS_API_KEY 环境变量");
    let grpc_endpoint = env::var("YELLOWSTONE_GRPC_ENDPOINT").expect("需要设置 YELLOWSTONE_GRPC_ENDPOINT 环境变量");

    println!("Helius API Key (前缀): {}", &api_key[..std::cmp::min(5, api_key.len())]); // 只打印前5个字符，避免泄露
    println!("Yellowstone gRPC Endpoint: {}", grpc_endpoint);

    // TODO: 在这里添加 gRPC 连接逻辑

    Ok(()) // main 函数成功返回
}

async fn subscribe_accounts(client: &mut GeyserClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;

    // 示例：订阅某个账户更新
    let pubkey = "SysvarC1ock11111111111111111111111111111111".to_string();

    let mut accounts = HashMap::new();
    accounts.insert(
        "client".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![pubkey], // 订阅的账户公钥
            owner: vec![], // 账户所有者过滤条件
            filters: vec![
                SubscribeRequestFilterAccountsFilter {
                    filter: Some(AccountsFilter::TokenAccountState(true)), // 账户状态过滤条件
                }
            ],
            nonempty_txn_signature: None, // 交易签名过滤条件
        },
    );

    let request = SubscribeRequest {
        accounts,
        commitment: Some(CommitmentLevel::Processed as i32), // 承诺级别
        ..Default::default()
    };

    // 发送订阅请求并接收流
    let (mut tx, mut stream) = client.subscribe_with_request(Some(request)).await?;
    println!("🚀 Stream started.");

    // 处理接收到的更新消息
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(update) => handle_update(update),
            Err(e) => {
                eprintln!("Stream error: {e}");
                break;
            }
        }
    }

    Ok(())
}

fn handle_update(update: SubscribeUpdate) {
    match update.update_oneof {
        Some(yellowstone_grpc_proto::prelude::subscribe_update::UpdateOneof::Account(account)) => {
            if let Some(info) = account.account {
                println!(
                    "[slot {}] 🪙 account: {} lamports: {}",
                    account.slot,
                    bs58::encode(&info.pubkey).into_string(),
                    info.lamports
                );
            }
        }
        _ => {
            println!("🔔 Other update: {:?}", update);
        }
    }
}
