//! Provides methods for transactions

use anyhow::{Context, Ok};
use aptos_sdk::{
    coin_client::CoinClient,
    rest_client::{Client, PendingTransaction},
    types::LocalAccount,
};

use crate::{client::AptosClient, utils::wrap_coin_amount};

/// create a txn hash
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// crate_txn_hash(aptos_client,from address,to address,amount)
/// ```
pub async fn create_txn_hash(
    aptos_client: &mut AptosClient,
    from: &mut LocalAccount,
    to: &mut LocalAccount,
    amount: u64,
) -> Result<PendingTransaction, String> {
    let rest_client: Client = aptos_client.rest_client().clone().unwrap();
    let coin_client = CoinClient::new(&rest_client);
    let txn_hash: Result<aptos_sdk::rest_client::PendingTransaction, anyhow::Error> = coin_client
        .transfer(from, to.address(), wrap_coin_amount(amount), None)
        .await;
    Err("err".to_string())
}

/// send a txn hash
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// let txn = crate_txn_hash(&aptos_client,from address,to address,amount)?
/// send_txn_hash(aptos_client,txn)
/// ```
pub async fn send_txn_hash(
    aptos_client: &mut AptosClient,
    txn_hash: PendingTransaction,
) -> Result<PendingTransaction, String> {
    let rest_client: Client = aptos_client.rest_client().clone().unwrap();
    rest_client.wait_for_transaction(&txn_hash).await;
    Err("err".to_string())
}
