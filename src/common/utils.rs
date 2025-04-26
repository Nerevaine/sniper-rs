use std::str::FromStr;
use solana_program::pubkey::Pubkey;

// 从字符串创建 Pubkey
pub fn pubkey_from_string(key_str: &str) -> Option<Pubkey> {
    Pubkey::from_str(key_str).ok()
}

// 将 Pubkey 转换为字符串
pub fn pubkey_to_string(pubkey: &Pubkey) -> String {
    pubkey.to_string()
}

// 检查数据长度是否足够
pub fn check_data_len(data: &[u8], required_len: usize) -> bool {
    data.len() >= required_len
}