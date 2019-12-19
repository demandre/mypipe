extern crate clap; 
use clap::{Arg, App};
use std::process::Command;
use std::io::{self, Write};
use std::str;
 
fn main() { 
    let matches = App::new("Pipe")
       .version("1.0")
       .about("Simple Pipe")
       .author("Demandre J.")
       .arg(Arg::with_name("in")
            .short("in")
            .long("in")
            .value_name("INPUT_COMMAND")
            .help("Sets input command to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("out")
            .short("out")
            .long("out")
            .value_name("OUTPUT_COMMAND")
            .help("Sets output command to use")
            .takes_value(true)
            .required(true))
       .get_matches(); 

    let input = matches.value_of("in").unwrap();
    let output = matches.value_of("out").unwrap();

    let input_output = Command::new(input)
        .output()
        .expect("failed to execute input command");

    let input_output_concat = match str::from_utf8(&input_output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let output_output = Command::new(output)
        .arg(input_output_concat)
        .output()
        .expect("failed to execute output command");

    io::stdout().write_all(&output_output.stdout).unwrap();
}