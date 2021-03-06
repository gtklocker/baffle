extern crate web3;

use std::{fs, time};
use std::path::Path;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::Address;
use web3::Web3;

pub struct ContractArtifact {
    abi: Vec<u8>,
    bin: String
}

fn get_artifact_for_files(abi_file: &str, bin_file: &str) -> ContractArtifact {
    ContractArtifact {
        abi: fs::read(abi_file).unwrap(),
        bin: fs::read_to_string(bin_file).unwrap()
    }
}

pub fn get_artifact(build_path: &Path, contract_name: &str) -> ContractArtifact {
    let abi_path = build_path.join(format!("{}.abi", contract_name));
    let bin_path = build_path.join(format!("{}.bin", contract_name));
    get_artifact_for_files(abi_path.to_str().unwrap(), bin_path.to_str().unwrap())
}

pub fn deploy_contract<T: web3::Transport>(artifact: &ContractArtifact, web3: &Web3<T>, from_address: Address) -> Contract<T> {
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

pub fn make_web3(rpc_url: &str) -> (web3::transports::EventLoopHandle, Web3<web3::transports::Http>) {
    let (_eloop, transport) = web3::transports::Http::new(rpc_url).unwrap();
    (_eloop, web3::Web3::new(transport))
}

pub fn make_web3_ganache() -> (web3::transports::EventLoopHandle, Web3<web3::transports::Http>) {
    make_web3("http://localhost:8545")
}