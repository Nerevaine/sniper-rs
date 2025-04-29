//! Rust gRPC DEX 处理库
//! 
//! 该库提供了处理各种 DEX 协议数据的功能

pub mod common;
pub mod config;  // 添加 config 模块
pub mod dex;
pub mod dex_processor;  // 注意这里改成 pub

// 可以选择重导出常用的模块项
pub use dex_processor::{pump, raydium_lp_v4, raydium_cpmm, raydium_clmm, solfi, meteora_dlmm};