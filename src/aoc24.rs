use std::cmp::min;
use std::collections::{HashMap, HashSet};
use anyhow::Result;

pub fn advent() {
    let moves = read_data().unwrap();
    let mut tiles = identify_tiles(&moves);
    println!("Initial Black Tiles: {}", tiles.len());

    for _ in 0..100 {
        tiles = day_passes(&tiles);
    }
    println!("Black Tiles after 100 days: {}", tiles.len());
}

fn identify_tiles(moves: &Vec<Vec<Move>>) -> HashSet<HexPoint> {
    let mut tiles = HashMap::new();
    for mv in moves.iter() {
        *tiles.entry(HexPoint::create(0, 0, 0).steps(mv)).or_insert(0) += 1;
    }
    tiles.iter().filter_map(|(&t, c)| if c % 2 == 1 { Some(t) } else { None }).collect()
}

fn day_passes(black_tiles: &HashSet<HexPoint>) -> HashSet<HexPoint> {
    fn count(black_tiles: &HashSet<HexPoint>, candidates: &Vec<HexPoint>) -> usize {
        candidates.iter().filter(|c| black_tiles.contains(c)).count()
    }

    let mut ret = HashSet::new();
    // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    for black_tile in black_tiles.iter().filter(|t| {
        let c = count(black_tiles, &t.adjacent());
        c == 1 || c == 2
    }) {
        ret.insert(*black_tile);
    }

    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    for white_tile in black_tiles.iter().flat_map(|t| t.adjacent()).filter(|t| !black_tiles.contains(t)) {
        if count(black_tiles, &white_tile.adjacent()) == 2 {
            ret.insert(white_tile);
        }
    }

    ret
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Move {
    EAST,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
    NORTHWEST,
    NORTHEAST,
}

fn to_moves(path: &str) -> Result<Vec<Move>> {
    let mut ret = Vec::new();
    let mut chars = path.chars();
    while let Some(dir) = chars.next() {
        let mv = match dir {
            'e' => Move::EAST,
            'w' => Move::WEST,
            's' => {
                match chars.next() {
                    Some('e') => Move::SOUTHEAST,
                    Some('w') => Move::SOUTHWEST,
                    c => anyhow::bail!("Unexpected {:?} after s", c),
                }
            },
            'n' => {
                match chars.next() {
                    Some('e') => Move::NORTHEAST,
                    Some('w') => Move::NORTHWEST,
                    c => anyhow::bail!("Unexpected {:?} after n", c),
                }
            },
            _ => anyhow::bail!("Unexpected char: '{}'", dir),
        };
        ret.push(mv);
    }
    Ok(ret)
}

// TODO probably worth pulling this type out
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct HexPoint {
    run: i32,
    run_up: i32,
    run_down: i32,
}

impl HexPoint {
    fn create(run: i32, run_up: i32, run_down: i32) -> HexPoint {
        let mut point = HexPoint{run, run_up, run_down};
        point.normalize();
        point
    }

    // forked from 2017 day 11
    fn normalize(&mut self) {
        // if sign(run_up) == sign(run_down) subtract from both and add to run
        if self.run_up.signum() == self.run_down.signum() {
            let run = min(self.run_up.abs(), self.run_down.abs()) * self.run_up.signum();
            self.run_up -= run;
            self.run_down -= run;
            self.run += run;
        }

        // if sign(run) != sign(run_up) subtract from both and add to run_down
        if self.run_up.signum() != self.run.signum() {
            let run_down = min(self.run_up.abs(), self.run.abs()) * self.run.signum();
            self.run -= run_down;
            self.run_up += run_down;   // actually a subtraction, since we know left is the opposite sign
            self.run_down += run_down;
        }

        // and same for run_up
        if self.run_down.signum() != self.run.signum() {
            let run_up = min(self.run_down.abs(), self.run.abs()) * self.run.signum();
            self.run -= run_up;
            self.run_down += run_up;   // actually a subtraction, since we know left is the opposite sign
            self.run_up += run_up;
        }
    }

    fn step(&self, mv: &Move) -> HexPoint {
        match mv {
            Move::EAST => HexPoint::create(self.run+1, self.run_up, self.run_down),
            Move::SOUTHEAST => HexPoint::create(self.run, self.run_up, self.run_down+1),
            Move::SOUTHWEST => HexPoint::create(self.run, self.run_up-1, self.run_down),
            Move::WEST => HexPoint::create(self.run-1, self.run_up, self.run_down),
            Move::NORTHWEST => HexPoint::create(self.run, self.run_up, self.run_down-1),
            Move::NORTHEAST => HexPoint::create(self.run, self.run_up+1, self.run_down),
        }
    }

    fn steps(&self, moves: &[Move]) -> HexPoint {
        let mut pos = *self;
        for mv in moves {
            pos = pos.step(mv);
        }
        pos
    }

    fn adjacent(&self) -> Vec<HexPoint> {
        vec!(
            HexPoint::create(self.run+1, self.run_up, self.run_down),
            HexPoint::create(self.run-1, self.run_up, self.run_down),
            HexPoint::create(self.run, self.run_up+1, self.run_down),
            HexPoint::create(self.run, self.run_up-1, self.run_down),
            HexPoint::create(self.run, self.run_up, self.run_down+1),
            HexPoint::create(self.run, self.run_up, self.run_down-1),
        )
    }
}

fn read_data() -> Result<Vec<Vec<Move>>> {
    include_str!("../data/day24.txt").trim().split("\n").map(|s| to_moves(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_example() -> Result<Vec<Vec<Move>>> {
        include_str!("../data/day24_example.txt").trim().split("\n").map(|s| to_moves(s)).collect()
    }

    parameterized_test::create!{ move_examples, (moves, expected), {
        let moves = to_moves(moves).unwrap();
        let dest = HexPoint::create(0,0,0).steps(&moves);
        assert_eq!(dest, expected);
        }}
    move_examples! {
        a: ("esew", HexPoint::create(0, 0, 1)),
        b: ("nwwswee", HexPoint::create(0, 0, 0)),
    }

    #[test]
    fn example() {
        let mut tiles = identify_tiles(&read_example().unwrap());
        assert_eq!(tiles.len(), 10);

        let expected_days: HashMap<_, _> = vec!(
            (1, 15), (2, 12), (3, 25), (4, 14), (5, 23), (6, 28), (7, 41), (8, 37), (9, 49), (10, 37),
            (20, 132), (30, 259), (40, 406), (50, 566), (60, 788), (70, 1106), (80, 1373), (90, 1844), (100, 2208)
        ).into_iter().collect();

        for day in 1..=100 {
            tiles = day_passes(&tiles);
            if let Some(count) = expected_days.get(&day) {
                assert_eq!(tiles.len(), *count);
            }
        }
    }

    #[test]
    fn parse_file() {
        read_data().unwrap();
    }
}
