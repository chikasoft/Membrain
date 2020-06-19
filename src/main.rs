extern crate clap;

use clap::{App, Arg};

mod interpreter;
mod language;
mod optimizer;

use interpreter::Interpreter;
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
            if matches.occurrences_of("optimize") > 0 {
                instructions = optimize_instructions(instructions);
            }

            Interpreter::new(instructions).run();
        }
        Err(err) => println!("Failed to parse Brainfuck file: {}", err),
    }
}
