//based on examples/contract.rs
extern crate rustc_hex;
extern crate web3;

use std::{fs, time};
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{Address, U256};
use web3::Web3;

struct ContractArtifact {
    abi: Vec<u8>,
    bin: String
}

fn get_artifact(abi_file: &str, bin_file: &str) -> ContractArtifact {
    ContractArtifact {
        abi: fs::read(abi_file).unwrap(),
        bin: fs::read_to_string(bin_file).unwrap()
    }
}

fn deploy_contract<T: web3::Transport>(artifact: &ContractArtifact, web3: &Web3<T>, from_address: Address) -> Contract<T> {
    return Contract::deploy(web3.eth(), &artifact.abi)
        .unwrap()
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(&artifact.bin, (), from_address)
        .unwrap()
        .wait()
        .unwrap()
}

pub fn run() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();
    let first_account = accounts[0];

    let simple_storage_artifact = get_artifact("./contracts-output/SimpleStorage.abi", "./contracts-output/SimpleStorage.bin");
    let contract = deploy_contract(&simple_storage_artifact, &web3, first_account);
    println!("{}", contract.address());

    //interact with the contract
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);

    //Change state of the contract
    contract.call("set", (42,), first_account, Options::default());

    //View changes made
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);
}
