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
faucet : faucet related methods, valid in Mode::DEV Mode::TEST mode
utils : internal utils
```
## Function
### account::create_new_account
create a account
#### Examples
```rust
account::create_new_account()
```
### account::create_account_by_private_key
create a account by private key
#### Examples
```rust
let mut aptos_client = AptosClient::new(Mode::DEV);
account::create_account_by_private_key(&mut aptos_client,"private_key")
```
### account::create_vanity_account
create a vanity account
#### Examples
```rust
/// trying to create an account whose public key meets the conditions starting with 6 and ending with 8
account::create_vanity_account("6666".to_string(),"8888".to_string())
```
### account::get_public_key
get the public key string of an account
#### Examples
```rust
get_public_key(&account).unwrap();
```
### account::get_private_key
get the private key string of an account
#### Examples
```rust
get_private_key(&account).unwrap();
```
### account::get_account_balance
get account balance
#### Examples
```rust
let mut aptos_client = AptosClient::new(Mode::DEV);
get_account_balance(&mut aptos_client,&account).unwrap();
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
### faucet::get_faucet_coin
use the designated account to obtain faucet tokens
#### Examples
```rust
let aptos_client = AptosClient::new(Mode::DEV);
let mut account = create_new_account();
faucet::get_faucet_coin(&aptos_client,account,10)
```