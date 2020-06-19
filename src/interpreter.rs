#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    JumpIfZero(usize),
    JumpUnlessZero(usize),
}

pub struct Interpreter {
    pc: usize,
    pointer: usize,
    tape: Vec<u8>,
    instructions: Vec<Instruction>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Interpreter {
            pc: 0,
            pointer: 0,
            tape: vec![0; 30_000],
            instructions,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.instructions.len() {
            // Safety: The condition for the loop above guarantees
            // self.pc is within the bounds of self.instructions.
            match unsafe { self.instructions.get_unchecked(self.pc) } {
                Instruction::MoveRight => {
                    self.pointer += 1;

                    if self.pointer >= self.tape.len() {
                        self.tape.push(0);
                    }
                }
                Instruction::MoveLeft => {
                    if self.pointer > 0 {
                        self.pointer -= 1;
                    } else {
                        self.tape.insert(0, 0);
                    }
                }
                Instruction::Increment => {
                    let cell = self.get_current_cell_mut();
                    *cell = cell.wrapping_add(1);
                }
                Instruction::Decrement => {
                    let cell = self.get_current_cell_mut();
                    *cell = cell.wrapping_sub(1);
                }
                Instruction::Write => {
                    let cell = self.get_current_cell_value();
                    print!("{}", cell as char);
                }
                Instruction::Read => {
                    use std::io::{stdin, Read};

                    let cell = self.get_current_cell_mut();
                    let mut input = [0u8];

                    *cell = match stdin().read_exact(&mut input) {
                        Ok(_) => input[0],
                        Err(_) => 0,
                    };
                }
                Instruction::JumpIfZero(matching) => {
                    let cell = self.get_current_cell_value();

                    if cell == 0 {
                        self.pc = *matching;
                    }
                }
                Instruction::JumpUnlessZero(matching) => {
                    let cell = self.get_current_cell_value();

                    if cell != 0 {
                        self.pc = *matching;
                    }
                }
            }

            self.pc += 1;
        }
    }

    #[inline]
    fn get_current_cell_value(&self) -> u8 {
        // Safety: self.pointer is always in bounds:
        // the move instructions perform safety checks,
        // and self.tape is initialized with at least
        // one cell.
        unsafe { *self.tape.get_unchecked(self.pointer) }
    }

    #[inline]
    fn get_current_cell_mut(&mut self) -> &mut u8 {
        // Safety: self.pointer is always in bounds:
        // the move instructions perform safety checks,
        // and self.tape is initialized with at least
        // one cell.
        unsafe { self.tape.get_unchecked_mut(self.pointer) }
    }
}
