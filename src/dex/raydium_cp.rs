use log;
use solana_program::pubkey::Pubkey;
use crate::common::layout::{read_pubkey, read_u64, read_u8};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RaydiumCpLayout {
    pub discriminator: u64,       // Layout discriminator
    pub configId: Pubkey,        // Configuration ID
    pub poolCreator: Pubkey,     // Pool creator address
    pub vaultA: Pubkey,          // Vault A address
    pub vaultB: Pubkey,          // Vault B address
    pub lpMint: Pubkey,          // LP token mint address
    pub mintA: Pubkey,           // Token A mint address
    pub mintB: Pubkey,           // Token B mint address
    pub token0Program: Pubkey,   // Token program for token0
    pub token1Program: Pubkey,   // Token program for token1
    pub observationKey: Pubkey,  // Observation key
    pub authBump: u8,            // Authority bump
    pub status: u8,              // Pool status
    pub lpMintDecimals: u8,      // LP token decimals
    pub mint0Decimals: u8,       // Token0 decimals
    pub mint1Decimals: u8,       // Token1 decimals
    pub lpSupply: u64,           // LP token supply
    pub protocolFeesMintA: u64,  // Protocol fees for token A
    pub protocolFeesMintB: u64,  // Protocol fees for token B
    pub fundFeesMintA: u64,      // Fund fees for token A
    pub fundFeesMintB: u64,      // Fund fees for token B
}

impl RaydiumCpLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() < 336 {
            log::error!("数据长度不足，无法解析 RaydiumCpLayout");
            return None;
        }

        let mut offset = 8;
        
        Some(Self {
            discriminator: read_u64(data, &mut offset),
            configId: read_pubkey(data, &mut offset),
            poolCreator: read_pubkey(data, &mut offset),
            vaultA: read_pubkey(data, &mut offset),
            vaultB: read_pubkey(data, &mut offset),
            lpMint: read_pubkey(data, &mut offset),
            mintA: read_pubkey(data, &mut offset),
            mintB: read_pubkey(data, &mut offset),
            token0Program: read_pubkey(data, &mut offset),
            token1Program: read_pubkey(data, &mut offset),
            observationKey: read_pubkey(data, &mut offset),
            authBump: read_u8(data, &mut offset),
            status: read_u8(data, &mut offset),
            lpMintDecimals: read_u8(data, &mut offset),
            mint0Decimals: read_u8(data, &mut offset),
            mint1Decimals: read_u8(data, &mut offset),
            lpSupply: read_u64(data, &mut offset),
            protocolFeesMintA: read_u64(data, &mut offset),
            protocolFeesMintB: read_u64(data, &mut offset),
            fundFeesMintA: read_u64(data, &mut offset),
            fundFeesMintB: read_u64(data, &mut offset),
        })
    }
}

pub fn print_raydium_cp_layout(ammkey: String, cp_data: &RaydiumCpLayout) {
    log::info!("\n==================== Raydium CP 数据 ====================");
    log::info!("AMM Address: {}", ammkey);
    log::info!("Discriminator: {}", cp_data.discriminator);
    log::info!("Config ID: {}", cp_data.configId);
    log::info!("Pool Creator: {}", cp_data.poolCreator);
    log::info!("Vault A: {}", cp_data.vaultA);
    log::info!("Vault B: {}", cp_data.vaultB);
    log::info!("LP Mint: {}", cp_data.lpMint);
    log::info!("Mint A: {}", cp_data.mintA);
    log::info!("Mint B: {}", cp_data.mintB);
    log::info!("Token0 Program: {}", cp_data.token0Program);
    log::info!("Token1 Program: {}", cp_data.token1Program); 
    log::info!("Observation Key: {}", cp_data.observationKey);
    log::info!("Auth Bump: {}", cp_data.authBump);
    log::info!("Status: {}", cp_data.status);
    log::info!("LP Mint Decimals: {}", cp_data.lpMintDecimals);
    log::info!("Mint0 Decimals: {}", cp_data.mint0Decimals);
    log::info!("Mint1 Decimals: {}", cp_data.mint1Decimals);
    log::info!("LP Supply: {}", cp_data.lpSupply);
    log::info!("Protocol Fees Mint A: {}", cp_data.protocolFeesMintA);
    log::info!("Protocol Fees Mint B: {}", cp_data.protocolFeesMintB);
    log::info!("Fund Fees Mint A: {}", cp_data.fundFeesMintA);
    log::info!("Fund Fees Mint B: {}", cp_data.fundFeesMintB);
    log::info!("======================================================\n");
}