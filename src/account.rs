//! Provides operations for accounts
use aptos_sdk::{coin_client::CoinClient, types::LocalAccount};

use crate::client::AptosClient;

/// create a account
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// create_new_account(aptos_client)
/// ```
pub fn create_new_account(aptos_client: &mut AptosClient) -> Result<LocalAccount, String> {
    let mut account = LocalAccount::generate(&mut rand::rngs::OsRng);
    Ok(account)
}

/// create a account by private key
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// create_account_by_private_key(aptos_client,private_key)
/// ```
pub fn create_account_by_private_key(
    aptos_client: &mut AptosClient,
    private_key: &str,
) -> Result<LocalAccount, String> {
    let account = LocalAccount::from_private_key(private_key, 0).unwrap();
    Ok(account)
}

/// create a vanity account
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// create_vanity_account("6666".to_string(),"8888".to_string())
/// ```
pub fn create_vanity_account(start: String, end: String) -> Result<LocalAccount, String> {
    Err("".to_string())
}
