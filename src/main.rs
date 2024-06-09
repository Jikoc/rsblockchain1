mod models;
use std::error::Error;
#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>>{
    let mut blockchain = models::blockchain::Blockchain::new(3);
    let mut Bob = models::accounts::Account::new(&mut blockchain);
    let mut Alice = models::accounts::Account::new(&mut blockchain);
    Bob.create_trans(&mut Alice,10,1,&mut blockchain);
    Alice.create_trans(&mut Bob,50,2,&mut blockchain);
    Bob.add_block(&mut blockchain).await;
    Alice.add_block(&mut blockchain).await;
    println!("区块链全部数据：{:#?}",blockchain);
    Ok(())
}
