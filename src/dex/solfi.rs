use log;
use solana_program::pubkey::Pubkey;
use crate::common::binary_reader::{read_pubkey, read_u64, read_u8, read_u16, read_u128};

// 账户数据大小常量
pub const SOLFI_POOL_SIZE: usize = 904;

#[derive(Debug)]
pub struct SolFiLayout {
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

impl SolFiLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() != SOLFI_POOL_SIZE {
            log::error!("数据长度不匹配: 期望 {}, 实际 {}", SOLFI_POOL_SIZE, data.len());
            return None;
        }

        let mut offset = 8; // 跳过discriminator
        
        offset += 1; // 跳过bump字段
        
        let amm_config = read_pubkey(data, &mut offset);
        let owner = read_pubkey(data, &mut offset);
        let token_mint0 = read_pubkey(data, &mut offset);
        let token_mint1 = read_pubkey(data, &mut offset);
        let token_vault0 = read_pubkey(data, &mut offset);
        let token_vault1 = read_pubkey(data, &mut offset);
        let observation_key = read_pubkey(data, &mut offset);

        let mint_decimals0 = read_u8(data, &mut offset);
        let mint_decimals1 = read_u8(data, &mut offset);
        let tick_spacing = read_u16(data, &mut offset);

        let liquidity = read_u128(data, &mut offset);
        let sqrt_price_x64 = read_u128(data, &mut offset);

        let tick_current = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        offset += 4; // 跳过padding

        offset += 32; // 跳过fee_growth_global字段

        let protocol_fees_token0 = read_u64(data, &mut offset);
        let protocol_fees_token1 = read_u64(data, &mut offset);

        offset += 64; // 跳过swap amounts

        let status = read_u8(data, &mut offset);

        Some(Self {
           
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

pub fn print_solfi_layout(account_key: String, data: &SolFiLayout) {
    log::info!("\n==================== SolFi 数据 ====================");
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