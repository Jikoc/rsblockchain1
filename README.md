## 稍微复杂一点的区块链代码示例

创建账户--生成一对密钥--公钥派生出地址--保存私钥种子--发起交易--将交易保存交易池中--矿工验证并打包交易--工作证明生成新的区块--挖矿为异步函数表现出竞争--矿工获得新区快奖励和交易手续费

## 输出示例
- 创建的交易成功添加到交易池中!
- 创建的交易成功添加到交易池中!
- 挖矿成功，新区快索引为：1，矿工地址为：`28841208d65fb4ff8c9539cf97f50915a6287c4e853adeaa912858032bb62b71`矿工获得了奖励和手续费！
- 挖矿成功，新区快索引为：2，矿工地址为：`5a374ff48a8a561eb5d27364aaac4cdf7a0d5b7094afb918a0043c0a8aa47ee7`矿工获得了奖励和手续费！

### 区块链数据
####  Blockchain
- `difficulty`: 3
- `genesis_block`:
  - `index`: 0
  - `time`: "2024-06-09 17:37:18"
  - `prev_hash`: ""
  - `nonce`: 0
  - `hash`: ""
  - `transactions`: []

#### Blocks
1. `Block 0`:
   - `index`: 0
   - `time`: "2024-06-09 17:37:18"
   - `prev_hash`: ""
   - `nonce`: 0
   - `hash`: ""
   - `transactions`: []
2. `Block 1`:
   - `index`: 1
   - `time`: "2024-06-09 17:37:18"
   - `prev_hash`: ""
   - `nonce`: 1280
   - `hash`: "0000f3c7f9e74d6ea2eb8d3224150e042da64feea62f13e170ed5b87cf08af6b"
   - `transactions`:
     - `Transactions`:
       - `sender`: "28841208d65fb4ff8c9539cf97f50915a6287c4e853adeaa912858032bb62b71"
       - `receiver`: "5a374ff48a8a561eb5d27364aaac4cdf7a0d5b7094afb918a0043c0a8aa47ee7"
       - `amount`: 10
       - `signture`: [略]
       - `public_key`: [略]
       - `time`: "2024-06-09 17:37:18"
       - `service_fee`: 1

3. `Block 2`:
   - `index`: 2
   - `time`: "2024-06-09 17:37:18"
   - `prev_hash`: "0000f3c7f9e74d6ea2eb8d3224150e042da64feea62f13e170ed5b87cf08af6b"
   - `nonce`: 2116
   - `hash`: "000ae23eafd7b04941c9238a9a696f056e444b31ef62b6b4f756cf200b7ee7ae"
   - `transactions`:
     - `Transactions`:
       - `sender`: "5a374ff48a8a561eb5d27364aaac4cdf7a0d5b7094afb918a0043c0a8aa47ee7"
       - `receiver`: "28841208d65fb4ff8c9539cf97f50915a6287c4e853adeaa912858032bb62b71"
       - `amount`: 50
       - `signture`: [略]
       - `public_key`: [略]
       - `time`: "2024-06-09 17:37:18"
       - `service_fee`: 2

#### Accounts
- `Account 1`:
  - `private_key_seed`: [略]
  - `public_key`: [略]
  - `address`: "5a374ff48a8a561eb5d27364aaac4cdf7a0d5b7094afb918a0043c0a8aa47ee7"
  - `balance`: 110
- `Account 2`:
  - `private_key_seed`: [略]
  - `public_key`: [略]
  - `address`: "28841208d65fb4ff8c9539cf97f50915a6287c4e853adeaa912858032bb62b71"
  - `balance`: 190

#### Pending Transactions
- 无待处理交易
