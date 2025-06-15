use log;
use solana_program::pubkey::Pubkey;
use crate::common::binary_reader::{read_pubkey, read_u64, read_u8, read_bool};

// 账户数据大小常量
pub const METEORA_POOLS_SIZE: usize = 944;

#[derive(Debug)]
pub struct PoolFees {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub protocol_trade_fee_numerator: u64,
    pub protocol_trade_fee_denominator: u64,
}

#[derive(Debug)]
pub struct Bootstrapping {
    pub activation_point: u64,
    pub whitelisted_vault: Pubkey,
    pub pool_creator: Pubkey,
    pub activation_type: u8,
}

#[derive(Debug)]
pub struct PartnerInfo {
    pub fee_numerator: u64,
    pub partner_authority: Pubkey,
    pub pending_fee_a: u64,
    pub pending_fee_b: u64,
}

#[derive(Debug)]
pub struct Padding {
    pub padding0: [u8; 6],
    pub padding1: [u64; 21],
    pub padding2: [u64; 21],
}

#[derive(Debug)]
pub struct MeteoraPools {
    pub lp_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_bump: u8,
    pub enabled: bool,
    pub protocol_token_a_fee: Pubkey,
    pub protocol_token_b_fee: Pubkey,
    pub fee_last_updated_at: u64,
    pub padding0: [u8; 24],
    pub fees: PoolFees,
    pub pool_type: u8,  // 0 = permissionless
    pub stake: Pubkey,
    pub total_locked_lp: u64,
    pub bootstrapping: Bootstrapping,
    pub partner_info: PartnerInfo,
    pub padding: Padding,
    pub curve_type: u8,  // 0 = constant product
}

impl MeteoraPools {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() != METEORA_POOLS_SIZE {
            log::error!("数据长度不匹配: 期望 {}, 实际 {}", METEORA_POOLS_SIZE, data.len());
            return None;
        }

        let mut offset = 8; // 跳过 discriminator

        let lp_mint = read_pubkey(data, &mut offset);
        let token_a_mint = read_pubkey(data, &mut offset);
        let token_b_mint = read_pubkey(data, &mut offset);
        let a_vault = read_pubkey(data, &mut offset);
        let b_vault = read_pubkey(data, &mut offset);
        let a_vault_lp = read_pubkey(data, &mut offset);
        let b_vault_lp = read_pubkey(data, &mut offset);
        let a_vault_lp_bump = read_u8(data, &mut offset);
        let enabled = read_bool(data, &mut offset);
        let protocol_token_a_fee = read_pubkey(data, &mut offset);
        let protocol_token_b_fee = read_pubkey(data, &mut offset);
        let fee_last_updated_at = read_u64(data, &mut offset);

        let mut padding0 = [0u8; 24];
        padding0.copy_from_slice(&data[offset..offset + 24]);
        offset += 24;

        let fees = PoolFees {
            trade_fee_numerator: read_u64(data, &mut offset),
            trade_fee_denominator: read_u64(data, &mut offset),
            protocol_trade_fee_numerator: read_u64(data, &mut offset),
            protocol_trade_fee_denominator: read_u64(data, &mut offset),
        };

        let pool_type = read_u8(data, &mut offset);
        let stake = read_pubkey(data, &mut offset);
        let total_locked_lp = read_u64(data, &mut offset);

        let bootstrapping = Bootstrapping {
            activation_point: read_u64(data, &mut offset),
            whitelisted_vault: read_pubkey(data, &mut offset),
            pool_creator: read_pubkey(data, &mut offset),
            activation_type: read_u8(data, &mut offset),
        };

        let partner_info = PartnerInfo {
            fee_numerator: read_u64(data, &mut offset),
            partner_authority: read_pubkey(data, &mut offset),
            pending_fee_a: read_u64(data, &mut offset),
            pending_fee_b: read_u64(data, &mut offset),
        };

        let mut padding = Padding {
            padding0: [0u8; 6],
            padding1: [0u64; 21],
            padding2: [0u64; 21],
        };
        padding.padding0.copy_from_slice(&data[offset..offset + 6]);
        offset += 6;

        for i in 0..21 {
            padding.padding1[i] = read_u64(data, &mut offset);
        }
        for i in 0..21 {
            padding.padding2[i] = read_u64(data, &mut offset);
        }

        let curve_type = read_u8(data, &mut offset);

        Some(Self {
            lp_mint,
            token_a_mint,
            token_b_mint,
            a_vault,
            b_vault,
            a_vault_lp,
            b_vault_lp,
            a_vault_lp_bump,
            enabled,
            protocol_token_a_fee,
            protocol_token_b_fee,
            fee_last_updated_at,
            padding0,
            fees,
            pool_type,
            stake,
            total_locked_lp,
            bootstrapping,
            partner_info,
            padding,
            curve_type,
        })
    }
}

pub fn print_meteora_pools_layout(account_key: String, data: &MeteoraPools) {
    log::info!("==================== Meteora Pools 数据 ====================");
    log::info!("Pool Address: (https://solscan.io/account/{}#anchorData)", account_key);
    log::info!("LP Mint: {}", data.lp_mint);
    log::info!("Token A Mint: {}", data.token_a_mint);
    log::info!("Token B Mint: {}", data.token_b_mint);
    log::info!("A Vault: {}", data.a_vault);
    log::info!("B Vault: {}", data.b_vault);
    log::info!("A Vault LP: {}", data.a_vault_lp);
    log::info!("B Vault LP: {}", data.b_vault_lp);
    log::info!("A Vault LP Bump: {}", data.a_vault_lp_bump);
    log::info!("Enabled: {}", data.enabled);
    log::info!("Protocol Token A Fee: {}", data.protocol_token_a_fee);
    log::info!("Protocol Token B Fee: {}", data.protocol_token_b_fee);
    log::info!("Fee Last Updated At: {}", data.fee_last_updated_at);
    
    log::info!("\nFees:");
    log::info!("  Trade Fee: {}/{}", data.fees.trade_fee_numerator, data.fees.trade_fee_denominator);
    log::info!("  Protocol Trade Fee: {}/{}", data.fees.protocol_trade_fee_numerator, data.fees.protocol_trade_fee_denominator);
    
    log::info!("\nPool Type: {}", if data.pool_type == 0 { "Permissionless" } else { "Unknown" });
    log::info!("Stake: {}", data.stake);
    log::info!("Total Locked LP: {}", data.total_locked_lp);
    
    log::info!("\nBootstrapping:");
    log::info!("  Activation Point: {}", data.bootstrapping.activation_point);
    log::info!("  Whitelisted Vault: {}", data.bootstrapping.whitelisted_vault);
    log::info!("  Pool Creator: {}", data.bootstrapping.pool_creator);
    log::info!("  Activation Type: {}", data.bootstrapping.activation_type);
    
    log::info!("\nPartner Info:");
    log::info!("  Fee Numerator: {}", data.partner_info.fee_numerator);
    log::info!("  Partner Authority: {}", data.partner_info.partner_authority);
    log::info!("  Pending Fee A: {}", data.partner_info.pending_fee_a);
    log::info!("  Pending Fee B: {}", data.partner_info.pending_fee_b);
    
    log::info!("\nCurve Type: {}", if data.curve_type == 0 { "Constant Product" } else { "Unknown" });
    log::info!("======================================================\n");
}