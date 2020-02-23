use std::path::Path;
use std::process::{Command, Stdio};

pub fn args(build_path: &Path, files: &[String]) -> Vec<String> {
    let args = [
        "-o",
        build_path.to_str().unwrap(),
        "--overwrite",
        "--bin",
        "--abi",
    ];

    args.iter()
        .map(ToString::to_string)
        .chain(files.iter().cloned())
        .collect()
}

pub fn run(args: &[String]) {
    Command::new("solc")
        .args(args)
        .stdin(Stdio::null())
        .status()
        .expect("solc command failed to start");
}

fn get_files_with_extension(dir_path: &Path, _ext: String) -> Vec<String> {
    let file_paths = dir_path
        .read_dir()
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path());
    file_paths
        .filter(|p| {
            p.extension()
                .and_then(|ext| ext.to_str())
                .filter(|ext_str| ext_str.to_string() == _ext)
                .is_some()
        })
        .filter_map(|p| p.to_str().map(|s| s.to_string()))
        .collect()
}

pub fn get_contracts(contracts_path: &Path) -> Vec<String> {
    get_files_with_extension(contracts_path, "sol".to_string())
}
