use tonic::transport::Channel;
use yellowstone_grpc_proto::geyser::geyser_client::GeyserClient;
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeUpdate};
use yellowstone_grpc_proto::prelude::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterAccountsFilter,
    subscribe_request_filter_accounts_filter::Filter as AccountsFilter,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 gRPC 客户端连接
    let channel = Channel::from_static("https://api.rpcpool.com:443")
        .connect()
        .await?;

    let mut client = GeyserClient::new(channel);

    // 这里可以调用客户端方法，例如 get_slot、subscribe 等

    subscribe_accounts(&mut client).await?;

    Ok(())
}

async fn subscribe_accounts(client: &mut GeyserClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;

    // 示例：订阅某个账户更新
    let pubkey = "SysvarC1ock11111111111111111111111111111111".to_string();

    let mut accounts = HashMap::new();
    accounts.insert(
        "client".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![pubkey],
            owner: vec![],
            filters: vec![
                SubscribeRequestFilterAccountsFilter {
                    filter: Some(AccountsFilter::TokenAccountState(true)),
                }
            ],
            nonempty_txn_signature: None,
        },
    );

    let request = SubscribeRequest {
        accounts,
        commitment: Some(CommitmentLevel::Processed as i32),
        ..Default::default()
    };

    let (mut tx, mut stream) = client.subscribe_with_request(Some(request)).await?;
    println!("🚀 Stream started.");

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
