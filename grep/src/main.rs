use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use clap::Parser;


#[derive(Parser, Debug)]
/// Find strings in text files
struct Args {
    /// The file to be searched
    #[clap(short='p', long)]
    path: String,

    sarch_string: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();
    // println!("{:?}", args);

    search_in_file(&args.path, &args.sarch_string)
}

fn search_in_file(path: &str, search_string: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;

    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.contains(search_string) {
            println!("{}", line);
        }
    }

    Ok(())
}