use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("项目初始化完成，准备连接 Yellowstone gRPC...");

    let api_key = env::var("HELIUS_API_KEY").expect("需要设置 HELIUS_API_KEY 环境变量");
    let grpc_endpoint = env::var("YELLOWSTONE_GRPC_ENDPOINT").expect("需要设置 YELLOWSTONE_GRPC_ENDPOINT 环境变量");

    println!("Helius API Key (前缀): {}", &api_key[..std::cmp::min(5, api_key.len())]);
    println!("Yellowstone gRPC Endpoint: {}", grpc_endpoint);

    Ok(())
}