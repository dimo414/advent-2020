use regex::Regex;
use std::str::FromStr;
use crate::error::ParseError;

pub fn advent() {
    let data = parse_data();
    println!("Valid Passwords: {}", data.iter().filter(|e| e.nums_as_range()).count());
    println!("Valid Passwords: {}", data.iter().filter(|e| e.nums_as_positions()).count());
}

#[derive(Debug)]
struct Entry {
    nums: (i32, i32),
    letter: char,
    password: String,
}

impl Entry {
    fn nums_as_range(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.letter).count() as i32;
        count >= self.nums.0 && count <= self.nums.1
    }

    fn nums_as_positions(&self) -> bool {
        let match_a = self.password.chars().nth(self.nums.0 as usize - 1).unwrap() == self.letter;
        let match_b = self.password.chars().nth(self.nums.1 as usize - 1).unwrap() == self.letter;
        (match_a || match_b) && !(match_a && match_b) // XOR
    }
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
        }

        let caps = regex_captures!(RE, s)?;
        let nums: (i32, i32) = (capture_group!(caps, 1).parse()?, capture_group!(caps, 2).parse()?);
        let letter = capture_group!(caps, 3).chars().next().unwrap();
        let password = capture_group!(caps, 4).to_string();
        return Ok(Entry{nums, letter, password});
    }
}

fn parse_data() -> Vec<Entry> {
    return include_str!("../data/day02.txt").lines().map(|l| l.parse::<Entry>().unwrap()).collect();
}



#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{examples, (entry, range, pos), {
      assert_eq!(entry.nums_as_range(), range, "nums_as_range");
      assert_eq!(entry.nums_as_positions(), pos, "nums_as_positions");
    }}
    examples!{
      a: (Entry{nums: (1, 3), letter: 'a', password: "abcde".into()}, true, true),
      b: (Entry{nums: (1, 3), letter: 'b', password: "cdefg".into()}, false, false),
      c: (Entry{nums: (2, 9), letter: 'c', password: "ccccccccc".into()}, true, false),
    }

    #[test]
    fn parse_file() {
        assert!(parse_data().len() > 0);
    }
}
