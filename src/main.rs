// https://github.com/rust-lang/cargo/issues/3591#issuecomment-475701083
//#![ allow( dead_code, unused_imports, unused_macros, unused_variables ) ]
#[macro_use] extern crate lazy_static;
extern crate parameterized_test;
extern crate regex;

use std::env;

macro_rules! regex_captures {
  ($re:tt, $s:expr) => {
    $re.captures($s).ok_or_else(|| format!("`{}` did not match `{}`", $s, $re.as_str()))
  };
}

macro_rules! capture_group {
  ($caps:expr, $group:expr) => { $caps.get($group).expect("valid capture group").as_str() };
}

#[macro_use] mod console;
mod error;
mod euclid;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;

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
        x => {
            eprintln!("Day {} hasn't happened yet.", x);
            ::std::process::exit(1);
        },
    }
}
