use log;
use solana_program::pubkey::Pubkey;

#[derive(Debug)]
pub struct PumpLayout {
    pub discriminator: u64,  // 8 bytes
    pub pool_bump: u8,       // 1 byte
    pub index: u16,          // 2 bytes
    pub creator: Pubkey,     // 32 bytes
    pub base_mint: Pubkey,   // 32 bytes
    pub quote_mint: Pubkey,  // 32 bytes
    pub lp_mint: Pubkey,     // 32 bytes
    pub base_vault: Pubkey,  // 32 bytes
    pub quote_vault: Pubkey, // 32 bytes
}

impl PumpLayout {
    pub const LEN: usize = 8 + 1 + 2 + 6 * 32; // 计算总长度

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