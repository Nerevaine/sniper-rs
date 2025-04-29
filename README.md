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

3. Meteora DLMM
   - 文件：`src/dex/meteora_dlmm.rs`
   - 支持解析动态流动性做市商程序数据 (904字节)
   - 支持解析 BinArray 账户数据 (3232字节)
     - 包含 Bin 数组信息和状态
     - 支持显示流动性分布数据
     - 支持实时流动性分析
     - 支持 Bin 范围和深度查询
   - 支持解析 Oracle 账户数据 (10136字节)
     - 包含价格预言机数据和更新时间戳
     - 支持历史价格数据查询和分析
     - 支持价格波动率计算
     - 支持移动平均价格追踪
     - 支持多时间周期价格统计

### 开发中的功能

1. SolFi DEX
   - 状态：开发进行中
   - 计划支持：流动性池数据解析

### 问题

1. SolFi DEX
   - 问题：SolFi 不采用流动性池模式，需要调整解析策略
   - 解决方案：正在重新设计数据结构以适配其交易模式
