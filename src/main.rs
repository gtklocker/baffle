use std::path::Path;
use std::env;

pub mod compiler;
pub mod deploy;
pub mod constants;

fn compiler_main() {
    let contracts = compiler::get_contracts(Path::new(constants::CONTRACTS_PATH));
    println!("Compiling...");
    println!("{}", contracts.join("\n"));
    let args = compiler::args(Path::new(constants::BUILD_PATH), &contracts);
    compiler::run(&args);
}

fn deploy_main() {
    deploy::run(Path::new(constants::BUILD_PATH));
}

fn main() {
    for argument in env::args() {
        if argument == "compile" {
            compiler_main();
            break;
        }
        if argument == "deploy" {
            deploy_main();
            break;
        }
    }
}