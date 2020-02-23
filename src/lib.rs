use std::path::Path;

pub mod compiler;
pub mod constants;
pub mod deploy;

pub use deploy::{deploy_contract, make_web3, make_web3_ganache};

pub fn get_artifact(contract_name: &str) -> deploy::ContractArtifact {
    deploy::get_artifact(Path::new(constants::BUILD_PATH), contract_name)
}
