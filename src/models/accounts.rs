use super::block::{cn_time, Block};
use super::blockchain::Blockchain;
use super::transaction::Transactions;
use hex;
use rand::rngs::OsRng;
use rand::Rng;
use ring::signature::UnparsedPublicKey;
use ring::signature::{Ed25519KeyPair, KeyPair, Signature};
use ring::{agreement, signature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    private_key_seed: Vec<u8>,
    pub public_key: Vec<u8>,
    address: String,
    pub balance: u64,
}

impl Account {
    pub fn new(blockchain: &mut Blockchain) -> Account {
        //创建账户时生成密钥对
        let mut csprng = OsRng {};
        let seed: [u8; 32] = csprng.gen();
        let keypair = Ed25519KeyPair::from_seed_unchecked(&seed).expect("创建账户时生成密钥对失败");
        //通过公钥派生出地址
        let public_key = keypair.public_key().as_ref().to_vec();
        let ser_public_key = serde_json::to_string(&public_key).expect("序列化公钥失败");
        let mut hasher = Sha256::new();
        hasher.update(&ser_public_key);
        let address = format!("{:x}", hasher.finalize());
        let address1 = address.clone();
        //保存公钥和私钥种子以便后续对交易签名验证
        let account = Account {
            private_key_seed: seed.to_vec(),
            public_key,
            address,
            balance: 100, //初始资金
        };
        blockchain
            .accounts
            .insert(address1.clone(), account.clone());
        account
    }
    //每个账户都可以发起向别的账户的交易，并对交易进行签名
    pub fn create_trans(
        &mut self,
        other_account: &mut Account,
        amount: u64,
        ser_fee: u64,
        blockchain: &mut Blockchain,
    ) {
        // 检查发送方余额是否足够
        if self.balance < (amount + ser_fee) {
            println!("交易失败：余额不足。");
            return;
        }
        // 验证交易是否有效
        let transaction = Transactions::new(
            self.address.clone(),
            other_account.address.clone(),
            amount,
            vec![],
            self.public_key.clone().to_vec(), // 签名和公钥字段在签名后再填充
            cn_time(),
            ser_fee,
        );

        // 更新交易双方的账户余额
        self.balance -= amount;
        self.balance -= ser_fee;
        other_account.balance += amount;

        // 对交易数据进行签名
        let trans_bytes = serde_json::to_vec(&transaction).expect("序列化交易数据失败");
        let keypair = Ed25519KeyPair::from_seed_unchecked(&self.private_key_seed)
            .expect("使用私钥种子生成密钥对失败");
        let signature = keypair.sign(&trans_bytes);
        let mut signed_transaction = transaction;
        signed_transaction.signture = signature.as_ref().to_vec();
        signed_transaction.public_key = keypair.public_key().as_ref().to_vec();

        // 将签名后的交易添加到交易池
        blockchain.pending_transactions.push(signed_transaction);
        println!("创建的交易成功添加到交易池中!");

        // 更新交易双方账户余额
        blockchain.accounts.get_mut(&self.address).unwrap().balance = self.balance;
        blockchain
            .accounts
            .get_mut(&other_account.address)
            .unwrap()
            .balance = other_account.balance;
    }

    //每个账户拥有挖矿功能，异步体现出竞争性
    //会将区块链中待处理的交易按照手续费排序，添加两个最高的到块中
    pub async fn add_block(&mut self, blockchain: &mut Blockchain) {
        let mut new_block = Block::new(
            blockchain.chains.len() as u64,
            blockchain.chains[&blockchain.chains.len() - 1].hash.clone(),
        );
        //验证交易是否有效
        println!("在交易池中找到一笔交易，正在验证该交易是否有效。");
        if self.verify_signature(
            &blockchain.pending_transactions[0],
            &blockchain.pending_transactions[0].public_key,
        ) {
            println!("交易验证成功。");
            return;
        }
        new_block.mine(&blockchain.clone());
        new_block
            .transactions
            .push(blockchain.pending_transactions.remove(0));
        blockchain.chains.push(new_block.clone());
        println!(
            "挖矿成功，新区快索引为：{}，矿工地址为：{}矿工获得了奖励和手续费！",
            new_block.index,
            self.address.clone()
        );
        //更新矿工的账户余额
        self.balance += 50;
        self.balance += new_block.transactions[0].service_fee;
        blockchain.accounts.get_mut(&self.address).unwrap().balance = self.balance;
    }
    // 验证签名的函数
    pub fn verify_signature(&self, transaction: &Transactions, public_key: &[u8]) -> bool {
        let signature_bytes = &transaction.signture;
        let message = serde_json::to_vec(&transaction).expect("序列化交易数据失败");
        // 创建 UnparsedPublicKey 实例
        let public_key = UnparsedPublicKey::new(&signature::ED25519, public_key);

        // 使用 UnparsedPublicKey 的 verify 方法来验证签名
        public_key.verify(&message, signature_bytes).is_ok()
    }
}
