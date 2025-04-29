use log;
use solana_program::pubkey::Pubkey;
use crate::common::binary_reader::{read_pubkey, read_u64, read_u8};



pub const RAYDIUM_CP_POOL_SIZE: usize = 637;


#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RaydiumCpLayout {
    pub discriminator: u64,       // Layout discriminator
    pub ammConfig: Pubkey,        // AMM Configuration
    pub poolCreator: Pubkey,      // Pool creator address
    pub token0Vault: Pubkey,      // Token0 vault address (previously vaultA)
    pub token1Vault: Pubkey,      // Token1 vault address (previously vaultB)
    pub lpMint: Pubkey,          // LP token mint address
    pub token0Mint: Pubkey,       // Token0 mint address (previously mintA)
    pub token1Mint: Pubkey,       // Token1 mint address (previously mintB)
    pub token0Program: Pubkey,    // Token program for token0
    pub token1Program: Pubkey,    // Token program for token1
    pub observationKey: Pubkey,   // Observation key
    pub authBump: u8,            // Authority bump
    pub status: u8,              // Pool status
    pub lpMintDecimals: u8,      // LP token decimals
    pub mint0Decimals: u8,       // Token0 decimals
    pub mint1Decimals: u8,       // Token1 decimals
    pub lpSupply: u64,           // LP token supply
    pub protocolFeesToken0: u64,  // Protocol fees for token0
    pub protocolFeesToken1: u64,  // Protocol fees for token1
    pub fundFeesToken0: u64,      // Fund fees for token0
    pub fundFeesToken1: u64,      // Fund fees for token1
    pub openTime: u64,           // Pool open time
}

impl RaydiumCpLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() < RAYDIUM_CP_POOL_SIZE {
            log::error!("数据长度不足，无法解析 RaydiumCpLayout");
            return None;
        }

        let mut offset = 0;
        
        Some(Self {
            discriminator: read_u64(data, &mut offset),
            ammConfig: read_pubkey(data, &mut offset),
            poolCreator: read_pubkey(data, &mut offset),
            token0Vault: read_pubkey(data, &mut offset),      // 改自 vaultA
            token1Vault: read_pubkey(data, &mut offset),      // 改自 vaultB
            lpMint: read_pubkey(data, &mut offset),
            token0Mint: read_pubkey(data, &mut offset),       // 改自 mintA
            token1Mint: read_pubkey(data, &mut offset),       // 改自 mintB
            token0Program: read_pubkey(data, &mut offset),
            token1Program: read_pubkey(data, &mut offset),
            observationKey: read_pubkey(data, &mut offset),
            authBump: read_u8(data, &mut offset),
            status: read_u8(data, &mut offset),
            lpMintDecimals: read_u8(data, &mut offset),
            mint0Decimals: read_u8(data, &mut offset),
            mint1Decimals: read_u8(data, &mut offset),
            lpSupply: read_u64(data, &mut offset),
            protocolFeesToken0: read_u64(data, &mut offset),  // 改自 protocolFeesMintA
            protocolFeesToken1: read_u64(data, &mut offset),  // 改自 protocolFeesMintB
            fundFeesToken0: read_u64(data, &mut offset),      // 改自 fundFeesMintA
            fundFeesToken1: read_u64(data, &mut offset),      // 改自 fundFeesMintB
            openTime: read_u64(data, &mut offset),            // 新增字段
        })
    }
}

pub fn print_raydium_cpmm_layout(ammkey: String, cp_data: &RaydiumCpLayout) {
    log::info!("\n==================== Raydium CPMM 数据 ====================");
    log::info!("AMM Address: (https://solscan.io/account/{}#anchorData)", ammkey);
    log::info!("AMM Config: {}", cp_data.ammConfig);
    log::info!("Pool Creator: {}", cp_data.poolCreator);
    log::info!("Token0 Vault: {}", cp_data.token0Vault);
    log::info!("Token1 Vault: {}", cp_data.token1Vault);
    log::info!("LP Mint: {}", cp_data.lpMint);
    log::info!("Token0 Mint: {}", cp_data.token0Mint);
    log::info!("Token1 Mint: {}", cp_data.token1Mint);
    log::info!("Token0 Program: {}", cp_data.token0Program);
    log::info!("Token1 Program: {}", cp_data.token1Program);
    log::info!("Observation Key: {}", cp_data.observationKey);
    log::info!("Auth Bump: {}", cp_data.authBump);
    log::info!("Status: {}", cp_data.status);
    log::info!("LP Mint Decimals: {}", cp_data.lpMintDecimals);
    log::info!("Token0 Decimals: {}", cp_data.mint0Decimals);
    log::info!("Token1 Decimals: {}", cp_data.mint1Decimals);
    log::info!("LP Supply: {}", cp_data.lpSupply);
    log::info!("Protocol Fees Token0: {}", cp_data.protocolFeesToken0);
    log::info!("Protocol Fees Token1: {}", cp_data.protocolFeesToken1);
    log::info!("Fund Fees Token0: {}", cp_data.fundFeesToken0);
    log::info!("Fund Fees Token1: {}", cp_data.fundFeesToken1);
    log::info!("Open Time: {}", cp_data.openTime);
    log::info!("======================================================\n");
}
