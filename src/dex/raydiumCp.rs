use log;
use solana_program::pubkey::Pubkey;
use crate::common::layout::{read_pubkey, read_u64};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RaydiumCpLayout {
    pub baseVault: Pubkey,        // 基础币种金库地址
    pub quoteVault: Pubkey,       // 报价币种金库地址
    pub baseMint: Pubkey,         // 基础币种铸币地址
    pub quoteMint: Pubkey,        // 报价币种铸币地址
    pub lpMint: Pubkey,           // LP 代币铸币地址
    pub baseReserve: u64,         // 基础币种储备量
    pub quoteReserve: u64,        // 报价币种储备量
    pub lpSupply: u64,            // LP 代币总供应量
    pub startTime: u64,           // 池子启动时间
}

impl RaydiumCpLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() < 336 {
            log::error!("数据长度不足，无法解析 RaydiumCpLayout");
            return None;
        }

        // 从 Raydium CP 账户数据的第 8 字节开始，依次解析各个字段
        let mut offset = 8;
        
        Some(Self {
            baseVault: read_pubkey(data, &mut offset),        // 基础币种金库地址
            quoteVault: read_pubkey(data, &mut offset),       // 报价币种金库地址
            baseMint: read_pubkey(data, &mut offset),         // 基础币种铸币地址
            quoteMint: read_pubkey(data, &mut offset),        // 报价币种铸币地址
            lpMint: read_pubkey(data, &mut offset),           // LP 代币铸币地址
            baseReserve: read_u64(data, &mut offset),         // 基础币种储备量
            quoteReserve: read_u64(data, &mut offset),        // 报价币种储备量
            lpSupply: read_u64(data, &mut offset),            // LP 代币总供应量
            startTime: read_u64(data, &mut offset),           // 池子启动时间
        })
    }
}

pub fn print_raydium_cp_layout(ammkey: String, cp_data: &RaydiumCpLayout) {
    log::info!("\n==================== Raydium CP 数据 ====================");
    log::info!("AMM Address: {}", ammkey);
    log::info!("Base Token Vault: {}", cp_data.baseVault);
    log::info!("Quote Token Vault: {}", cp_data.quoteVault);
    log::info!("Base Token Mint: {}", cp_data.baseMint);
    log::info!("Quote Token Mint: {}", cp_data.quoteMint);
    log::info!("LP Token Mint: {}", cp_data.lpMint);
    log::info!("Base Reserve: {}", cp_data.baseReserve);
    log::info!("Quote Reserve: {}", cp_data.quoteReserve);
    log::info!("LP Supply: {}", cp_data.lpSupply);
    log::info!("Start Time: {}", cp_data.startTime);
    log::info!("======================================================\n");
}