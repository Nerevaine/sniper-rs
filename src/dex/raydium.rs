/// Represents the layout of Raydium data, providing methods for#![allow(unused_imports)]
use log;
use solana_program::pubkey::Pubkey;
use crate::common::layout::{read_pubkey, read_u64};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RaydiumLayout {
    pub baseVault: Pubkey,        // 基础币种金库地址
    pub quoteVault: Pubkey,       // 报价币种金库地址
    pub baseMint: Pubkey,         // 基础币种铸币地址
    pub quoteMint: Pubkey,        // 报价币种铸币地址
    pub lpMint: Pubkey,           // LP 代币铸币地址
    pub openOrders: Pubkey,       // OpenOrders 账户地址
    pub marketId: Pubkey,         // 市场ID
    pub marketProgramId: Pubkey,  // 市场程序ID
    pub targetOrders: Pubkey,     // target orders 账户地址
}

impl RaydiumLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() < 752 {
            log::error!("数据长度不足，无法解析 RaydiumLayout");
            return None;
        }

        // 从 Raydium AMM 账户数据的第 336 字节开始，依次解析各个 Pubkey 字段
        let mut offset = 336;
        
        Some(Self {
            baseVault: read_pubkey(data, &mut offset),        // 基础币种金库地址
            quoteVault: read_pubkey(data, &mut offset),       // 报价币种金库地址
            baseMint: read_pubkey(data, &mut offset),         // 基础币种铸币地址
            quoteMint: read_pubkey(data, &mut offset),        // 报价币种铸币地址
            lpMint: read_pubkey(data, &mut offset),           // LP 代币铸币地址
            openOrders: read_pubkey(data, &mut offset),       // OpenOrders 账户地址
            marketId: read_pubkey(data, &mut offset),         // 市场ID
            marketProgramId: read_pubkey(data, &mut offset),  // 市场程序ID
            targetOrders: read_pubkey(data, &mut offset),     // target orders 账户地址
        })
    }
}

pub fn print_raydium_layout(ammkey: String, raydium_data: &RaydiumLayout) {
    log::info!("\n==================== Raydium AMM 数据 ====================");
    log::info!("AMM Address: {}", ammkey);
    log::info!("Base Token Vault: {}", raydium_data.baseVault);
    log::info!("Quote Token Vault: {}", raydium_data.quoteVault);
    log::info!("Base Token Mint: {}", raydium_data.baseMint);
    log::info!("Quote Token Mint: {}", raydium_data.quoteMint);
    log::info!("LP Token Mint: {}", raydium_data.lpMint);
    log::info!("OpenOrders: {}", raydium_data.openOrders);
    log::info!("Market ID: {}", raydium_data.marketId);
    log::info!("Market Program ID: {}", raydium_data.marketProgramId);
    log::info!("Target Orders: {}", raydium_data.targetOrders);
    log::info!("======================================================\n");
}

#[derive(Debug)]
#[allow(dead_code)]  // Add this line to suppress the warning
pub struct MarketLayout {
    pub market_flags: u64,
    pub own_address: Pubkey,
    pub vault_signer_nonce: u64,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
}

impl MarketLayout {
    pub fn slice_market(data: &[u8]) -> Option<Self> {
        if data.len() < 388 { // 388是Serum市场账户的典型长度
            log::error!("数据长度不足，无法解析 MarketLayout");
            return None;
        }
        let mut offset = 0;
        Some(Self {
            market_flags: read_u64(data, &mut offset),
            own_address: read_pubkey(data, &mut offset),
            vault_signer_nonce: read_u64(data, &mut offset),
            base_mint: read_pubkey(data, &mut offset),
            quote_mint: read_pubkey(data, &mut offset),
            base_vault: read_pubkey(data, &mut offset),
            quote_vault: read_pubkey(data, &mut offset),
            request_queue: read_pubkey(data, &mut offset),
            event_queue: read_pubkey(data, &mut offset),
            bids: read_pubkey(data, &mut offset),
            asks: read_pubkey(data, &mut offset),
            base_lot_size: read_u64(data, &mut offset),
            quote_lot_size: read_u64(data, &mut offset),
            // ...继续解析其他字段
        })
    }
}

pub fn process_market(pubkey: String, market_data: &MarketLayout) {
    log::info!("处理市场数据: {}", pubkey);
    log::info!("{:?}", market_data);
    // 打印市场详细信息
    log::info!("\n==================== 市场数据 ====================");
    log::info!("市场地址: {}", pubkey);
    log::info!("市场标志: {}", market_data.market_flags);
    log::info!("自身地址: {}", market_data.own_address);
    log::info!("签名者随机数: {}", market_data.vault_signer_nonce);
    log::info!("基础代币铸币地址: {}", market_data.base_mint);
    log::info!("报价代币铸币地址: {}", market_data.quote_mint);
    log::info!("基础代币金库: {}", market_data.base_vault);
    log::info!("报价代币金库: {}", market_data.quote_vault);
    log::info!("请求队列: {}", market_data.request_queue);
    log::info!("事件队列: {}", market_data.event_queue);
    log::info!("买单簿: {}", market_data.bids);
    log::info!("卖单簿: {}", market_data.asks);
    log::info!("基础代币最小交易量: {}", market_data.base_lot_size);
    log::info!("报价代币最小交易量: {}", market_data.quote_lot_size);
    log::info!("================================================\n");

    // 退出程序
    std::process::exit(0);
}
