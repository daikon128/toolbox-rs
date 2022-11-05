use clap::Parser;
use std::io;
use std::io::{BufRead, BufReader, Stdin};

#[derive(Parser)]
struct Args {
    /// input text
    input: Option<String>,
    /// separate pattern
    #[arg(short, long)]
    pattern: String,

    /// output format
    #[arg(short, long)]
    format: String
}

fn read_from_stdin() -> Result<String, io::Error> {
    let stdin: Stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut buffer = String::new();

    return match reader.read_line(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => {
            Err(e)
        }
    }
}

fn get_args_or_stdin(input: Option<String>) -> Result<String, io::Error> {
    match input {
        Some(string) => Ok(string),
        None => read_from_stdin()
    }
}

fn main() {
    let args = Args::parse();
    let input = match get_args_or_stdin(args.input) {
        Ok(input) => input,
        Err(e) => panic!("{}", e)
    };

    println!("arg 1 or stdin: {}", input);
    println!("arg 2: {}", args.pattern);
    println!("arg 3: {}", args.format);
}

