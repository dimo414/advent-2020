use std::cmp::max;
use std::collections::BTreeSet;

pub fn advent() {
    let seats = parse_data();
    let mut highest = 0;
    for seat in seats.iter() {
      highest = max(highest, make_id(parse(seat)));
    }
    println!("Highest seat ID: {}", highest);

    let mut candidate_seats = {0..1024}.collect::<BTreeSet<_>>();
    for seat in seats.iter() {
        candidate_seats.remove(&make_id(parse(seat)));
    }
    let empty_seats = strip_sequences(&candidate_seats);
    assert_eq!(empty_seats.len(), 1);
    println!("Empty seat ID: {}", empty_seats.iter().next().unwrap());
}

fn parse_data() -> Vec<&'static str> {
    return include_str!("../data/day05.txt").lines().collect();
}

fn to_num(symbol: &str, ones: char) -> u32 {
  let mut num = 0;
  for c in symbol.chars() {
    num *= 2;
    if c == ones {
      num += 1;
    }
  }
  return num;
}

fn parse(seat: &str) -> (u32, u32) {
  assert_eq!(seat.len(), 10);
  let row = &seat[..7];
  let col = &seat[7..];
  return (to_num(row, 'B'), to_num(col, 'R'));
}

fn make_id(seat: (u32, u32)) -> u32 { seat.0 * 8 + seat.1 }

fn strip_sequences(set: &BTreeSet<u32>) -> BTreeSet<u32> {
    let mut ret = BTreeSet::new();
    for e in set.iter() {
        if (*e == 0 || !set.contains(&(*e-1))) && !set.contains(&(*e+1)) {
            ret.insert(*e);
        }
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{seats, (code, seat, id), {
      let decoded = parse(code);
      assert_eq!(decoded, seat);
      assert_eq!(make_id(decoded), id);
    }}
    seats!{
      a: ("FBFBBFFRLR", (44, 5), 357),
      b: ("BFFFBBFRRR", (70, 7), 567),
      c: ("FFFBBBFRRR", (14, 7), 119),
      d: ("BBFFBBFRLL", (102, 4), 820),
    }

    #[test]
    fn strip_seq() {
        let set = vec!(2, 3, 4, 6, 9, 10, 12, 14, 15).into_iter().collect::<BTreeSet<_>>();
        let expected = vec!(6, 12).into_iter().collect::<BTreeSet<_>>();
        assert_eq!(strip_sequences(&set), expected);
    }

    #[test]
    fn parse_file() {
        assert!(parse_data().len() > 0);
    }
}
