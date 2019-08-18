use std::path::Path;
use std::env;

pub mod compiler;
pub mod deploy;

fn compiler_main() {
    let contracts_path = Path::new("./contracts/");
    let contracts = compiler::get_contracts(contracts_path);
    println!("Compiling...");
    println!("{}", contracts.join("\n"));
    let args = compiler::args(&contracts);
    compiler::run(&args);
}

fn deploy_main() {
    deploy::run();
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