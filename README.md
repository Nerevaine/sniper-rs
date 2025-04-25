### GRPC数据订阅

#### 订阅raydiumCp
* 合约 CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C
* 数据一 池子信息
* 举例：https://solscan.io/account/ABP7VLGq1EZbwcPosDReaMLojJkCvZjLaHMkLjv5DNwo#anchorData
```Typescript
export const RAYDIUM_CP_LAYOUT = struct([
    u64('discriminator'),
    publicKey('configId'),
    publicKey('poolCreator'),
    publicKey('vaultA'),
    publicKey('vaultB'),
    publicKey('lpMint'),
    publicKey('mintA'),
    publicKey('mintB'),
    publicKey('token0Program'),
    publicKey('token1Program'),
    publicKey('observationKey'),
    u8('authBump'),
    u8('status'),
    u8('lpMintDecimals'),
    u8('mint0Decimals'),
    u8('mint1Decimals'),
    u64('lpSupply'),
    u64('protocolFeesMintA'),
    u64('protocolFeesMintB'),
    u64('fundFeesMintA'),
    u64('fundFeesMintB'),
]);
```
* 数据二 配置信息
* 举例：https://solscan.io/account/BhH6HphjBKXu2PkUc2aw3xEMdUvK14NXxE5LbNWZNZAA#anchorData


### 已解决问题

#### 1. 项目依赖编译问题

**问题描述：**
项目依赖 `protobuf-src v1.1.0+21.5` 在 Windows 环境下编译失败，具体错误为路径不安全。

**问题回答：**
在ubuntu环境下开发
