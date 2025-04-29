/// Represents the layout of Raydium data, providing methods for#![allow(unused_imports)]
use log;
use solana_program::pubkey::Pubkey;
use crate::common::binary_reader::{read_pubkey, read_u64};


pub const RAYDIUM_LP_V4_ACCOUNT_SIZE: usize = 752;
pub const SERUM_MARKET_ACCOUNT_SIZE: usize = 388;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RaydiumLpV4Layout {
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

impl RaydiumLpV4Layout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() < RAYDIUM_LP_V4_ACCOUNT_SIZE {
            log::error!("数据长度不足，无法解析 RaydiumLpV4Layout");
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

pub fn print_raydium_lp_v4_layout(ammkey: String, raydium_data: &RaydiumLpV4Layout) {
    log::info!("==================== Raydium LP V4 数据 ====================");
    log::info!("AMM Address: (https://solscan.io/account/{}#anchorData)", ammkey);
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
pub struct SerumMarketLayout {
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

impl SerumMarketLayout {
    pub fn slice_market(data: &[u8]) -> Option<Self> {
        if data.len() < 388 { // 388是Serum市场账户的典型长度
            log::error!("数据长度不足，无法解析 Serum_MarketLayout");
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

pub fn process_market(pubkey: String, market_data: &SerumMarketLayout) {
    log::info!("\n==================== Serum Market Data ====================");
    log::info!("Market Address: {}", pubkey);
    log::info!("Market Flags: {}", market_data.market_flags);
    log::info!("Own Address: {}", market_data.own_address);
    log::info!("Vault Signer Nonce: {}", market_data.vault_signer_nonce);
    log::info!("Base Token Mint: {}", market_data.base_mint);
    log::info!("Quote Token Mint: {}", market_data.quote_mint);
    log::info!("Base Token Vault: {}", market_data.base_vault);
    log::info!("Quote Token Vault: {}", market_data.quote_vault);
    log::info!("Request Queue: {}", market_data.request_queue);
    log::info!("Event Queue: {}", market_data.event_queue);
    log::info!("Bids: {}", market_data.bids);
    log::info!("Asks: {}", market_data.asks);
    log::info!("Base Lot Size: {}", market_data.base_lot_size);
    log::info!("Quote Lot Size: {}", market_data.quote_lot_size);
    log::info!("==========================================================\n");
    std::process::exit(0);
}
