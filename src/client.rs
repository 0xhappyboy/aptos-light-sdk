//! used to initialize the client and oper
use std::{borrow::Borrow, collections::HashMap, str::FromStr, sync::Mutex};

use anyhow::Ok;
use aptos_sdk::{
    coin_client::CoinClient,
    rest_client::{faucet, Client, FaucetClient},
};

use crate::config::{APTOS_DEV_NET_URL, APTOS_FAUCET_URL, APTOS_MAIN_NET_URL, APTOS_TEST_NET_URL};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    MAIN,
    TEST,
    DEV,
}

// client abstraction
pub struct AptosClient {
    mode: Mode,
    rest_client: Option<Client>,
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
            },
            Mode::TEST => AptosClient {
                mode: mode,
                rest_client: Some(Client::new(APTOS_TEST_NET_URL.clone())),
            },
            Mode::DEV => AptosClient {
                mode: mode,
                rest_client: Some(Client::new(APTOS_DEV_NET_URL.clone())),
            },
            _ => AptosClient {
                mode: mode,
                rest_client: None,
            },
        }
    }
    pub fn rest_client(&self) -> &Option<Client> {
        &self.rest_client
    }
    pub fn mode(&self) -> Mode {
        self.mode
    }
}
