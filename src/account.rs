//! Provides operations for accounts
use anyhow::Ok;
use aptos_sdk::{coin_client::CoinClient, crypto::hash::TestOnlyHash, types::LocalAccount};

use crate::client::AptosClient;

/// create a account
///
/// # Examples
///
/// ```
/// create_new_account()
/// ```
pub fn create_new_account() -> Result<LocalAccount, String> {
    let mut account = LocalAccount::generate(&mut rand::rngs::OsRng);
    Ok(account)
}

/// create a account by private key
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// create_account_by_private_key(&aptos_client,private_key)
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
    // create new account
    let mut account: Result<LocalAccount, String> = create_new_account();
    loop {
        // get public key string
        let mut public_key_string = get_public_key(account).unwrap();
        public_key_string = public_key_string.replace("0x", "");
        if (public_key_string.starts_with(start) && public_key_string.ends_with(end)) {
            // to meet the conditions
            break;
        } else {
            account = create_new_account();
        }
    }
    Ok(account)
}

/// get the public key string of an account
///
/// # Examples
///
/// ```
/// get_public_key(account)
/// ```
pub fn get_public_key(account: &LocalAccount) -> Result<String, String> {
    Ok(account.address().to_hex_literal())
}

/// get the private key string of an account
///
/// # Examples
///
/// ```
/// get_private_key(account)
/// ```
pub fn get_private_key(account: &LocalAccount) -> Result<String, String> {
    Ok(account.private_key().to_encoded_string()?)
}

/// get account balance
///
/// # Examples
///
/// ```
/// get_account_balance(account)
/// ```
pub async fn get_account_balance(
    aptos_client: &mut AptosClient,
    account: &LocalAccount,
) -> Result<String, String> {
    let coin_client = CoinClient::new(&aptos_client.rest_client());
    Ok(coin_client.get_account_balance(&account.address()).await)
}
