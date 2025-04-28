### GRPC数据订阅

#### 成功订阅raydiumClmm
* 数据返回正常，长度为:
    * 池子数据长度: 1544字节
* 数据一 池子信息：成功获取

池子信息结构:
```Typescript
export const RAYDIUM_CLMM_LAYOUT = struct([
        u64('discriminator'),
        u8('bump'),
        publicKey('ammConfig'),
        publicKey('owner'),
        publicKey('tokenMint0'),
        publicKey('tokenMint1'),
        publicKey('tokenVault0'),
        publicKey('tokenVault1'),
        publicKey('observationKey'),
        u8('mintDecimals0'),
        u8('mintDecimals1'),
        u16('tickSpacing'),
        u128('liquidity'),
        u128('sqrtPriceX64'),
        i32('tickCurrent'),
        u8('status'),
        u64('protocolFeesToken0'),
        u64('protocolFeesToken1')
]);
```




