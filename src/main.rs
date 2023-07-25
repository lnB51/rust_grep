/* 
Main file of rust grep project.
License: MIT
Github repo: https://github.com/lnB51/rust_grep
*/

use std::env;
use std::process;
use grep_rust::Config;

fn main() {
    // Get commang line arguments
    let args: Vec<String> = env::args().collect();

    // Create config struct from commang line arguments
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    
    // Return Application error on error case
    if let Err(e) = grep_rust::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}



