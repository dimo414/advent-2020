use crate::machine::{Instruction, Program, Machine};

pub fn advent() {
    let program = parse_data().unwrap();
    let mut machine = Machine::new();
    assert!(!machine.run_until_complete(&program));
    println!("Machine looped after setting accumulator to {}", machine.accumulator());

    let (i, acc) = find_completable_program(&program);
    println!("Machine completed after flipping command {} with accumulator set to {}", i, acc);
}

fn swap_jmp_nop(instr: &Instruction) -> Instruction {
    match instr {
        Instruction::ACC(_) => *instr,
        Instruction::JMP(value) => Instruction::NOP(*value),
        Instruction::NOP(value) => Instruction::JMP(*value),
    }
}

fn find_completable_program(program: &Program) -> (usize, i32) {
    for i in 0..program.commands.len() {
        let mut machine = Machine::new();
        let mut program = program.clone();
        program.commands[i] = swap_jmp_nop(&program.commands[i]);
        if machine.run_until_complete(&program) {
            return (i, machine.accumulator());
        }
    }
    panic!("No valid programs.");
}

fn parse_data() -> anyhow::Result<Program> {
    include_str!("../data/day08.txt").parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example() -> anyhow::Result<Program> {
        include_str!("../data/day08_example.txt").parse()
    }

    #[test]
    fn find_loop() {
        let program = parse_example().unwrap();
        let mut machine = Machine::new();
        assert!(!machine.run_until_complete(&program));
        assert_eq!(machine.accumulator(), 5);
    }

    #[test]
    fn fix_program() {
        let program = parse_example().unwrap();
        let (i, acc) = find_completable_program(&program);
        assert_eq!(i, 7);
        assert_eq!(acc, 8);
    }

    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}
