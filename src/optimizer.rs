use crate::interpreter::Instruction;

// TODO: These optimizations are very trivial
// and don't cover even the most basic deviations.
// Converting instructions into an intermediate representation
// would allow for more and better optimizations.
//
// Also, the interpreter could be made to interpret this
// intermediate representation, thereby making even unoptimized
// code faster to interpret.
pub fn optimize_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut opt_instructions = Vec::with_capacity(instructions.len());
    let mut peekable = instructions.into_iter().peekable();

    while let Some(instruction) = peekable.next() {
        if let Some(&next_instruction) = peekable.peek() {
            match instruction {
                Instruction::MoveRight => {
                    // Opt: >< is a no-op so skip them.
                    if next_instruction == Instruction::MoveLeft {
                        peekable.next().unwrap();
                        continue;
                    }
                }
                Instruction::MoveLeft => {
                    // Opt: <> is a no-op so skip them.
                    if next_instruction == Instruction::MoveRight {
                        peekable.next().unwrap();
                        continue;
                    }
                }
                Instruction::Increment => {
                    // Opt: +- is a no-op so skip them.
                    if next_instruction == Instruction::Decrement {
                        peekable.next().unwrap();
                        continue;
                    }
                }
                Instruction::Decrement => {
                    // Opt: -+ is a no-op so skip them.
                    if next_instruction == Instruction::Increment {
                        peekable.next().unwrap();
                        continue;
                    }
                }
                _ => {}
            }
        }

        opt_instructions.push(instruction);
    }

    opt_instructions
}
