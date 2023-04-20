use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    pub filename: PathBuf,
}

struct Brainfuck {
    cell_index: usize,
    cells: Vec<u8>,

    loops: Vec<usize>, // ? If a ']' is encountered, set the instruction pointer to the last value in loops Vec

    instructions: Vec<u8>,
    instruction_index: usize,
}

impl Brainfuck {
    pub fn new(source: String) -> Brainfuck {
        let instructions = Vec::from(source);

        Brainfuck {
            cell_index: 0,
            cells: vec![0],
            loops: vec![],
            instructions,
            instruction_index: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut output = String::new();

        loop {
            if self.instruction_index >= self.instructions.len() {
                break;
            }

            let instruction = self.instructions[self.instruction_index];

            match instruction {
                b'>' => {
                    if self.cell_index == self.cells.len() - 1 {
                        self.cells.push(0);
                    }

                    self.cell_index += 1;
                }
                b'<' => {
                    if self.cell_index == 0 {
                        return Err(anyhow::anyhow!(
                            "Cannot go below starting cell Instruction {}: {}",
                            self.instruction_index,
                            self.instructions[self.instruction_index] as char
                        ));
                    }

                    self.cell_index -= 1;
                }
                b'+' => {
                    self.cells[self.cell_index] = self.cells[self.cell_index].wrapping_add(1);
                }
                b'-' => self.cells[self.cell_index] = self.cells[self.cell_index].wrapping_sub(1),
                b'[' => {
                    self.loops.push(self.instruction_index);
                }
                b']' => {
                    if self.cells[self.cell_index] != 0 {
                        if let Some(loop_start) = self.loops.last() {
                            self.instruction_index = *loop_start;
                        } else {
                            return Err(anyhow::anyhow!(
                                "No loop found {}: {}",
                                self.instruction_index,
                                self.instructions[self.instruction_index] as char
                            ));
                        }
                    }
                }
                b'.' => {
                    output.push(self.cells[self.cell_index] as char);
                }
                b',' | b'#' => todo!("{}", instruction),
                _ => {
                    // No-op
                }
            };

            self.instruction_index += 1;
        }

        print!("{output}");

        Ok(())
    }
}

fn main() -> Result<()> {
    let comman = Command::parse();

    let filename = comman.filename;

    let source = std::fs::read_to_string(filename)?;

    let mut brainfuck = Brainfuck::new(source);

    brainfuck.run()
}
