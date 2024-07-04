//! used to initialize the client and oper
use std::{collections::HashMap, str::FromStr, sync::Mutex};

use aptos_sdk::{
    coin_client::CoinClient,
    rest_client::{Client, FaucetClient},
};

use crate::config::{APTOS_DEV_NET_URL, APTOS_FAUCET_URL, APTOS_MAIN_NET_URL, APTOS_TEST_NET_URL};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    MAIN,
    TEST,
    DEV,
}

// client abstraction
#[derive(Debug)]
pub struct AptosClient {
    mode: Mode,
    rest_client: Option<Client>,
    faucet_client: Option<FaucetClient>,
}

impl AptosClient {
    /// Initialize client instance
    ///
    /// # Examples
    ///
    /// ```
    /// AptosClient::new(Mode::DEV)
    /// ```
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::MAIN => AptosClient {
                mode: mode,
                rest_client: Some(Client::new(APTOS_MAIN_NET_URL.clone())),
                faucet_client: None,
            },
            Mode::TEST => AptosClient {
                mode: mode,
                rest_client: Some(Client::new(APTOS_TEST_NET_URL.clone())),
                faucet_client: Some(FaucetClient::new(
                    APTOS_FAUCET_URL.clone(),
                    APTOS_TEST_NET_URL.clone(),
                )),
            },
            Mode::DEV => AptosClient {
                mode: mode,
                rest_client: Some(Client::new(APTOS_DEV_NET_URL.clone())),
                faucet_client: Some(FaucetClient::new(
                    APTOS_FAUCET_URL.clone(),
                    APTOS_TEST_NET_URL.clone(),
                )),
            },
            _ => AptosClient {
                mode: mode,
                rest_client: None,
                faucet_client: None,
            },
        }
    }
    pub fn rest_client(&self) -> &Option<Client> {
        &self.rest_client
    }
    pub fn faucet_client(&self) -> &Option<FaucetClient> {
        &self.faucet_client
    }
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
}
