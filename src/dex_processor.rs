use log;

use crate::dex::raydium_lp_v4::{RaydiumLpV4Layout, print_raydium_lp_v4_layout, SerumMarketLayout, process_market, RAYDIUM_LP_V4_ACCOUNT_SIZE, SERUM_MARKET_ACCOUNT_SIZE};
use crate::dex::raydium_cpmm::{RaydiumCpLayout, print_raydium_cpmm_layout, RAYDIUM_CP_POOL_SIZE}; 
use crate::dex::raydium_clmm::{RaydiumClmmLayout, print_raydium_clmm_layout, RAYDIUM_CLMM_POOL_SIZE};
use crate::dex::solfi::{SolFiLayout, print_solfi_layout, SOLFI_POOL_SIZE};
use crate::dex::meteora_dlmm::{
    METEORA_DLMM_POOL_SIZE,
    METEORA_DLMM_ORACLE_SIZE,
    METEORA_DLMM_BIN_ARRAY_SIZE,
    MeteoraLayout,
    OracleLayout,
    BinArrayLayout,
    print_meteora_layout,
    print_oracle_layout,
    print_bin_array_layout,
};
use crate::dex::meteora_pools::{MeteoraPools, print_meteora_pools_layout, METEORA_POOLS_SIZE};

// FILTERS


/// 处理 raydium 类型账户数据，仅打印信息
/// - ammkey: 账户公钥字符串
/// - buffer: 账户原始数据字节
pub fn raydium_lp_v4(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == RAYDIUM_LP_V4_ACCOUNT_SIZE {
        match RaydiumLpV4Layout::try_from_slice_manual(buffer.as_slice()) {
            Some(raydium_data) => print_raydium_lp_v4_layout(account_key, &raydium_data),
            None => log::error!("无法解析 raydium 数据: buffer长度 {}", buffer.len()),
        }
    } else if buffer.len() == SERUM_MARKET_ACCOUNT_SIZE {
        // 处理 serum 市场账户数据
        match SerumMarketLayout::slice_market(buffer.as_slice()) {
            Some(market_data) => process_market(account_key, &market_data),
            None => log::error!("无法解析 market 数据: buffer长度 {}", buffer.len()),
        }
    } else {
        log::error!("未知的数据长度: {}, 期望 {} 或 {}", buffer.len(), RAYDIUM_LP_V4_ACCOUNT_SIZE, SERUM_MARKET_ACCOUNT_SIZE);
    }
}

/// 处理 Raydium Concentrated Pool 类型账户数据，仅打印信息
pub fn raydium_cpmm(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == RAYDIUM_CP_POOL_SIZE {
        match RaydiumCpLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(cp_data) => print_raydium_cpmm_layout(account_key, &cp_data),
            None => log::error!("无法解析 Raydium CP 数据: buffer长度 {}", buffer.len()),
        }
    } else {
        log::error!("Raydium CP 数据长度错误: {}, 期望 {}", buffer.len(), RAYDIUM_CP_POOL_SIZE);
    }
}

/// 处理 Raydium CLMM Pool 类型账户数据，仅打印信息
pub fn raydium_clmm(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == RAYDIUM_CLMM_POOL_SIZE {
        match RaydiumClmmLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(clmm_data) => print_raydium_clmm_layout(account_key, &clmm_data),
            None => log::error!("无法解析 Raydium CLMM 数据: buffer长度 {}", buffer.len()),
        }
    } else {
        log::error!("Raydium CLMM 数据长度错误: {}, 期望 {}", buffer.len(), RAYDIUM_CLMM_POOL_SIZE);
    }
}

/// 处理 SolFi Pool 类型账户数据，仅打印信息
pub fn solfi(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == SOLFI_POOL_SIZE {
        match SolFiLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(solfi_data) => print_solfi_layout(account_key, &solfi_data),
            None => log::error!("无法解析 SolFi 数据: buffer长度 {}", buffer.len()),
        }
    } else {
        log::error!("SolFi 数据长度错误: {}, 期望 {}", buffer.len(), SOLFI_POOL_SIZE);
    }
}

/// 处理 Meteora DLMM Pool 类型账户数据，仅打印信息
pub fn meteora_dlmm(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == METEORA_DLMM_POOL_SIZE {
        match MeteoraLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(meteora_data) => print_meteora_layout(account_key, &meteora_data),
            None => log::error!("无法解析 Meteora DLMM 数据: buffer长度 {}", buffer.len()),
        }
    } else if buffer.len() == METEORA_DLMM_ORACLE_SIZE {
        match OracleLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(oracle_data) => print_oracle_layout(account_key, &oracle_data),
            None => log::error!("无法解析 Meteora DLMM Oracle 数据: buffer长度 {}", buffer.len()),
        }
    } else if buffer.len() == METEORA_DLMM_BIN_ARRAY_SIZE {
        match BinArrayLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(bin_array_data) => print_bin_array_layout(account_key, &bin_array_data),
            None => log::error!("无法解析 Meteora DLMM BinArray 数据: buffer长度 {}", buffer.len()),
        }
    } else {
        log::error!("未知的 Meteora DLMM 数据类型: 数据长度 = {}", buffer.len());
    }
}

/// 处理 Meteora Pools 类型账户数据，仅打印信息
pub fn meteora_pools(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == METEORA_POOLS_SIZE {
        match MeteoraPools::try_from_slice_manual(buffer.as_slice()) {
            Some(pools_data) => print_meteora_pools_layout(account_key, &pools_data),
            None => log::error!("无法解析 Meteora Pools 数据: buffer长度 {}", buffer.len()),
        }
    } else {
        log::error!("Meteora Pools 数据长度错误: {}, 期望 {}", buffer.len(), METEORA_POOLS_SIZE);
    }
}

