use std::fmt;
use std::str::FromStr;
use anyhow::{Error,Result};
use crate::parsing::{regex_captures, capture_group};
use std::collections::{HashMap, VecDeque, BTreeSet, HashSet};
use crate::euclid::{Point, point, Vector, vector};
use crate::console::{Console, Color};

const TOP: Vector = vector(0, -1);
const LEFT: Vector = vector(-1, 0);
const BOTTOM: Vector = vector(0, 1);
const RIGHT: Vector = vector(1, 0);

pub fn advent() {
    let pieces = parse_data();

    let corners = pieces.corners();
    println!("Corners: {:?} - product: {}", corners, corners.iter().map(|&i| i as i64).product::<i64>());

    let mut image = pieces.arrange_pieces();
    Console::colorize_char('░', Color::RED);
    Console::interactive_display(&image, std::time::Duration::from_millis(500));

    image.find_sea_monsters();
    Console::colorize_char('▒', Color::GREEN);
    Console::interactive_display(&image, std::time::Duration::from_millis(500));
    Console::clear_interactive();

    println!("Found {} monsters in {} candidate pixels; {} remain",
             image.monsters.len(), image.pixels.len(),
             image.pixels.len() - image.monsters.iter().map(|m| m.len()).sum::<usize>());
}

fn bits_to_int<'a>(bits: impl Iterator<Item=&'a bool>) -> u32 {
    bits.fold(0, |acc, &b| acc*2 + b as u32)
}

fn normalize(side: u32) -> u32 {
    std::cmp::min(side, side.reverse_bits() >> (32-Tile::SIDE_LEN))
}

#[derive(Clone)]
struct Tile {
    id: i32,
    grid: [[bool; Tile::SIDE_LEN]; Tile::SIDE_LEN],
    sides: [u32; 4],
}

impl Tile {
    const SIDE_LEN: usize = 10;
    const IMG_LEN: usize = 8;

    fn create(id: i32, grid: [[bool; Tile::SIDE_LEN]; Tile::SIDE_LEN]) -> Tile {
        let sides = [
            bits_to_int(grid[0].iter()), // TOP
            bits_to_int(grid.iter().map(|v| &v[0])), // LEFT
            bits_to_int(grid[Tile::SIDE_LEN-1].iter()), // BOTTOM
            bits_to_int(grid.iter().map(|v| &v[Tile::SIDE_LEN-1])) // RIGHT
        ];

        Tile{id, grid, sides}
    }

    fn side(&self, side: &Vector) -> u32 {
        match *side {
            TOP => self.sides[0],
            LEFT => self.sides[1],
            BOTTOM => self.sides[2],
            RIGHT => self.sides[3],
            _ => panic!("Invalid: {}", side),
        }
    }

    fn signature(&self) -> BTreeSet<u32> {
        self.sides.iter().map(|&s| normalize(s)).collect()
    }

    fn shared_edge(&self, other: &Tile) -> Option<Vector> {
        let other_sig = other.signature();
        if other_sig.contains(&normalize(self.sides[0])) { Some(TOP) }
        else if other_sig.contains(&normalize(self.sides[1])) { Some(LEFT) }
        else if other_sig.contains(&normalize(self.sides[2])) { Some(BOTTOM) }
        else if other_sig.contains(&normalize(self.sides[3])) { Some(RIGHT) }
        else { None }
    }

    fn align_with(&mut self, other: &Tile) -> Vector {
        let other_dir = other.shared_edge(self).expect("Tiles do not connect");

        while other_dir + self.shared_edge(other).expect("Asymmetric?") != Vector::ZERO {
            self.rotate();
        }
        let our_dir = self.shared_edge(other).expect("Asymmetric?");

        if other.side(&other_dir) != self.side(&our_dir) {
            if our_dir.x == 0 {
                self.flip_horizontal();
            } else {
                self.flip_vertical();
            }
        }
        assert_eq!(other.side(&other_dir), self.side(&our_dir),
                   "{:010b} vs. {:010b}\n{:?}\n{:?}",
                   other.side(&other_dir), self.side(&our_dir), other, self);

        other_dir
    }

    fn flip_vertical(&mut self) {
        let mut new_grid = [[false; Tile::SIDE_LEN]; Tile::SIDE_LEN];
        for x in 0..Tile::SIDE_LEN {
            for y in 0..Tile::SIDE_LEN {
                new_grid[Tile::SIDE_LEN-y-1][x] = self.grid[y][x];
            }
        }
        *self = Tile::create(self.id, new_grid);
    }

    fn flip_horizontal(&mut self) {
        let mut new_grid = [[false; Tile::SIDE_LEN]; Tile::SIDE_LEN];
        for x in 0..Tile::SIDE_LEN {
            for y in 0..Tile::SIDE_LEN {
                new_grid[y][Tile::SIDE_LEN-x-1] = self.grid[y][x];
            }
        }
        *self = Tile::create(self.id, new_grid);
    }

    // https://math.stackexchange.com/q/1330161/1887
    fn rotate(&mut self) {
        let mut new_grid = [[false; Tile::SIDE_LEN]; Tile::SIDE_LEN];
        for x in 0..Tile::SIDE_LEN {
            for y in 0..Tile::SIDE_LEN {
                // vector(-vec.y, vec.x),
                new_grid[x][Tile::SIDE_LEN-y-1] = self.grid[y][x];
            }
        }
        *self = Tile::create(self.id, new_grid);
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                if self.grid[y][x] {
                    if x == 0 || x == self.grid.len()-1 || y == 0 || y == self.grid.len() - 1 {
                        out.push('░');
                    } else {
                        out.push('█');
                    }
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: [{:010b}, {:010b}, {:010b}, {:010b}]", self.id,
               normalize(self.sides[0]), normalize(self.sides[1]),
               normalize(self.sides[2]), normalize(self.sides[3]))
    }
}

impl FromStr for Tile {
    type Err = Error;
    fn from_str(str: &str) -> Result<Self> {
        let lines: Vec<_> = str.trim().split("\n").collect();
        anyhow::ensure!(lines.len() == 11);
        let regex = static_regex!(r"Tile (\d+):");
        let caps = regex_captures(regex, lines[0])?;
        let id = capture_group(&caps, 1).parse()?;

        let mut grid = [[false; Tile::SIDE_LEN]; Tile::SIDE_LEN];
        for y in 0..Tile::SIDE_LEN {
            for x in 0..Tile::SIDE_LEN {
                let c = lines[y+1].chars().skip(x).next().unwrap();
                anyhow::ensure!(c == '#' || c == '.');
                grid[x][y] = c == '#';
            }
        }
        Ok(Tile::create(id, grid))
    }
}

struct Pieces {
    tiles: HashMap<i32, Tile>,
    neighbors: HashMap<i32, BTreeSet<i32>>,
}

impl Pieces {
    fn create(tiles: Vec<Tile>) -> Pieces {
        let tiles: HashMap<_, _> = tiles.into_iter().map(|t| (t.id, t)).collect();
        let mut edges= HashMap::new();
        for tile in tiles.values() {
            for sig in tile.signature().iter() {
                edges.entry(*sig).or_insert(BTreeSet::new()).insert(tile.id);
            }
        }
        if let Some((edge, ids)) =
                edges.iter().find(|(_, v)| !(1..=2).contains(&v.len())) {
            panic!("Unexpected edge counts; edge {:010b} matched tiles {:?}", edge, ids);
        }

        let mut neighbors = HashMap::new();
        for tile in tiles.values() {
            let n = tile.signature().iter()
                .flat_map(|s| edges[s].iter()).cloned().filter(|&id| id!=tile.id).collect();
            neighbors.insert(tile.id, n);
        }

        // Remove unpaired edges, we don't care about them
        edges.retain(|_, v| v.len() > 1);

        Pieces{tiles, neighbors}
    }

    fn corners(&self) -> Vec<i32> {
        self.neighbors.iter().filter(|(_, n)| n.len() == 2).map(|(&id, _)| id).collect()
    }

    fn arrange_pieces(mut self) -> Image {
        let mut board = HashMap::new();
        let first_corner = self.corners()[0]; // arbitrary corner
        board.insert(first_corner, point(0, 0));
        let mut frontier = VecDeque::new();
        for neighbor in self.neighbors[&first_corner].iter() {
            frontier.push_back((first_corner, *neighbor));
        }

        while let Some((next_to, id)) = frontier.pop_front() {
            if board.contains_key(& id) { continue; }
            let mut tile = self.tiles[&id].clone();
            let neighbor = &self.tiles[&next_to];
            let dir = tile.align_with(neighbor);
            board.insert(tile.id, board[&next_to] + dir);
            for next in self.neighbors[&tile.id].iter() {
                frontier.push_back((tile.id, *next));
            }
            self.tiles.insert(tile.id, tile); // overwrite with newly-positioned tile
        }

        Image::create(&board.into_iter().map(|(id, p)| (p, self.tiles.remove(&id).unwrap())).collect())
    }
}

struct Image {
    pixels: HashSet<Point>,
    monsters: Vec<HashSet<Point>>, // Not ideal that this isn't populated at construction but w/e
}

impl Image {
    //                  #
    //#    ##    ##    ###
    // #  #  #  #  #  #
    const MONSTER: &'static [Vector] = &[
        vector(18, -1),
        vector(0, 0), vector(5, 0), vector(6, 0), vector(11, 0), vector(12, 0), vector(17, 0), vector(18, 0), vector(19, 0),
        vector(1, 1), vector(4, 1), vector(7, 1), vector(10, 1), vector(13, 1), vector(16, 1)
    ];

    fn create(tiles: &HashMap<Point, Tile>) -> Image {
        let mut pixels = HashSet::new();
        for (pos, tile) in tiles.iter() {
            for y in 0..Tile::IMG_LEN {
                for x in 0..Tile::IMG_LEN {
                    if tile.grid[y+1][x+1] {
                        let img_len = Tile::IMG_LEN as i32;
                        pixels.insert(point(pos.x*img_len+(x as i32), pos.y*img_len+(y as i32)));
                    }
                }
            }
        }
        Image{pixels, monsters: Vec::new()}
    }

    fn find_sea_monsters(&mut self) {
        // https://math.stackexchange.com/q/1330161/1887
        fn rotate(shape: &[Vector]) -> Vec<Vector> {
            shape.iter().map(|vec| vector(-vec.y, vec.x)).collect()
        }

        fn flip_x(shape: &[Vector]) -> Vec<Vector> {
            shape.iter().map(|vec| vector(-vec.x, vec.y)).collect()
        }

        fn flip_y(shape: &[Vector]) -> Vec<Vector> {
            shape.iter().map(|vec| vector(vec.x, -vec.y)).collect()
        }

        fn scan(pixels: &HashSet<Point>, shape: &[Vector]) -> Vec<HashSet<Point>> {
            let mut found = Vec::new();
            for pos in pixels.iter() {
                if shape.iter().all(|offset| pixels.contains(&(pos + offset))) {
                    found.push(shape.iter().map(|offset| pos+offset).collect());
                }
            }
            found

        }

        for mut shape in
        vec!(Image::MONSTER.to_vec(), flip_x(&Image::MONSTER), flip_y(&Image::MONSTER)) {
            for _ in 0..4 {
                let found = scan(&self.pixels, &shape);
                if !found.is_empty() {
                    self.monsters = found;
                    return;
                }
                shape = rotate(&shape);
            }
        }
        panic!("No sea monsters found; bad data?")
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let mut last_y = None;
        for pos in Point::display_order_box(self.pixels.iter().cloned()).unwrap() {
            if let Some(last_y) = last_y {
                if pos.y != last_y { out.push('\n'); }
            }
            last_y = Some(pos.y);

            if self.monsters.iter().any(|m| m.contains(&pos)) {
                debug_assert!(self.pixels.contains(&pos), "??? {:?}", self.monsters);
                out.push('▒');
            } else if self.pixels.contains(&pos) {
                out.push('█');
            } else {
                out.push(' ');
            }
        }
        write!(f, "{}", out)
    }
}

fn parse_data() -> Pieces {
    Pieces::create(include_str!("../data/day20.txt").trim()
        .split("\n\n").map(|t| t.parse()).collect::<Result<Vec<_>>>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example() -> Pieces {
        Pieces::create(include_str!("../data/day20_example.txt").trim()
            .split("\n\n").map(|t| t.parse()).collect::<Result<Vec<_>>>().unwrap())
    }

    #[test]
    fn corners() {
        let tiles = parse_example();
        let corners: HashSet<_> = tiles.corners().into_iter().collect();
        let expected: HashSet<_> = vec!(1951, 3079, 2971, 1171).into_iter().collect();
        assert_eq!(corners, expected);
    }

    #[test]
    fn count_monsters() {
        let mut image = parse_example().arrange_pieces();
        image.find_sea_monsters();
        assert_eq!(image.monsters.len(), 2);
        assert_eq!(image.pixels.len() - image.monsters.iter().map(|m| m.len()).sum::<usize>(), 273);
    }
}