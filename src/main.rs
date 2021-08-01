extern crate dotenv;
use dotenv::dotenv;

mod util;
use util::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let provider = get_provider();
    let block_number = get_block_num(&provider).await.unwrap();
    print!("Block: {b}\n", b = block_number);
    let block = get_block(&provider, block_number).await.unwrap();
    match block {
        Some(x) => {
            let txs = x.transactions;
            for tx in &txs {
                let t = get_tx(&provider, tx.hash).await.unwrap();
                match t {
                    Some(x) => println!("Hash: {:?}", x.hash),
                    None => println!("Error"),
                }
            }
        }
        None => println!("could not get block"),
    }
}
