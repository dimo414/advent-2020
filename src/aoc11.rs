use std::collections::HashMap;
use crate::euclid::{Point,point,vector,Vector};
use crate::console::{Color, Console};
use std::fmt;

pub fn advent() {
    Console::colorize_char('L', Color::BLUE);
    Console::colorize_char('#', Color::YELLOW);
    Console::colorize_char('.', Color::GREY);
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

#[derive(Eq, PartialEq, Clone)]
struct Floor {
    points: HashMap<Point, State>,
}

impl fmt::Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let mut last_y = None;
        for point in Point::display_order_box(self.points.keys().cloned()).unwrap() {
            if let Some(last_y) = last_y {
                if point.y != last_y { out.push('\n'); }
            }
            last_y = Some(point.y);
            let c = match self.points.get(&point) {
                Some(State::Empty) => 'L',
                Some(State::Occupied) => '#',
                Some(State::Floor) => '.',
                None => ' ',
            };
            out.push(c);
        }
        write!(f, "{}", out)
    }
}

trait Strategy {
    fn count_nearby(&self, floor: &Floor, pos: Point) -> u32;
    fn next_state(&self, state: State, count: u32) -> State;
}

struct Adjacent{}
impl Strategy for Adjacent {
    fn count_nearby(&self, floor: &Floor, pos: Point) -> u32 {
        let mut sum = 0;
        for dir in Vector::ORDINAL {
            if let Some(s) = floor.points.get(&(pos+dir)) {
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
    fn count_nearby(&self, floor: &Floor, pos: Point) -> u32 {
        let mut sum = 0;
        for dir in Vector::ORDINAL {
            let mut looking = pos;
            loop {
                looking += dir;
                if let Some(s) = floor.points.get(&looking) {
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

fn count_occupied(floor: &Floor) -> usize {
    floor.points.values().filter(|s| **s==State::Occupied).count()
}

fn find_stable(floor: &Floor, strat: &dyn Strategy) -> Floor {
    let mut last = floor.clone();
    loop {
        let next = iteration(&last, strat);
        if next == last { break; }
        Console::interactive_display(last, std::time::Duration::from_millis(50));
        last = next;
    }
    Console::clear_interactive();
    last
}

fn iteration(floor: &Floor, strat: &dyn Strategy) -> Floor {
    let mut next = HashMap::new();
    for pos in Point::display_order_box(floor.points.keys().cloned()).unwrap() {
        if let Some(s) = floor.points.get(&pos) {
            let count = strat.count_nearby(floor, pos);
            next.insert(pos, strat.next_state(*s, count));
        }
    }
    Floor { points: next }
}

fn build_map(str: &str) -> Floor {
    let rows: Vec<_> = str.trim().split("\n").collect();
    let mut points = HashMap::new();
    let mut pos = point(0, 0);
    for row in rows {
        for col in row.chars() {
            let state = match col {
                'L' => State::Empty,
                '#' => State::Occupied,
                '.' => State::Floor,
                _ => panic!(),
            };
            points.insert(pos, state);
            pos += vector(1, 0);
        }
        pos = point(0, pos.y+1);
    }
    Floor { points }
}

fn parse_data() -> Floor {
    build_map(include_str!("../data/day11.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example() -> Floor {
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
