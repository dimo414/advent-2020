use std::collections::HashMap;
use crate::euclid::{Point,point,vector,Vector};

pub fn advent() {
    let floorplan = parse_data();
    println!("Occupied seats with adjacency: {}",
             count_occupied(&find_stable(&floorplan, &Adjacent{})));
    println!("Occupied seats with visibility: {}",
             count_occupied(&find_stable(&floorplan, &Visible{})));
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Empty,
    Occupied,
    Floor,
}

trait Strategy {
    fn count_nearby(&self, floor: &HashMap<Point, State>, pos: Point) -> u32;
    fn next_state(&self, state: State, count: u32) -> State;
}

struct Adjacent{}
impl Strategy for Adjacent {
    fn count_nearby(&self, floor: &HashMap<Point, State>, pos: Point) -> u32 {
        let mut sum = 0;
        for dir in Vector::ORDINAL {
            if let Some(s) = floor.get(&(pos+dir)) {
                if *s == State::Occupied { sum += 1; }
            }
        }
        sum
    }

    fn next_state(&self, state: State, count: u32) -> State {
        match state {
            State::Empty => if count == 0 { return State::Occupied; },
            State::Occupied => if count >= 4 { return State::Empty; },
            _ => {},
        }
        state
    }
}

struct Visible{}
impl Strategy for Visible {
    fn count_nearby(&self, floor: &HashMap<Point, State>, pos: Point) -> u32 {
        let mut sum = 0;
        for dir in Vector::ORDINAL {
            let mut looking = pos;
            loop {
                looking += dir;
                if let Some(s) = floor.get(&looking) {
                    match s {
                        State::Occupied => { sum += 1; break; },
                        State::Empty => { break; },
                        _ => {},
                    }
                } else { break; }
            }
        }
        sum
    }

    fn next_state(&self, state: State, count: u32) -> State {
        match state {
            State::Empty => if count == 0 { return State::Occupied; },
            State::Occupied => if count >= 5 { return State::Empty; },
            _ => {},
        }
        state
    }
}

fn count_occupied(floor: &HashMap<Point, State>) -> usize {
    floor.values().filter(|s| **s==State::Occupied).count()
}

fn find_stable(floor: &HashMap<Point, State>, strat: &dyn Strategy) -> HashMap<Point, State> {
    let mut last = floor.clone();
    loop {
        let next = iteration(&last, strat);
        if next == last { break; }
        last = next;
    }
    last
}

fn iteration(floor: &HashMap<Point, State>, strat: &dyn Strategy) -> HashMap<Point, State> {
    let mut next = HashMap::new();
    for pos in Point::display_order_box(floor.keys().cloned()).unwrap() {
        if let Some(s) = floor.get(&pos) {
            let count = strat.count_nearby(floor, pos);
            next.insert(pos, strat.next_state(*s, count));
        }
    }
    next
}

fn build_map(str: &str) -> HashMap<Point, State> {
    let rows: Vec<_> = str.trim().split("\n").collect();
    let mut ret = HashMap::new();
    let mut pos = point(0, 0);
    for row in rows {
        for col in row.chars() {
            let state = match col {
                'L' => State::Empty,
                '#' => State::Occupied,
                '.' => State::Floor,
                _ => panic!(),
            };
            ret.insert(pos, state);
            pos += vector(1, 0);
        }
        pos = point(0, pos.y+1);
    }
    ret
}

fn parse_data() -> HashMap<Point, State> {
    build_map(include_str!("../data/day11.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example() -> HashMap<Point, State> {
        build_map(include_str!("../data/day11_example.txt"))
    }

    #[test]
    fn adjacent() {
        assert_eq!(count_occupied(&find_stable(&parse_example(), &Adjacent{})), 37);
    }

    #[test]
    fn visible() {
        assert_eq!(count_occupied(&find_stable(&parse_example(), &Visible{})), 26);
    }

    #[test]
    fn parse_file() {
        parse_data();
    }
}
