use log;
use crate::dex::pump::{PumpLayout, print_pump_layout};
use crate::dex::raydium::RaydiumLayout;
use crate::dex::raydium::{print_raydium_layout, MarketLayout, process_market};
use crate::dex::raydiumCp::{RaydiumCpLayout, print_raydium_cp_layout};

/// 处理 pump 类型账户数据
/// - ammkey: 账户公钥字符串
/// - buffer: 账户原始数据字节
pub fn pump(ammkey: String, buffer: Vec<u8>) {
    log::info!("pump ammkey: {}", ammkey);

    // 特殊 AMM 跳过处理
    const SPECIAL_AMM_KEY: &str = "ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw";
    if ammkey == SPECIAL_AMM_KEY {
        return;
    }

    // 尝试解析 buffer 为 PumpLayout 结构体
    match PumpLayout::try_from_slice_manual(buffer.as_slice()) {
        Some(pump_data) => print_pump_layout(&pump_data), // 解析成功则打印
        None => log::error!("无法解析 pump 数据"),         // 失败则报错
    }
}

/// 处理 raydium 类型账户数据，仅打印信息
/// - ammkey: 账户公钥字符串
/// - buffer: 账户原始数据字节
pub fn raydium(pubkey: String, buffer:Vec<u8>){
    if buffer.len() == 752 {
        match RaydiumLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(raydium_data) => print_raydium_layout(pubkey, &raydium_data),
            None => log::error!("无法解析 raydium 数据"),
        }
    } else if buffer.len() == 388 {
        // 处理 serum 市场账户数据
        match MarketLayout::slice_market(buffer.as_slice()) {
            Some(market_data) => process_market(pubkey, &market_data),  // process_market 函数会处理退出
            None => log::error!("无法解析 market 数据"),
        }
    } else {
        log::error!("未知的数据长度: {}", buffer.len());
    }
}

/// 处理 Raydium Concentrated Pool 类型账户数据，仅打印信息
pub fn raydium_cp(pubkey: String, buffer: Vec<u8>) {
    if buffer.len() == 4075 || buffer.len() == 637 {
        match RaydiumCpLayout::try_from_slice_manual(buffer.as_slice()) {
            Some(cp_data) => print_raydium_cp_layout(pubkey, &cp_data),
            None => log::error!("无法解析 Raydium CP 数据"),
        }
    } else {
        log::error!("Raydium CP 数据长度错误: {}", buffer.len());
    }
}

