use std::collections::HashSet;

pub fn advent() {
    let points = parse_data();
    println!("3D space: {}", Simulator3D{}.cycles(6, &points).len());
    println!("4D space: {}", Simulator4D{}.cycles(6, &points).len());
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[inline]
pub const fn point(x: i32, y: i32, z: i32, w: i32) -> Point {
    Point { x, y, z, w }
}

trait Simulator {
    fn neighbors(&self, pos: Point) -> HashSet<Point>;

    fn round(&self, points: &HashSet<Point>) -> HashSet<Point> {
        let mut ret = HashSet::new();
        let candidates: HashSet<_> = points.iter().flat_map(|&p| self.neighbors(p)).collect();
        for candidate in candidates {
            let neig = self.neighbors(candidate).intersection(&points).count();
            if neig == 3 {
                ret.insert(candidate);
            } else if neig == 2 && points.contains(&candidate) {
                ret.insert(candidate);
            }
        }
        ret
    }

    fn cycles(&self, rounds: u32, points: &HashSet<Point>) -> HashSet<Point> {
        let mut points = points.clone();
        for _ in 0..rounds {
            points = self.round(&points);
        }
        points
    }
}

struct Simulator3D{}
impl Simulator for Simulator3D {
    fn neighbors(&self, pos: Point) -> HashSet<Point> {
        let mut ret = HashSet::new();
        for x in pos.x-1..pos.x+2 {
            for y in pos.y-1..pos.y+2 {
                for z in pos.z-1..pos.z+2 {
                    ret.insert(point(x, y, z, 0));
                }
            }
        }
        ret.remove(&pos);
        ret
    }
}

struct Simulator4D{}
impl Simulator for Simulator4D {
    fn neighbors(&self, pos: Point) -> HashSet<Point> {
        let mut ret = HashSet::new();
        for x in pos.x-1..pos.x+2 {
            for y in pos.y-1..pos.y+2 {
                for z in pos.z-1..pos.z+2 {
                    for w in pos.w-1..pos.w+2 {
                        ret.insert(point(x, y, z, w));
                    }
                }
            }
        }
        ret.remove(&pos);
        ret
    }
}

fn to_set(str: &str) -> HashSet<Point> {
    let mut ret = HashSet::new();
    for (y, line) in str.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { ret.insert(point(x as i32, y as i32, 0, 0)); },
                '.' => {},
                _ => panic!(),
            }
        }
    }
    ret
}

fn parse_data() -> HashSet<Point> {
    to_set(include_str!("../data/day17.txt").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = ".#.\n..#\n###";

    #[test]
    fn example3d() {
        let points = to_set(EXAMPLE);
        assert_eq!(Simulator3D{}.cycles(6, &points).len(), 112);
    }

    #[test]
    fn example4d() {
        let points = to_set(EXAMPLE);
        assert_eq!(Simulator4D{}.cycles(6, &points).len(), 848);
    }
}
