/*
Lib file of rust grep project, contains all structs, enums and functions.
License: MIT
Github repo: https://github.com/b1rd-dev/rust_grep
*/

#![allow(unused_assignments)]

use std::fs;
use std::error::Error;
use std::env;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Clean the terminal output before printing
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
    // \x1b[38;2;43;255;0;1m{}\x1b[0m - used for green highlighting of text in {}
    println!("Searching for: \x1b[38;2;43;255;0;1m{}\x1b[0m", config.query);
    let mut contents = String::new();
    
    // Match search type and set content
    match  config.s_type{
        SearchOptions::File => {
            println!("In the file: \x1b[38;2;43;255;0;1m{}\x1b[0m\n", config.target);
            contents = fs::read_to_string(config.target)?;
        },
        SearchOptions::String => {
            println!("In the string: \x1b[38;2;43;255;0;1m{}\x1b[0m\n", config.target);
            contents = config.target;
        },
    }

    // Call search functions(depends on search method)
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    } else{
        search_case_insensitive(&config.query, &contents)
    };

    // Check search results
    if results.len() > 0{
        for line in results {
            println!("{}", line);
        }
    } else{
        println!("\x1b[38;2;237;237;0;1mNothing found\x1b[0m\n");
    }

    Ok(())
}


// Struct for config
pub struct Config{
    pub query: String,
    pub s_type: SearchOptions,
    pub target: String,
    pub case_sensitive: bool,
}

impl Config {
    // Create config struct from console args
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4{
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let s_type = SearchOptions::new(args[2].clone());
        let target = args[3].clone();

        // Takes value from env, to set value on macOS/Linux use export [value]
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config{
            query, 
            s_type,
            target,
            case_sensitive,
        })
    }
}

// Available search methods
pub enum SearchOptions {
    File,
    String
}

impl SearchOptions {
    // Convert string console param to SearchOptions type
    fn new(target: String) -> SearchOptions{
        match target.as_str() {
            "-f" => SearchOptions::File,
            "-s" => SearchOptions::String,
            _ => {
                println!("Unexpected search type!");
                process::exit(1)
            }
        }
    }
}

// Case sensitive search method(HELLO != hello)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String>{
    let mut result = Vec::new();
    for line in contents.lines(){

        // Reformat line to array by whitespaces
        let line_split: Vec<_> = line
        .split_whitespace()
        .collect();

        let mut result_line = String::new();

        // Checks is the query in the line
        if line.contains(&query){
            for word in line_split{
                // Add highlighting for query word in string
                if word.contains(&query){
                    result_line.push_str(
                format!(
                            "\x1b[38;2;43;255;0;1m{}\x1b[0m ", 
                            word
                        ).as_str()
                    );
                } else {
                    result_line.push_str(
                        format!("{} ", word).as_str()
                    );
                }
            }
            result.push(result_line)
        }
    }
    result
}

// Case insensitive search method(HELLO == hello)
pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<String>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines(){

        // Reformat line to array by whitespaces
        let line_split: Vec<_> = line
        .split_whitespace()
        .collect();

        let mut result_line = String::new();

        // Checks is the query in the line
        if line.to_lowercase().contains(&query){
            for word in line_split{

                // Add highlighting for query word in string
                if word.to_lowercase().contains(&query){
                    result_line.push_str(
                format!(
                            "\x1b[38;2;43;255;0;1m{}\x1b[0m ", 
                            word
                        ).as_str()
                    );
                } else {
                    result_line.push_str(
                        format!("{} ", word).as_str()
                    );
                }
            }
            result.push(result_line)
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, \u{1b}[38;2;43;255;0;1mproductive.\u{1b}[0m "], 
        search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["\u{1b}[38;2;43;255;0;1mRust:\u{1b}[0m ", 
        "\u{1b}[38;2;43;255;0;1mTrust\u{1b}[0m me. "], 
        search_case_insensitive(query, contents));
    }
}

