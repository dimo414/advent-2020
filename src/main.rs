// https://github.com/rust-lang/cargo/issues/3591#issuecomment-475701083
//#![ allow( dead_code, unused_imports, unused_macros, unused_variables ) ]
#[macro_use] extern crate lazy_static;
extern crate parameterized_test;
extern crate regex;
extern crate anyhow;

use std::env;

#[macro_use] mod console;
#[macro_use] mod parsing;
mod euclid;
mod machine;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;

fn main() {
    let _console = console::Console::init();
    println!(); // split build output from runtime output
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} DAY_OF_ADVENT", args[0]);
        return;
    }
    let day: u32 = args[1].parse().expect("Should be a natural number");
    match day {
        1 => aoc01::advent(),
        2 => aoc02::advent(),
        3 => aoc03::advent(),
        4 => aoc04::advent(),
        5 => aoc05::advent(),
        6 => aoc06::advent(),
        7 => aoc07::advent(),
        8 => aoc08::advent(),
        9 => aoc09::advent(),
        10 => aoc10::advent(),
        11 => aoc11::advent(),
        12 => aoc12::advent(),
        13 => aoc13::advent(),
        14 => aoc14::advent(),
        15 => aoc15::advent(),
        16 => aoc16::advent(),
        17 => aoc17::advent(),
        18 => aoc18::advent(),
        x => {
            eprintln!("Day {} hasn't happened yet.", x);
            ::std::process::exit(1);
        },
    }
}
