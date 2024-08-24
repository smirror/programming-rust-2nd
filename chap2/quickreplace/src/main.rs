#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

use text_colorizer::*;

fn print_usage() {
    eprintln!("{} - change occurrences of one string to another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <repalcement> <filename> <output>");
}

use std::env;

fn parse_args() -> Arguments{
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[1].clone(),
        replacement: args[2].clone(),
        filename: args[3].clone(),
        output: args[4].clone(),
    }
}
use std::fs;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                      "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
}



