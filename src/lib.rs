use std::path::Path;

pub mod compiler;
pub mod deploy;
pub mod constants;

pub use deploy::{make_web3_ganache, deploy_contract};

pub fn get_artifact(contract_name: &str) -> deploy::ContractArtifact {
    deploy::get_artifact(Path::new(constants::BUILD_PATH), contract_name)
}