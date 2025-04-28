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
        
        // 读取并解析账户数据中的 discriminator 字段
        let discriminator = {
            // 创建一个 8 字节的缓冲区用于存储 discriminator
            let mut bytes = [0u8; 8];
            // 从数据切片中复制前 8 个字节到缓冲区
            bytes.copy_from_slice(&data[offset..offset+8]);
            // 将 offset 向后移动 8 个字节，为读取下一个字段做准备
            offset += 8;
            // 将字节数组转换为小端序的 u64 整数
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

/// 打印 PumpLayout 账户数据的详细信息
/// @param account_key - 账户地址字符串
/// @param pump_data - PumpLayout 结构体引用
pub fn print_pump_layout(account_key: String, pump_data: &PumpLayout) {
    log::info!("\n==================== Pump 数据 ====================");
    log::info!("Pool Address: (https://solscan.io/account/{}#anchorData)", account_key);
    log::info!("Discriminator: {}", pump_data.discriminator);
    log::info!("Pool Bump: {}", pump_data.pool_bump);
    log::info!("Index: {}", pump_data.index);
    log::info!("Creator: {}", pump_data.creator);
    log::info!("Base Mint: {}", pump_data.base_mint);
    log::info!("Quote Mint: {}", pump_data.quote_mint);
    log::info!("LP Mint: {}", pump_data.lp_mint);
    log::info!("Base Vault: {}", pump_data.base_vault);
    log::info!("Quote Vault: {}", pump_data.quote_vault);
    log::info!("================================================\n");
}