use std::collections::HashMap;

use aptos_light_sdk::{
    account,
    client::{AptosClient, Mode},
    faucet, rest,
    transfer::{self, Transaction},
    utils,
};

use aptos_sdk::{
    rest_client::{self, Client},
    types::PeerId,
};
use reqwest::Result;
use tokio;

#[tokio::main]
async fn main() {
    let aptos_client = AptosClient::new(Mode::DEV);
    let mut from = account::create_account_by_private_key(
        "0x",
    );
    // let mut account1 = account::create_account_by_private_key(
    //     "0x",
    // );
    // let mut account2 = account::create_account_by_private_key(
    //     "0x",
    // );
    let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    rest_api
        .get_transactions_by_address(account::get_public_key(&from).as_ref())
        .await;
    // rest_api
    //     .get_address_resources(account::get_public_key(&from).as_ref())
    //     .await;
    // // create a new transaction object
    // let mut transaction: Transaction = Transaction::new(&aptos_client, from, account1, 0.1, None)
    //     .await
    //     .unwrap();
    // // send
    // transaction.send(&aptos_client).await;
    //build signed transaction
    // let signed_transaction = t.build_signed_transaction(account, function_name, ty_args, args, tx_opts)
}
