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
    aptos_client: &AptosClient,
    from: &mut LocalAccount,
    to: &mut LocalAccount,
    amount: f64,
) -> PendingTransaction {
    let rest_client: Client = aptos_client.rest_client().clone().unwrap();
    let coin_client = CoinClient::new(&rest_client);
    let txn_hash: Result<aptos_sdk::rest_client::PendingTransaction, anyhow::Error> = coin_client
        .transfer(from, to.address(), wrap_coin_amount(amount), None)
        .await;
    txn_hash.unwrap()
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
pub async fn send_txn_hash(aptos_client: &AptosClient, txn_hash: PendingTransaction) {
    let rest_client: Client = aptos_client.rest_client().clone().unwrap();
    rest_client.wait_for_transaction(&txn_hash).await;
}

/// batch transfer
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// let mut from = account::create_account_by_private_key("0x");
/// let mut to_1 = account::create_account_by_private_key("0x");
/// let mut to_2 = account::create_account_by_private_key("0x");
///
/// let mut v = vec![];
/// v.push(to_1);
/// v.push(to_2);
///
/// transfer::batch_transfer(&aptos_client, &mut from, &v, 0.1).await;
/// ```
pub async fn batch_transfer(
    aptos_client: &AptosClient,
    from: &mut LocalAccount,
    to: &Vec<LocalAccount>,
    amount: f64,
) {
    for (index, to_accout) in to.iter().enumerate() {
        let rest_client: Client = aptos_client.rest_client().clone().unwrap();
        let coin_client = CoinClient::new(&rest_client);
        let txn_hash = coin_client
            .transfer(from, to_accout.address(), wrap_coin_amount(amount), None)
            .await;
        //send_txn_hash(&aptos_client, txn).await;
    }
}
