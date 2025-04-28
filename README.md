# Rust gRPC 项目

## 功能说明

### 支持的交易所数据解析
目前项目可以正确解析和读取以下交易所的数据：

1. Pump DEX
   - 文件：`src/dex/pump.rs`
   - 支持解析流动性池账户数据

2. Raydium DEX
   - CLMM (Concentrated Liquidity Market Maker)
     - 文件：`src/dex/raydium_clmm.rs`
     - 支持解析集中流动性做市商池数据
   - CPMM (Constant Product Market Maker)
     - 文件：`src/dex/raydium_cpmm.rs`
     - 支持解析恒定乘积做市商池数据
   - LP V4 (Liquidity Pool Version 4)
     - 文件：`src/dex/raydium_lp_v4.rs`
     - 支持解析 V4 版本流动性池数据

### 开发中的功能

1. SolFi DEX
   - 状态：开发进行中
   - 计划支持：流动性池数据解析

2. Meteora DLMM
   - 状态：开发进行中
   - 计划支持：动态流动性做市商程序数据解析


### 问题

1. SolFi DEX
   - 问题：SolFi 不采用流动性池模式，需要调整解析策略
   - 解决方案：正在重新设计数据结构以适配其交易模式
