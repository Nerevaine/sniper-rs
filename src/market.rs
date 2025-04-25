use log;
// 从 pump 模块导入 PumpLayout 结构体和 print_pump_layout 函数，用于解析和打印 pump 数据
use crate::pump::{PumpLayout, print_pump_layout};

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
pub fn raydium(ammkey: String, buffer: Vec<u8>) {
    log::info!("raydium ammkey: {}", ammkey);
    log::info!("Buffer length: {}", buffer.len());
}