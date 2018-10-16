mod minigrep;
mod minicat;
mod minicp;

use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = minicp::Config::new(&args)
        .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
        });


    if let Err(e) = minicp::run(conf) {
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}
