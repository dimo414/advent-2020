use crate::parsing;
use anyhow::{bail, Error, Result};
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = static_regex!(r"^([^ ]+) ([^ ]+)$");
        let caps = parsing::regex_captures(&regex, s)?;
        let opccode = parsing::capture_group(&caps, 1);
        let arg: i32 = parsing::capture_group(&caps, 2).parse()?;
        let instr = match opccode {
            "acc" => Instruction::ACC(arg),
            "jmp" => Instruction::JMP(arg),
            "nop" => Instruction::NOP(arg),
            _ => bail!("Unkown opcode for instruction: {}", s),
        };
        Ok(instr)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub commands: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let commands: Vec<_> = s.trim().split("\n").map(|i| i.parse::<Instruction>()).collect::<Result<_>>()?;
        Ok(Program{commands})
    }
}

pub struct Machine {
    acc: i32,
}

impl Machine {
    pub fn new() -> Machine {
        Machine{acc:0}
    }

    pub fn accumulator(&self) -> i32 {
        self.acc
    }

    pub fn run_until_complete(&mut self, program: &Program) -> bool {
        let mut counter: i32 = 0;
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(counter) { return false; }
            if counter == program.commands.len() as i32 { return true; }
            let command = &program.commands[counter as usize];
            match command {
                Instruction::ACC(value) => self.acc += value,
                Instruction::JMP(value) => counter += value -1,
                Instruction::NOP(_) => {},
            }
            counter = counter+1; // TODO handle overflow?
        }
    }
}