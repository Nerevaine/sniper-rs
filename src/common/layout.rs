#![allow(unused_imports)]  // 允许本文件中存在未使用的导入，不产生编译警告
use solana_program::pubkey::Pubkey; // 导入 Solana 公钥类型

// 定义 Raydium AMM 账户的数据结构，对应链上账户字段布局
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

// 从字节流读取 u8 类型，并推进 offset
pub fn read_u8(data: &[u8], offset: &mut usize) -> u8 {
    let value = data[*offset];
    *offset += 1;
    value
}

// 从字节流读取 u16 类型，并推进 offset
pub fn read_u16(data: &[u8], offset: &mut usize) -> u16 {
    let mut bytes = [0u8; 2];
    bytes.copy_from_slice(&data[*offset..*offset+2]);
    *offset += 2;
    u16::from_le_bytes(bytes)
}

// 从字节流读取 u64 类型，并推进 offset
pub fn read_u64(data: &[u8], offset: &mut usize) -> u64 {
    let bytes = &data[*offset..*offset+8];
    *offset += 8;
    u64::from_le_bytes(bytes.try_into().unwrap())
}

// 从字节流读取 u128 类型，并推进 offset
#[allow(dead_code)]
pub fn read_u128(data: &[u8], offset: &mut usize) -> u128 {
    let bytes = &data[*offset..*offset+16];
    *offset += 16;
    u128::from_le_bytes(bytes.try_into().unwrap())
}

// 从字节流读取 Pubkey 类型（32字节），并推进 offset
pub fn read_pubkey(data: &[u8], offset: &mut usize) -> Pubkey {
    let mut key = [0u8; 32];
    key.copy_from_slice(&data[*offset..*offset+32]);
    *offset += 32;
    Pubkey::new_from_array(key)
}
