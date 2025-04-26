#![allow(unused_imports)]  // 对整个文件生效
use solana_program::pubkey::Pubkey;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RaydiumLayout {
    pub baseVault: Pubkey,
    pub quoteVault: Pubkey,
    pub baseMint: Pubkey,
    pub quoteMint: Pubkey,
    pub lpMint: Pubkey,
    pub openOrders: Pubkey,
    pub marketId: Pubkey,
    pub marketProgramId: Pubkey,
    pub targetOrders: Pubkey,
}

pub fn read_u8(data: &[u8], offset: &mut usize) -> u8 {
    let value = data[*offset];
    *offset += 1;
    value
}

pub fn read_u16(data: &[u8], offset: &mut usize) -> u16 {
    let mut bytes = [0u8; 2];
    bytes.copy_from_slice(&data[*offset..*offset+2]);
    *offset += 2;
    u16::from_le_bytes(bytes)
}

pub fn read_u64(data: &[u8], offset: &mut usize) -> u64 {
    let bytes = &data[*offset..*offset+8];
    *offset += 8;
    u64::from_le_bytes(bytes.try_into().unwrap())
}

#[allow(dead_code)]
pub fn read_u128(data: &[u8], offset: &mut usize) -> u128 {
    let bytes = &data[*offset..*offset+16];
    *offset += 16;
    u128::from_le_bytes(bytes.try_into().unwrap())
}

pub fn read_pubkey(data: &[u8], offset: &mut usize) -> Pubkey {
    let mut key = [0u8; 32];
    key.copy_from_slice(&data[*offset..*offset+32]);
    *offset += 32;
    Pubkey::new_from_array(key)
}
