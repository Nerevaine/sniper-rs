### GRPC数据订阅

#### 订阅PumpAmm
```Rust
let pubkey = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string();
accounts.insert(
        "client".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![program],
            owner: vec![],
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



#### 其他事项

* 一般的git项目中要增加.gitignore
* rust项目中，通常target文件夹需要忽略