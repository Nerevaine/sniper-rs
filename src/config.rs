use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub endpoints: Vec<String>,
    pub program_ids: ProgramIds,
    pub retry_settings: RetrySettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgramIds {
    pub pump: String,
    pub raydium_lp_v4: String,
    // ...
}