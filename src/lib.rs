use std::path::Path;

pub mod compiler;
pub mod deploy;
pub mod constants;

pub fn get_artifact(contract_name: &str) -> deploy::ContractArtifact {
    deploy::get_artifact(Path::new(constants::BUILD_PATH), contract_name)
}