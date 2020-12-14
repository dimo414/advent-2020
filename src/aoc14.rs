use crate::parsing::{regex_captures, capture_group};
use std::collections::HashMap;
use std::str::FromStr;
use anyhow::{Error, Result, bail};

pub fn advent() {
    let instructions = parse_data().unwrap();
    println!("Memory sum using V1 masks: {}", run_v1(&instructions).values().sum::<i64>());
    println!("Memory sum using V2 masks: {}", run_v2(&instructions).values().sum::<i64>());
}

#[derive(Copy, Clone)]
struct Mask(i64,i64,i64);

impl Mask {
    fn apply_v1(&self, value: i64) -> i64 {
        (value | self.0) & !self.1
    }

    fn apply_v2(&self, addr: i64) -> Vec<i64> {
        let addr = addr | self.0;
        let mut addrs = vec!(addr);
        for i in 0..36 {
            let bit = 2_i64.pow(i);
            if self.2 & bit != 0 {
                for old in addrs.drain(..).collect::<Vec<_>>() {
                    addrs.push(old | bit);
                    addrs.push(old & !bit);
                }
            }
        }
        addrs
    }
}

impl FromStr for Mask {
    type Err = Error;
    fn from_str(mask: &str) -> Result<Self> {
        let (mut ones, mut zeros, mut floats) = (0, 0, 0);
        for c in mask.chars() {
            ones *= 2;
            zeros *= 2;
            floats *= 2;
            match c {
                '0' => zeros += 1,
                '1' => ones += 1,
                'X' => floats += 1,
                _ => bail!("Invalid mask: {}", mask),
            }
        }
        Ok(Mask(ones, zeros, floats))
    }
}

enum Instruction {
    Mask(Mask),
    Memory(i64, i64),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self> {
        if line.starts_with("mask = ") {
            Ok(Instruction::Mask(line[7..].parse()?))
        } else {
            let regex = static_regex!(r"mem\[(\d+)\] = (\d+)");
            let caps = regex_captures(regex, &line)?;
            let addr = capture_group(&caps, 1).parse::<i64>()?;
            let value = capture_group(&caps, 2).parse::<i64>()?;
            Ok(Instruction::Memory(addr, value))
        }
    }
}

fn run_v1(instructions: &[Instruction]) -> HashMap<i64, i64> {
    let mut mask = None;
    let mut memory = HashMap::new();
    for instr in instructions.iter() {
        match instr {
            Instruction::Mask(m) => mask = Some(*m),
            Instruction::Memory(addr, value) => {
                memory.insert(*addr, mask.expect("Mask not yet set").apply_v1(*value));
            },
        }
    }
    memory
}

fn run_v2(instructions: &[Instruction]) -> HashMap<i64, i64> {
    let mut mask = None;
    let mut memory = HashMap::new();
    for instr in instructions.iter() {
        match instr {
            Instruction::Mask(m) => mask = Some(*m),
            Instruction::Memory(addr, value) => {
                for masked_addr in mask.expect("Mask not yet set").apply_v2(*addr) {
                    memory.insert(masked_addr, *value);
                }
            },
        }
    }
    memory
}

fn parse_data() -> Result<Vec<Instruction>> {
    include_str!("../data/day14.txt").trim().split("\n").map(|s|s.parse()).collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1() {
        let instr = vec!(
            Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap()),
            Instruction::Memory(8, 11), Instruction::Memory(7, 101), Instruction::Memory(8, 0));
        assert_eq!(run_v1(&instr), vec!((7, 101), (8, 64)).into_iter().collect());
    }

    #[test]
    fn v2() {
        let instr = vec!(
            Instruction::Mask("".parse().unwrap()),
            Instruction::Memory(42, 100),
            Instruction::Mask("".parse().unwrap()),
            Instruction::Memory(26, 1),
        );
        assert_eq!(run_v2(&instr), vec!((26, 1), (42, 100)).into_iter().collect());
    }

    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}
