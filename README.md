# <center> Aptos Light SDK </center>  
It is used to simplify complex operations on the aptos network, reduce the difficulty of aptos development, and allow more people to happily build on the aptos network. <br>
**continuous building** 🔨
## Use
```
### Cargo.toml
[dependencies]
aptos-light-sdk = {git = "https://github.com/0xhappyboy/aptos-light-sdk", branch = "main"}

[patch.crates-io]
merlin = {git = "https://github.com/aptos-labs/merlin"}
x25519-dalek = {git = "https://github.com/aptos-labs/x25519-dalek", branch = "zeroize_v1"}
### .cargo/config.toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```
## Module
```
account: provides operations for accounts
client : used to initialize the client and oper
config : overall situation config
transfer : provides methods for transactions
utils : internal utils
```
## Function
### account::create_new_account
create a account
#### Examples
```rust
let aptos_client = AptosClient::new(Mode::DEV);
create_new_account(aptos_client)
```
### account::create_account_by_private_key
create a account by private key
#### Examples
```rust
let aptos_client = AptosClient::new(Mode::DEV);
create_account_by_private_key(aptos_client,private_key)
```
### account::create_vanity_account
create a vanity account
#### Examples
```rust
let aptos_client = AptosClient::new(Mode::DEV);
create_vanity_account("6666".to_string(),"8888".to_string())
```
### client::AptosClient::new
initialize client instance
#### Examples
```rust
AptosClient::new(Mode::DEV)
```
### transfer::create_txn_hash
create a txn hash
#### Examples
```rust
let aptos_client = AptosClient::new(Mode::DEV);
crate_txn_hash(aptos_client,from address,to address,amount)
```
### transfer::send_txn_hash
send a txn hash
#### Examples
```rust
let aptos_client = AptosClient::new(Mode::DEV);
let txn = crate_txn_hash(aptos_client,from address,to address,amount)?
send_txn_hash(aptos_client,txn)
```