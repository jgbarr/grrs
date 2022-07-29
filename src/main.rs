#![allow(unused)]

use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(name = "grrs")]
#[clap(author = "jgbarr")]
#[clap(version = "1.0")]
#[clap(about = "Search for a string in a file", long_about = None)]
struct Cli {
    /// search for a string in file
    pattern: String,
    /// read search path
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    
    let cli = Cli::parse();
    let content = std::fs::read_to_string(&cli.path)
        .expect("could not read file");
        for line in content.lines() {
            if line.contains(&cli.pattern) {
                println!("{}", line);
            }
        }
    println!("pattern: {:?}", cli.pattern);
    println!("path: {:?}", cli.path);
    println!("arg fuck kill");
}
