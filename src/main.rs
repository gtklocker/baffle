use std::path::Path;

pub mod compiler;

fn main() {
    let contracts_path = Path::new("./contracts/");
    let contracts = compiler::get_contracts(contracts_path);
    println!("Compiling...");
    println!("{}", contracts.join("\n"));
    let args = compiler::args(&contracts);
    compiler::run(&args);
}