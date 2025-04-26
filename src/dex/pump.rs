use log;
use solana_program::pubkey::Pubkey;

/// PumpLayout 结构体定义了流动性池的账户数据布局
#[derive(Debug)]
pub struct PumpLayout {
    pub discriminator: u64,  // 账户类型识别码，用于区分不同类型的账户，占用 8 字节
    pub pool_bump: u8,       // PDA派生时使用的bump种子值，占用 1 字节
    pub index: u16,          // 流动性池的唯一索引号，占用 2 字节
    pub creator: Pubkey,     // 创建者的公钥地址，占用 32 字节
    pub base_mint: Pubkey,   // 基础代币的铸币权地址，占用 32 字节
    pub quote_mint: Pubkey,  // 报价代币的铸币权地址，占用 32 字节
    pub lp_mint: Pubkey,     // LP代币的铸币权地址，占用 32 字节
    pub base_vault: Pubkey,  // 基础代币的金库地址，占用 32 字节
    pub quote_vault: Pubkey, // 报价代币的金库地址，占用 32 字节
}

impl PumpLayout {
    /// 账户数据的总长度：8(discriminator) + 1(bump) + 2(index) + 6*32(pubkeys)
    pub const LEN: usize = 8 + 1 + 2 + 6 * 32;

    /// 从字节切片中手动解析 PumpLayout 结构
    /// @param data - 要解析的字节数据
    /// @returns Option<PumpLayout> - 解析成功返回 Some(PumpLayout)，失败返回 None
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() < Self::LEN {
            log::error!("数据长度不足，无法解析 PumpLayout");
            return None;
        }

        let mut offset = 0;
        
        let discriminator = {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&data[offset..offset+8]);
            offset += 8;
            u64::from_le_bytes(bytes)
        };
        
        let pool_bump = data[offset];
        offset += 1;
        
        let index = {
            let mut bytes = [0u8; 2];
            bytes.copy_from_slice(&data[offset..offset+2]);
            offset += 2;
            u16::from_le_bytes(bytes)
        };
        
        let read_pubkey = |data: &[u8], offset: &mut usize| {
            let mut key = [0u8; 32];
            key.copy_from_slice(&data[*offset..*offset+32]);
            *offset += 32;
            Pubkey::new_from_array(key)
        };
        
        Some(Self {
            discriminator,
            pool_bump,
            index,
            creator: read_pubkey(data, &mut offset),
            base_mint: read_pubkey(data, &mut offset),
            quote_mint: read_pubkey(data, &mut offset),
            lp_mint: read_pubkey(data, &mut offset),
            base_vault: read_pubkey(data, &mut offset),
            quote_vault: read_pubkey(data, &mut offset),
        })
    }
}

/// 打印 PumpLayout 结构体中的所有字段信息
/// @param pump_data - PumpLayout 结构体引用
pub fn print_pump_layout(pump_data: &PumpLayout) {
    log::info!("PumpLayout data:");
    log::info!("  discriminator: {}", pump_data.discriminator);
    log::info!("  pool_bump: {}", pump_data.pool_bump);
    log::info!("  index: {}", pump_data.index);
    log::info!("  creator: {}", pump_data.creator);
    log::info!("  base_mint: {}", pump_data.base_mint);
    log::info!("  quote_mint: {}", pump_data.quote_mint);
    log::info!("  lp_mint: {}", pump_data.lp_mint);
    log::info!("  base_vault: {}", pump_data.base_vault);
    log::info!("  quote_vault: {}", pump_data.quote_vault);
}