use std::str::FromStr;
use anyhow::{Error, Result};
use crate::euclid::{Point,Vector,point,vector};

pub fn advent() {
    let landscape = parse_data();
    let slope = vector(3, 1);
    println!("Traversed via {} and hit {} trees", slope, landscape.traverse(slope));

    let slopes = vec!(vector(1, 1), vector(3, 1), vector(5, 1), vector(7, 1), vector(1, 2));
    let tree_product: i64 = landscape.traverse_multi(slopes).iter().product();
    println!("Product of trees: {}", tree_product);
}

#[derive(Debug)]
struct Landscape {
    trees: Vec<Vec<bool>>,
}

impl Landscape {
    fn check(&self, loc: Point) -> bool {
        assert!(loc.y >= 0 && loc.y < self.trees.len() as i32);
        let row = &self.trees[loc.y as usize];
        row[loc.x as usize % row.len()]
    }

    fn traverse(&self, slope: Vector) -> i64 {
        let mut pos = point(0, 0);
        let mut count = 0;
        while pos.y < self.trees.len() as i32 {
            if self.check(pos) { count += 1; }
            pos += slope;
        }
        count
    }

    fn traverse_multi(&self, slopes: Vec<Vector>) -> Vec<i64> {
        slopes.iter().map(|s| self.traverse(*s)).collect()
    }
}

impl FromStr for Landscape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut trees = Vec::new();
        for line in s.lines() {
            trees.push(line.chars().map(|c| c == '#').collect());
        }
        Ok(Landscape{trees})
    }
}

fn parse_data() -> Landscape {
    include_str!("../data/day03.txt").parse::<Landscape>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example() -> Landscape {
        include_str!("../data/day03_example.txt").parse::<Landscape>().unwrap()
    }

    parameterized_test::create!{slopes, (slope, trees), {
      let landscape = parse_example();
      assert_eq!(landscape.traverse(slope), trees);
    }}
    slopes!{
      a: (vector(1, 1), 2),
      b: (vector(3, 1), 7),
      c: (vector(5, 1), 3),
      d: (vector(7, 1), 4),
      e: (vector(1, 2), 2),
    }

    #[test]
    fn multiply() {
        let landscape = parse_example();
        let slopes = vec!(vector(1, 1), vector(3, 1), vector(5, 1), vector(7, 1), vector(1, 2));
        assert_eq!(landscape.traverse_multi(slopes), vec!(2, 7, 3, 4, 2));
    }

    #[test]
    fn parse_file() {
        parse_example();
        parse_data();
    }
}
