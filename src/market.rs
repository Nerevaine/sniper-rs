use log;
use crate::pump::{PumpLayout, print_pump_layout};
use crate::layout::RaydiumLayout;
use crate::raydium::{print_raydium_layout, MarketLayout, process_market};

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
        match MarketLayout::slice_market(buffer.as_slice()) {
            Some(market_data) => process_market(pubkey, &market_data),
            None => log::error!("无法解析 market 数据"),
        }
    } else {
        log::error!("未知的数据长度: {}", buffer.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raydium_invalid_buffer() {
        let pubkey = String::from("test_key");
        let buffer = vec![0; 100]; // 无效长度的缓冲区
        raydium(pubkey, buffer); // 应该记录错误日志
    }
}