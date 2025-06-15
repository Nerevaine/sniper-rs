use log;
use solana_program::pubkey::Pubkey;
use crate::common::binary_reader::{read_pubkey, read_u64, read_u8, read_u16, read_u128};

// 账户数据大小常量
pub const RAYDIUM_CLMM_POOL_SIZE: usize = 1544;

#[derive(Debug)]
pub struct RaydiumClmmLayout {
    pub bump: [u8; 1],
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub token_mint0: Pubkey,
    pub token_mint1: Pubkey,
    pub token_vault0: Pubkey,
    pub token_vault1: Pubkey,
    pub observation_key: Pubkey,
    pub mint_decimals0: u8,
    pub mint_decimals1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub status: u8,
    pub protocol_fees_token0: u64,
    pub protocol_fees_token1: u64,
}

impl RaydiumClmmLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() != RAYDIUM_CLMM_POOL_SIZE {
            log::error!("数据长度不匹配: 期望 {}, 实际 {}", RAYDIUM_CLMM_POOL_SIZE, data.len());
            return None;
        }

        let mut offset = 8; // 跳过discriminator
        
        // 1. 读取基础字段
        let mut bump = [0u8; 1];
        bump[0] = read_u8(data, &mut offset);
        
        // 2. 读取所有 Pubkey 字段
        let amm_config = read_pubkey(data, &mut offset);
        let owner = read_pubkey(data, &mut offset);
        let token_mint0 = read_pubkey(data, &mut offset);
        let token_mint1 = read_pubkey(data, &mut offset);
        let token_vault0 = read_pubkey(data, &mut offset);
        let token_vault1 = read_pubkey(data, &mut offset);
        let observation_key = read_pubkey(data, &mut offset);

        // 3. 读取小数位和tick间距
        let mint_decimals0 = read_u8(data, &mut offset);
        let mint_decimals1 = read_u8(data, &mut offset);
        let tick_spacing = read_u16(data, &mut offset);

        // 4. 读取流动性和价格
        let liquidity = read_u128(data, &mut offset);
        let sqrt_price_x64 = read_u128(data, &mut offset);

        // 5. 读取当前tick
        let tick_current = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        // 6. 跳过padding3和padding4 (2个u16)
        offset += 4;

        // 7. 跳过fee_growth_global字段 (2个u128)
        offset += 32;

        // 8. 读取protocol fees
        let protocol_fees_token0 = read_u64(data, &mut offset);
        let protocol_fees_token1 = read_u64(data, &mut offset);

        // 9. 跳过swap amounts (4个u128)
        offset += 64;

        // 10. 读取status
        let status = read_u8(data, &mut offset);

        // 返回结构体实例
        Some(Self {
            bump,
            amm_config,
            owner,
            token_mint0,
            token_mint1,
            token_vault0,
            token_vault1,
            observation_key,
            mint_decimals0,
            mint_decimals1,
            tick_spacing,
            liquidity,
            sqrt_price_x64,
            tick_current,
            status,
            protocol_fees_token0,
            protocol_fees_token1,
        })
    }
}

pub fn print_raydium_clmm_layout(account_key: String, data: &RaydiumClmmLayout) {
    log::info!("\n==================== Raydium CLMM 数据 ====================");
    log::info!("Pool Address: (https://solscan.io/account/{}#anchorData)", account_key);
    log::info!("AMM Config: {}", data.amm_config);
    log::info!("Owner: {}", data.owner);
    log::info!("Token0 Mint: {}", data.token_mint0);
    log::info!("Token1 Mint: {}", data.token_mint1);
    log::info!("Token0 Vault: {}", data.token_vault0);
    log::info!("Token1 Vault: {}", data.token_vault1);
    log::info!("Observation Key: {}", data.observation_key);
    log::info!("Token0 Decimals: {}", data.mint_decimals0);
    log::info!("Token1 Decimals: {}", data.mint_decimals1);
    log::info!("Tick Spacing: {}", data.tick_spacing);
    log::info!("Liquidity: {}", data.liquidity);
    log::info!("Sqrt Price X64: {}", data.sqrt_price_x64);
    log::info!("Current Tick: {}", data.tick_current);
    log::info!("Status: {}", data.status);
    log::info!("Protocol Fees Token0: {}", data.protocol_fees_token0);
    log::info!("Protocol Fees Token1: {}", data.protocol_fees_token1);

    log::info!("======================================================\n");
}