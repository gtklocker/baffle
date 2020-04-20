use std::path::Path;
use std::env;
use std::fs;

pub mod compiler;
pub mod deploy;
pub mod constants;

fn compiler_main() {
    let contracts = compiler::get_contracts(Path::new(constants::CONTRACTS_PATH));
    println!("Compiling...");
    println!("{}", contracts.join("\n"));
    let build_path = Path::new(constants::BUILD_PATH);
    if !build_path.is_dir() {
        fs::create_dir(build_path).expect("failed to create build directory");
    }
    let args = compiler::args(build_path, &contracts);
    compiler::run(&args);
}

fn main() {
    for argument in env::args() {
        if argument == "compile" {
            compiler_main();
            break;
        }
    }
}