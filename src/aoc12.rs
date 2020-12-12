use crate::euclid::{point, vector, Vector, Point};
use std::str::FromStr;
use anyhow::{Error, bail, Context, Result};
use std::fmt;

pub fn advent() {
    let path = parse_data();
    println!("Direct destination: {}", (move_direct(&path)-Point::ORIGIN).grid_len());
    println!("Waypoint destination: {}", (move_relative(&path)-Point::ORIGIN).grid_len());
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Move {
    instruction: Instruction,
    length: i32,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.instruction, self.length)
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(entry: &str) -> Result<Self> {
        let letter = entry.chars().next().context(format!("letter:{}", entry))?;
        let length: i32 = entry.chars().skip(1).collect::<String>().parse().context(format!("len:{}", entry))?;
        let instruction = match letter {
            'N' => Instruction::North,
            'S' => Instruction::South,
            'E' => Instruction::East,
            'W' => Instruction::West,
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            'F' => Instruction::Forward,
            _ => bail!("Invalid letter {}", letter),
        };
        Ok(Move{instruction, length})
    }
}

// https://math.stackexchange.com/q/1330161/1887
fn rotate(vec: Vector, degrees: i32) -> Vector {
    assert_eq!(degrees % 90, 0);
    let normalized = ((degrees % 360) + 360) % 360;
    match normalized {
        0 => vec,
        90 => vector(-vec.y, vec.x),
        180 => vector(-vec.x, -vec.y),
        270 => vector(vec.y, -vec.x),
        _ => panic!(),
    }
}

fn move_direct(path: &[Move]) -> Point {
    let mut pos = point(0, 0);
    let mut dir = vector(1, 0);
    for mv in path.iter() {
        match mv.instruction {
            Instruction::North => pos += vector(0, -1) * mv.length,
            Instruction::South => pos += vector(0, 1) * mv.length,
            Instruction::East => pos += vector(1, 0) * mv.length,
            Instruction::West => pos += vector(-1, 0) * mv.length,
            Instruction::Left => dir = rotate(dir, -mv.length),
            Instruction::Right => dir = rotate(dir, mv.length),
            Instruction::Forward => pos += dir * mv.length,
        }
    }
    pos
}

fn move_relative(path: &[Move]) -> Point {
    let mut pos = point(0, 0);
    let mut waypoint = point(10, -1);
    for mv in path.iter() {
        match mv.instruction {
            Instruction::North => waypoint += vector(0, -1) * mv.length,
            Instruction::South => waypoint += vector(0, 1) * mv.length,
            Instruction::East => waypoint += vector(1, 0) * mv.length,
            Instruction::West => waypoint += vector(-1, 0) * mv.length,
            Instruction::Left => waypoint = pos + rotate(waypoint-pos, -mv.length),
            Instruction::Right => waypoint = pos + rotate(waypoint-pos, mv.length),
            Instruction::Forward => {
                let dest = waypoint-pos;
                pos += dest * mv.length;
                waypoint = pos + dest;
            },
        }
    }
    pos
}

fn parse_data() -> Vec<Move> {
    include_str!("../data/day12.txt").trim().split("\n").map(|m| m.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static!{
        static ref EXAMPLE: Vec<Move> = vec!("F10","N3","F7","R90","F11").iter().map(|m| m.parse().unwrap()).collect();
    }

    #[test]
    fn direct() {
        assert_eq!(move_direct(&EXAMPLE), point(17, 8));
    }

    #[test]
    fn relative() {
        assert_eq!(move_relative(&EXAMPLE), point(214, 72));
    }

    #[test]
    fn parse_file() {
        parse_data();
    }
}
