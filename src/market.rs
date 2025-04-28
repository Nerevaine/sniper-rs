use log;
use crate::dex::pump::{PumpLayout, print_pump_layout};
use crate::dex::raydium_lp_v4::{RaydiumLpV4Layout, print_raydium_lp_v4_layout, SerumMarketLayout, process_market, RAYDIUM_LP_V4_ACCOUNT_SIZE, SERUM_MARKET_ACCOUNT_SIZE};
use crate::dex::raydium_cpmm::{RaydiumCpLayout, print_raydium_cpmm_layout, RAYDIUM_CP_POOL_SIZE};
use crate::dex::raydium_clmm::{RaydiumClmmLayout, print_raydium_clmm_layout, RAYDIUM_CLMM_POOL_SIZE};
use crate::dex::solfi::{SolFiLayout, print_solfi_layout, SOLFI_POOL_SIZE};
use crate::dex::meteora_dlmm::{MeteoraLayout, print_meteora_layout, METEORA_DLMM_POOL_SIZE};




/// 处理 pump 类型账户数据
/// - ammkey: 账户公钥字符串
/// - buffer: 账户原始数据字节
pub fn pump(account_key: String, buffer: Vec<u8>) {
    log::info!("pump account_key: {}", account_key);

    // 特殊 AMM 跳过处理
    const SPECIAL_AMM_KEY: &str = "ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw";
    if account_key == SPECIAL_AMM_KEY {
        return;
    }

    // 尝试解析 buffer 为 PumpLayout 结构体
    match PumpLayout::try_from_slice_manual(buffer.as_slice()) {
        Some(pump_data) => print_pump_layout(account_key, &pump_data),
        None => log::error!("无法解析 pump 数据: buffer长度 {}", buffer.len()),
    }
}

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
    } else {
        log::error!("Meteora DLMM 数据长度错误: {}, 期望 {}", buffer.len(), METEORA_DLMM_POOL_SIZE);
    }
}

