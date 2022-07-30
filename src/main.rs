#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use std::io::{BufReader, Read};
use std::process::Command;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct Cloner {
    config: Config,
}

#[derive(Debug, Deserialize)]
struct Config {
    host: String,
    dest_dir: String,
    repos: Vec<String>,
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

fn clone_cmd(config: Config) {
    std::env::set_current_dir(&config.dest_dir).unwrap();

    println!("────────────────────────────");
    println!("{}", &format!("host: {}", config.host));
    println!("{}", &format!("destination: {}", config.dest_dir));
    println!("────────────────────────────");

    for repo in &config.repos {
        println!("{}", &format!("✔ {}", repo));

        let arg = &["clone", &format!("git@github.com:{}.git", repo)];
        let output = Command::new("git").args(arg).output().expect("failed");
    }
}

fn main() {
    let args = Cli::parse();

    // read
    let str = match read_file(args.path.to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    // parse
    let cloner: Result<Cloner, toml::de::Error> = toml::from_str(&str);
    match cloner {
        Ok(p) => clone_cmd(p.config),
        Err(e) => panic!("fail to parse toml: {}", e),
    };
}

// cargo run -- sample.toml
