use log;
use solana_program::pubkey::Pubkey;
use crate::common::binary_reader::{read_pubkey, read_u64, read_u8, read_u16, read_u32, read_i32, read_i64, read_u128};

// 账户数据大小常量
pub const METEORA_DLMM_POOL_SIZE: usize = 904;
pub const METEORA_DLMM_ORACLE_SIZE: usize = 3232; // Oracle 账户大小常量
pub const METEORA_DLMM_BIN_ARRAY_SIZE: usize = 10136; // 添加 BinArray 大小常量


#[derive(Debug)]
pub struct StaticParameters {
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub min_bin_id: i32,
    pub max_bin_id: i32,
    pub protocol_share: u16,
    pub base_fee_power_factor: u8,
    pub padding: [u8; 5], // 新增
}

#[derive(Debug)]
pub struct VariableParameters {
    pub volatility_accumulator: u32,
    pub volatility_reference: u32,
    pub index_reference: i32,
    pub last_update_timestamp: i64,
}

#[derive(Debug)]
pub struct ProtocolFee {
    pub amount_x: u64,
    pub amount_y: u64,
}

#[derive(Debug)]
pub struct RewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub funder: Pubkey,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u64,
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}

#[derive(Debug)]
pub struct MeteoraLayout {
    pub parameters: StaticParameters,
    pub v_parameters: VariableParameters,
    pub bump_seed: [u8; 1],
    pub bin_step_seed: [u8; 2],
    pub pair_type: u8,
    pub active_id: i32,
    pub bin_step: u16,
    pub status: u8,
    pub require_base_factor_seed: u8,
    pub base_factor_seed: [u8; 2],
    pub activation_type: u8,
    pub creator_pool_on_off_control: u8,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub protocol_fee: ProtocolFee,
    pub reward_infos: [RewardInfo; 2],
    pub bin_array_bitmap: [u64; 16],
    pub last_updated_at: i64,
}

impl MeteoraLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() != METEORA_DLMM_POOL_SIZE {
            log::error!("数据长度不匹配: 期望 {}, 实际 {}", METEORA_DLMM_POOL_SIZE, data.len());
            return None;
        }

        let mut offset = 8; // 跳过 8 字节 discriminator

        // 读取 StaticParameters
        let parameters = StaticParameters {
            base_factor: read_u16(data, &mut offset),
            filter_period: read_u16(data, &mut offset),
            decay_period: read_u16(data, &mut offset),
            reduction_factor: read_u16(data, &mut offset),
            variable_fee_control: read_u32(data, &mut offset),
            max_volatility_accumulator: read_u32(data, &mut offset),
            min_bin_id: read_i32(data, &mut offset),
            max_bin_id: read_i32(data, &mut offset),
            protocol_share: read_u16(data, &mut offset),
            base_fee_power_factor: read_u8(data, &mut offset),
            padding: {
                let mut arr = [0u8; 5];
                arr.copy_from_slice(&data[offset..offset + 5]);
                offset += 5;
                arr
            },
        };

        // offset += 1; // 移除这行：跳过padding (调整为 1 字节对齐) - 这里不需要跳过

        // 读取 VariableParameters
        let v_parameters = VariableParameters {
            volatility_accumulator: read_u32(data, &mut offset),
            volatility_reference: read_u32(data, &mut offset),
            index_reference: read_i32(data, &mut offset),
            // 读取 padding
            // offset += 4; // 跳过 padding [u8; 4]
            last_update_timestamp: {
                offset += 4; // 跳过 padding
                read_i64(data, &mut offset)
            },
        };
        offset += 8; // 跳过 padding1 [u8; 8]

        // 读取基本字段
        let mut bump_seed = [0u8; 1];
        bump_seed[0] = read_u8(data, &mut offset);

        let mut bin_step_seed = [0u8; 2];
        bin_step_seed.copy_from_slice(&data[offset..offset + 2]);
        offset += 2;

        let pair_type = read_u8(data, &mut offset);
        let active_id = read_i32(data, &mut offset);
        let bin_step = read_u16(data, &mut offset);
        let status = read_u8(data, &mut offset);
        let require_base_factor_seed = read_u8(data, &mut offset);

        let mut base_factor_seed = [0u8; 2];
        base_factor_seed.copy_from_slice(&data[offset..offset + 2]);
        offset += 2;

        let activation_type = read_u8(data, &mut offset);
        let creator_pool_on_off_control = read_u8(data, &mut offset);

        // 读取 Pubkey 字段
        let token_x_mint = read_pubkey(data, &mut offset);
        let token_y_mint = read_pubkey(data, &mut offset);
        let reserve_x = read_pubkey(data, &mut offset);
        let reserve_y = read_pubkey(data, &mut offset);

        // 读取 ProtocolFee
        let protocol_fee = ProtocolFee {
            amount_x: read_u64(data, &mut offset),
            amount_y: read_u64(data, &mut offset),
        };

        offset += 32; // 跳过padding1

        // 读取 RewardInfos
        let mut reward_infos: [RewardInfo; 2] = core::array::from_fn(|_| RewardInfo {
            mint: Pubkey::default(),
            vault: Pubkey::default(),
            funder: Pubkey::default(),
            reward_duration: 0,
            reward_duration_end: 0,
            reward_rate: 0,
            last_update_time: 0,
            cumulative_seconds_with_empty_liquidity_reward: 0,
        });

        for reward_info in reward_infos.iter_mut() {
            reward_info.mint = read_pubkey(data, &mut offset);
            reward_info.vault = read_pubkey(data, &mut offset);
            reward_info.funder = read_pubkey(data, &mut offset);
            reward_info.reward_duration = read_u64(data, &mut offset);
            reward_info.reward_duration_end = read_u64(data, &mut offset);
            reward_info.reward_rate = read_u64(data, &mut offset);
            reward_info.last_update_time = read_u64(data, &mut offset);
            reward_info.cumulative_seconds_with_empty_liquidity_reward = read_u64(data, &mut offset);
        }

        // 确保读取完 RewardInfos 后的 offset 正确

        // 读取 binArrayBitmap
        let mut bin_array_bitmap = [0u64; 16];
        for item in bin_array_bitmap.iter_mut() {
            *item = read_u64(data, &mut offset);
        }

        let last_updated_at = read_i64(data, &mut offset);

        Some(Self {
            parameters,
            v_parameters,
            bump_seed,
            bin_step_seed,
            pair_type,
            active_id,
            bin_step,
            status,
            require_base_factor_seed,
            base_factor_seed,
            activation_type,
            creator_pool_on_off_control,
            token_x_mint,
            token_y_mint,
            reserve_x,
            reserve_y,
            protocol_fee,
            reward_infos,
            bin_array_bitmap,
            last_updated_at,
        })
    }
}

pub fn print_meteora_layout(account_key: String, data: &MeteoraLayout) {
    log::info!("==================== Meteora DLMM LbPair 数据 ====================");
    log::info!("Pool Address: (https://solscan.io/account/{}#anchorData)", account_key);
    
    // 打印 StaticParameters
    log::info!("Static Parameters:");
    log::info!("  Base Factor: {}", data.parameters.base_factor);
    log::info!("  Filter Period: {}", data.parameters.filter_period);
    log::info!("  Decay Period: {}", data.parameters.decay_period);
    log::info!("  Reduction Factor: {}", data.parameters.reduction_factor);
    log::info!("  Variable Fee Control: {}", data.parameters.variable_fee_control);
    log::info!("  Max Volatility Accumulator: {}", data.parameters.max_volatility_accumulator);
    log::info!("  Min Bin ID: {}", data.parameters.min_bin_id);
    log::info!("  Max Bin ID: {}", data.parameters.max_bin_id);
    log::info!("  Protocol Share: {}", data.parameters.protocol_share);
    
    // 打印 VariableParameters
    log::info!("\nVariable Parameters:");
    log::info!("  Volatility Accumulator: {}", data.v_parameters.volatility_accumulator);
    log::info!("  Volatility Reference: {}", data.v_parameters.volatility_reference);
    log::info!("  Index Reference: {}", data.v_parameters.index_reference);
    log::info!("  Last Update Timestamp: {}", data.v_parameters.last_update_timestamp);
    
    // 打印基本信息
    log::info!("\nPool Info:");
    log::info!("  Active ID: {}", data.active_id);
    log::info!("  Bin Step: {}", data.bin_step);
    log::info!("  Status: {}", data.status);
    log::info!("  Pair Type: {}", data.pair_type);
    
    // 打印Token信息
    log::info!("\nToken Info:");
    log::info!("  Token X Mint: {}", data.token_x_mint);
    log::info!("  Token Y Mint: {}", data.token_y_mint);
    log::info!("  Reserve X: {}", data.reserve_x);
    log::info!("  Reserve Y: {}", data.reserve_y);
    
    // 打印协议费用
    log::info!("\nProtocol Fees:");
    log::info!("  Amount X: {}", data.protocol_fee.amount_x);
    log::info!("  Amount Y: {}", data.protocol_fee.amount_y);
    
    // 打印奖励信息
    log::info!("\nReward Info:");
    for (i, reward) in data.reward_infos.iter().enumerate() {
        log::info!("  Reward {}:", i + 1);
        log::info!("    Mint: {}", reward.mint);
        log::info!("    Vault: {}", reward.vault);
        log::info!("    Funder: {}", reward.funder);
        log::info!("    Duration: {}", reward.reward_duration);
        log::info!("    Duration End: {}", reward.reward_duration_end);
        log::info!("    Rate: {}", reward.reward_rate);
        log::info!("    Last Update Time: {}", reward.last_update_time);
        log::info!("    Cumulative Empty Liquidity Seconds: {}", 
            reward.cumulative_seconds_with_empty_liquidity_reward);
    }
    
    // 删除打印 Oracle 的代码
    // log::info!("\nOracle: {}", data.oracle);
    log::info!("Last Updated At: {}", data.last_updated_at);
    log::info!("======================================================\n");
}

#[derive(Debug)]
pub struct OracleLayout {
    pub discriminator: u64,
    pub idx: u64,
    pub active_size: u64,
    pub length: u64,
    pub bins: Vec<BinData>,
}

#[derive(Debug)]
pub struct BinData {
    pub amount_x: u64,
    pub amount_y: u64,
    pub price: u64,
    pub liquidity: u64,
}

impl OracleLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() != METEORA_DLMM_ORACLE_SIZE {
            log::error!("数据长度不匹配: 期望 {}, 实际 {}", METEORA_DLMM_ORACLE_SIZE, data.len());
            return None;
        }

        let mut offset = 8; // 跳过 discriminator

        let discriminator = read_u64(data, &mut offset);
        let idx = read_u64(data, &mut offset);
        let active_size = read_u64(data, &mut offset);
        let length = read_u64(data, &mut offset);

        let mut bins = Vec::new();
        while offset + 32 <= data.len() {
            let amount_x = read_u64(data, &mut offset);
            let amount_y = read_u64(data, &mut offset);
            let price = read_u64(data, &mut offset);
            let liquidity = read_u64(data, &mut offset);
            
            bins.push(BinData {
                amount_x,
                amount_y,
                price,
                liquidity,
            });
        }

        Some(Self {
            discriminator,
            idx,
            active_size,
            length,
            bins,
        })
    }
}

pub fn print_oracle_layout(account_key: String, data: &OracleLayout) {
    log::info!("==================== Meteora DLMM Oracle 数据 ====================");
    log::info!("Oracle Address: (https://solscan.io/account/{}#anchorData)", account_key);
    
    log::info!("Discriminator: {}", data.discriminator);// discriminator 用于区分不同类型的账户
    log::info!("idx: {}", data.idx);
    log::info!("Active Size: {}", data.active_size);
    log::info!("Length: {}", data.length);
    
    // 打印前 5 个和后 5 个 bin (如果有足够多的 bin)
    let bin_count = data.bins.len();
    log::info!("\n总共 {} 个 Bin，显示部分数据:", bin_count);
    
    // 打印前 5 个 bin
    let front_limit = std::cmp::min(5, bin_count);
    for i in 0..front_limit {
        let bin = &data.bins[i];
        log::info!("Bin[{}]: X={}, Y={}, Price={}, Liquidity={}", 
            i, bin.amount_x, bin.amount_y, bin.price, bin.liquidity);
    }
    
    // 如果 bin 数量超过 10 个，打印省略号
    if bin_count > 10 {
        log::info!("...");
    }
    
    // 打印后 5 个 bin
    if bin_count > 5 {
        let back_start = std::cmp::max(front_limit, bin_count - 5);
        for i in back_start..bin_count {
            let bin = &data.bins[i];
            log::info!("Bin[{}]: X={}, Y={}, Price={}, Liquidity={}", 
                i, bin.amount_x, bin.amount_y, bin.price, bin.liquidity);
        }
    }
    
    log::info!("==============================================================\n");
}

#[derive(Debug)]
pub struct Bin {
    pub amount_x: u64,
    pub amount_y: u64,
    pub price: u64,
    pub liquidity_supply: u128,
    pub reward_per_token_stored: [u64; 2],
    pub fee_amount_x_per_token_stored: u64,
    pub fee_amount_y_per_token_stored: u64,
    pub amount_x_in: u64,
    pub amount_y_in: u64,
}

#[derive(Debug)]
pub struct BinArrayLayout {
    pub index: i64,
    pub version: u8,
    pub padding: [u8; 7],
    pub lb_pair: Pubkey,
    pub bins: Vec<Bin>,
}

impl BinArrayLayout {
    pub fn try_from_slice_manual(data: &[u8]) -> Option<Self> {
        if data.len() != METEORA_DLMM_BIN_ARRAY_SIZE {
            log::error!("数据长度不匹配: 期望 {}, 实际 {}", METEORA_DLMM_BIN_ARRAY_SIZE, data.len());
            return None;
        }

        let mut offset = 8; // 跳过 discriminator

        let index = read_i64(data, &mut offset);
        let version = read_u8(data, &mut offset);
        
        let mut padding = [0u8; 7];
        padding.copy_from_slice(&data[offset..offset + 7]);
        offset += 7;

        let lb_pair = read_pubkey(data, &mut offset);

        let mut bins = Vec::with_capacity(70);
        for _ in 0..70 {
            let bin = Bin {
                amount_x: read_u64(data, &mut offset),
                amount_y: read_u64(data, &mut offset),
                price: read_u64(data, &mut offset),
                liquidity_supply: read_u128(data, &mut offset),
                reward_per_token_stored: [read_u64(data, &mut offset), read_u64(data, &mut offset)],
                fee_amount_x_per_token_stored: read_u64(data, &mut offset),
                fee_amount_y_per_token_stored: read_u64(data, &mut offset),
                amount_x_in: read_u64(data, &mut offset),
                amount_y_in: read_u64(data, &mut offset),
            };
            bins.push(bin);
        }

        Some(Self {
            index,
            version,
            padding,
            lb_pair,
            bins,
        })
    }
}

pub fn print_bin_array_layout(account_key: String, data: &BinArrayLayout) {
    log::info!("==================== Meteora DLMM BinArray 数据 ====================");
    log::info!("BinArray Address: (https://solscan.io/account/{}#anchorData)", account_key);
    log::info!("Index: {}", data.index);
    log::info!("Version: {}", data.version);
    log::info!("LB Pair: {}", data.lb_pair);
    
    log::info!("\nBins 数据 (显示前5个):");
    for (i, bin) in data.bins.iter().enumerate().take(5) {
        log::info!("Bin[{}]:", i);
        log::info!("  Amount X: {}", bin.amount_x);
        log::info!("  Amount Y: {}", bin.amount_y);
        log::info!("  Price: {}", bin.price);
        log::info!("  Liquidity Supply: {}", bin.liquidity_supply);
        log::info!("  Reward Per Token Stored: {:?}", bin.reward_per_token_stored);
        log::info!("  Fee Amount X Per Token: {}", bin.fee_amount_x_per_token_stored);
        log::info!("  Fee Amount Y Per Token: {}", bin.fee_amount_y_per_token_stored);
        log::info!("  Amount X In: {}", bin.amount_x_in);
        log::info!("  Amount Y In: {}", bin.amount_y_in);
    }
    
    if data.bins.len() > 5 {
        log::info!("\n... 还有 {} 个 Bin 未显示", data.bins.len() - 5);
    }
    
    log::info!("==============================================================\n");
}