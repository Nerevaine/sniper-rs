### GRPC数据订阅

#### 订阅PumpAmm
```Rust
let pubkey = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string();
accounts.insert(
        "client".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![program],
            filters: vec![,
            nonempty_txn_signature: None,
        },
    );
```
#### 解读数据
> 数据可能有字段
* pubkey , 实际是 PumpAmm 的 AmmId（池子ID）
* owner, 实际是 pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
* data， https://solscan.io/account/CKASURHcRnx6U873hJ4e9UmCjsUR3LwH2yvKod6NsAfy#anchorData
* slot, 区块ID

##### 可能得代码（用AI生成的）
> 获得了data 数据后 ，直接用PumpLayout解析
```Rust
use borsh::BorshDeserialize;
use std::io::Cursor;

#[derive(BorshDeserialize, Debug)]
pub struct PumpLayout {
    pub discriminator: u64,  // u64 类型的标识符
    pub pool_bump: u8,       // u8 类型的 bump
    pub index: u16,          // u16 类型的索引
    pub creator: [u8; 32],   // PublicKey 类型 (32 字节)
    pub base_mint: [u8; 32], // PublicKey 类型 (32 字节)
    pub quote_mint: [u8; 32],// PublicKey 类型 (32 字节)
    pub lp_mint: [u8; 32],   // PublicKey 类型 (32 字节)
    pub base_vault: [u8; 32],// PublicKey 类型 (32 字节)
    pub quote_vault: [u8; 32]// PublicKey 类型 (32 字节)
}

fn main() {
    // 假设 `buffer` 是一个二进制数据的字节数组
    let buffer: Vec<u8> = vec![
        /* 这里填入对应的二进制数据 */
    ];

    // 使用 borsh 解码
    let result = PumpLayout::try_from_slice(&buffer);

    match result {
        Ok(info) => {
            println!("Decoded PumpLayout: {:?}", info);
        }
        Err(e) => {
            eprintln!("Failed to decode PumpLayout: {}", e);
        }
    }
}
```
##### 其他事项

* 一般的git项目中要增加.gitignore
* rust项目中，通常target文件夹需要忽略