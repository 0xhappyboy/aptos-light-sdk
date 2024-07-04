//! faucet related methods, valid in Mode::DEV Mode::TEST mode

/// use the designated account to obtain faucet tokens
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// let mut account = create_new_account();
/// faucet::get_faucet_coin(&aptos_client,account,10)
/// ```
pub async fn get_faucet_coin(
    aptos_client: &mut AptosClient,
    account: &LocalAccount,
    coin_amount: u64,
) {
    let faucet_client: Client = aptos_client.faucet_client().clone().unwrap();
    faucet_client
        .fund(account.address(), utils::wrap_coin_amount(coin_amount))
        .await;
}
