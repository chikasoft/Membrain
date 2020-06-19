use std::fs::File;
use std::hint;
use std::io::{BufRead, BufReader};

use crate::interpreter::Instruction;

pub fn parse_bf_file(filename: &str) -> Result<Vec<Instruction>, String> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(err) => {
            return Err(format!(
                "Failed to open Brainfuck file: {}",
                err.to_string()
            ))
        }
    };

    let mut jump_stack = Vec::new();
    let mut instructions = Vec::new();

    for (line_number, opt_line) in BufReader::new(file).lines().enumerate() {
        let line = match opt_line {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "Failed while reading Brainfuck file: {}",
                    err.to_string()
                ))
            }
        };

        for (char_number, c) in line.chars().enumerate() {
            match c {
                '>' => instructions.push(Instruction::MoveRight),
                '<' => instructions.push(Instruction::MoveLeft),
                '+' => instructions.push(Instruction::Increment),
                '-' => instructions.push(Instruction::Decrement),
                '.' => instructions.push(Instruction::Write),
                ',' => instructions.push(Instruction::Read),
                '[' => {
                    instructions.push(Instruction::JumpIfZero(0));

                    let jump_if_zero_pos = instructions.len() - 1;
                    jump_stack.push(jump_if_zero_pos);
                }
                ']' => {
                    match jump_stack.pop() {
                        Some(matching) => {
                            instructions.push(Instruction::JumpUnlessZero(matching));

                            let jump_unless_zero_pos = instructions.len() - 1;

                            // Safety: matching is always in bounds by construction;
                            // the case for '[' ensures 0 <= matching < instructions.len().
                            match unsafe { instructions.get_unchecked_mut(matching) } {
                                Instruction::JumpIfZero(ref mut matching) => {
                                    *matching = jump_unless_zero_pos;
                                }
                                _ => unsafe {
                                    // Safety: The only instruction that pushes to
                                    // the jump_stack is JumpIfZero; this case is
                                    // unreachable.
                                    hint::unreachable_unchecked();
                                },
                            }
                        }
                        None => {
                            return Err(format!("No matching open-bracket ('[') for bracket on line {}, character {}.", line_number, char_number));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(instructions)
}
