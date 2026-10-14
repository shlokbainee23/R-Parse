use std::env;
use std::process;

use rparse_cli::run_cli;

fn main() {
    let (code, output) = run_cli(env::args());
    if !output.is_empty() {
        if code == 0 {
            println!("{}", output);
        } else {
            eprintln!("{}", output);
        }
    }
    process::exit(code);
}
