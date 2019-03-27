use std::env;
use std::io::prelude::*;
use std::process;

use colored::*;
use regex::Regex;
use select::document::Document;
use select::predicate::Attr;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();
    if args.len() < 2 {
        writeln!(
            &mut stderr,
            "Usage: explain [COMMAND] [ARG 1]..[ARG N]\nExample: explain ls -l -a\n"
        )
        .expect("couldn't write to stderr");
        process::exit(1);
    }
    //percent encode arguments for query
    let query = utf8_percent_encode(&args[1..].join(" "), DEFAULT_ENCODE_SET).to_string();
    explain(query);
}

fn explain(query: String) {
    let base_url = String::from("https://explainshell.com/explain?cmd=");
    let url = format!("{}{}", base_url, query);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());
    let delimiter = format!("\n\n{}", "_".repeat(50));
    // regex to match command line arguments for coloring
    let regex = Regex::new("^[ \t]*-.+").unwrap();
    Document::from_read(resp)
        .unwrap()
        // explanation selector
        .find(Attr("class", "help-box"))
        .enumerate()
        .for_each(|(i, node)| {
            node.text()
                .split("\n")
                .collect::<Vec<_>>()
                .iter()
                .enumerate()
                .for_each(|(x, line)| {
                    let mut output = line.to_string().white();
                    if i == 0 {
                        // style title
                        output = line.to_string().bold().underline().red();
                    } else if regex.is_match(line) && x < 3 {
                        // style command line arguments
                        output = line.to_string().bold().yellow();
                    }
                    print!("\n{}", output);
                });
            println!("{}", delimiter)
        });
}
