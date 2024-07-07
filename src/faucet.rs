//! faucet related methods, valid in Mode::DEV Mode::TEST mode
use crate::client;
use crate::config::FAUCET_CLIENT;
use crate::utils;
use aptos_sdk::
    types::LocalAccount
;

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
    aptos_client: &client::AptosClient,
    account: &LocalAccount,
    coin_amount: f64,
) {
    FAUCET_CLIENT
        .fund(account.address(), utils::wrap_coin_amount(coin_amount))
        .await;
}
