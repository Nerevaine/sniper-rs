use log;

use crate::dex::raydium_lp_v4::{
    print_raydium_lp_v4_layout, process_market, RaydiumLpV4Layout, SerumMarketLayout,
    RAYDIUM_LP_V4_ACCOUNT_SIZE, SERUM_MARKET_ACCOUNT_SIZE,
};

// FILTERS

/// Process raydium type account data, print information only
/// - ammkey: Account public key string
/// - buffer: Account Raw Data Bytes
pub fn raydium_lp_v4(account_key: String, buffer: Vec<u8>) {
    if buffer.len() == RAYDIUM_LP_V4_ACCOUNT_SIZE {
        match RaydiumLpV4Layout::try_from_slice_manual(buffer.as_slice()) {
            Some(raydium_data) => print_raydium_lp_v4_layout(account_key, &raydium_data),
            None => log::error!(
                "Unable to parse raydium data: buffer length {}",
                buffer.len()
            ),
        }
    } else if buffer.len() == SERUM_MARKET_ACCOUNT_SIZE {
        // Processing of serum market account data
        match SerumMarketLayout::slice_market(buffer.as_slice()) {
            Some(market_data) => process_market(account_key, &market_data),
            None => log::error!(
                "Unable to parse market data: buffer length {}",
                buffer.len()
            ),
        }
    } else {
        log::error!(
            "Unknown data length: {}, expected {} or {}",
            buffer.len(),
            RAYDIUM_LP_V4_ACCOUNT_SIZE,
            SERUM_MARKET_ACCOUNT_SIZE
        );
    }
}
