#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use std::io::{BufReader, Read};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct Config {
    config: Cloner,
}

#[derive(Debug, Deserialize)]
struct Cloner {
    host: String,
    dest_dir: std::path::PathBuf,
    repo: String,
}

fn read_file(path: std::path::PathBuf) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

fn main() {
    let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let s = match read_file(args.path.to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let config: Result<Config, toml::de::Error> = toml::from_str(&s);
    match config {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("fail to parse toml: {}", e),
    };
}

// cargo run -- sample.toml
