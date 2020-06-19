extern crate clap;

use std::fs::File;

use clap::{App, Arg};

mod interpreter;
mod language;
mod optimizer;

use interpreter::{Interpreter, ReadSource};
use language::parse_bf_file;
use optimizer::optimize_instructions;

fn main() {
    let matches = App::new("Membrain")
        .version("0.1.0")
        .author("CHIKASOFT")
        .arg(
            Arg::new("optimize")
                .short('O')
                .long("optimize")
                .takes_value(false)
                .about("Optimize Brainfuck before interpreting."),
        )
        .arg(
            Arg::new("read")
                .short('r')
                .long("read")
                .takes_value(true)
                .about("Get user-input from a file instead of stdin."),
        )
        .arg(
            Arg::new("INPUT")
                .required(true)
                .takes_value(true)
                .about("The Brainfuck file to interpret."),
        )
        .get_matches();

    // Safety: unwrapping this value is safe because
    // it was marked as required.
    let input_file = matches.value_of("INPUT").unwrap();

    match parse_bf_file(input_file) {
        Ok(mut instructions) => {
            let read_source = if let Some(read_file) = matches.value_of("read") {
                match File::open(read_file) {
                    Ok(f) => ReadSource::File(f),
                    Err(err) => {
                        println!("User-input file source does not exist: {}", err.to_string());
                        return;
                    }
                }
            } else {
                ReadSource::StdIn
            };

            if matches.occurrences_of("optimize") > 0 {
                instructions = optimize_instructions(instructions);
            }

            Interpreter::new(instructions, read_source).run();
        }
        Err(err) => println!("Failed to parse Brainfuck file: {}", err),
    }
}
