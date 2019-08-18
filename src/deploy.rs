//based on examples/contract.rs
extern crate rustc_hex;
extern crate web3;

use std::{fs, time};
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{Address, U256};
use web3::Web3;

fn deploy_contract<T: web3::Transport>(bin_file: &str, abi_file: &str, web3: Web3<T>, from_address: Address) -> Contract<T> {
    let bytecode = fs::read_to_string(bin_file).unwrap();
    return Contract::deploy(web3.eth(), &fs::read(abi_file).unwrap())
        .unwrap()
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), from_address)
        .unwrap()
        .wait()
        .unwrap()
}

pub fn run() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();

    //Get current balance
    let balance = web3.eth().balance(accounts[0], None).wait().unwrap();

    println!("Balance: {}", balance);
    let bin_file = "./contracts-output/SimpleStorage.bin";
    let abi_file = "./contracts-output/SimpleStorage.abi";
    let contract = deploy_contract(bin_file, abi_file, web3, accounts[0]);

    println!("{}", contract.address());

    //interact with the contract
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);

    //Change state of the contract
    contract.call("set", (42,), accounts[0], Options::default());

    //View changes made
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);
}
