use ethers::prelude::*;
use std::{convert::TryFrom, env};

pub fn get_provider() -> Provider<Http> {
    let project_id = env::var("INFURA_PROJECT_ID").expect("Did not set INFURA_PROJECT_ID!");
    let infura = format!("https://mainnet.infura.io/v3/{}", project_id);
    let provider = Provider::<Http>::try_from(infura).unwrap();
    provider
}

pub async fn get_block_num(provider: &Provider<Http>) -> Result<U64, anyhow::Error> {
    let block_number: U64 = provider.get_block_number().await.unwrap();
    Ok(block_number)
}

pub async fn get_block(
    provider: &Provider<Http>,
    block_number: U64,
) -> Result<Option<Block<Transaction>>, anyhow::Error> {
    let block = provider.get_block_with_txs(block_number).await.unwrap();
    Ok(block)
}

pub async fn get_tx(
    provider: &Provider<Http>,
    hash: TxHash,
) -> Result<Option<Transaction>, anyhow::Error> {
    let tx = provider.get_transaction(hash).await.unwrap();
    Ok(tx)
}
