use std::num::ParseIntError;

pub fn advent() {
    let data = parse_data().unwrap();
    let invalid = find_non_sum(&data, 25).unwrap();
    println!("First invalid number: {}", invalid);
    let sequence = find_contiguous_sum(invalid, &data).unwrap();
    // TODO https://doc.rust-lang.org/1.1.0/std/iter/trait.Iterator.html#method.min_max
    println!("MinMax of contiguous sequence: {}",
             sequence.iter().min().unwrap() + sequence.iter().max().unwrap());
}

fn find_non_sum(stream: &[i64], window_size: usize) -> Option<i64> {
    for i in window_size+1..stream.len() {
        if !found_sum_in_window(stream[i], &stream[i-1-window_size..i]) {
            return Some(stream[i]);
        }
    }
    None
}

fn found_sum_in_window(value: i64, window: &[i64]) -> bool {
    for i in 0..window.len()-1 {
        for j in i+1..window.len() {
            if window[i] + window[j] == value { return true; }
        }
    }
    return false;
}

fn find_contiguous_sum<'a>(value: i64, stream: &'a[i64]) -> Option<&'a[i64]> {
    for start in 0..stream.len()-1 {
        for end in start+1..stream.len() {
            let sum: i64 = stream[start..end].iter().sum();
            if sum == value { return Some(&stream[start..end]); }
            if sum > value { break; }
            // else continue
        }
    }
    None
}

fn parse_data() -> Result<Vec<i64>, ParseIntError> {
    include_str!("../data/day09.txt").trim()
        .split("\n").map(|n| n.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &'static [i64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

    #[test]
    fn find_invalid() {
        assert_eq!(find_non_sum(&EXAMPLE, 5), Some(127));
    }

    #[test]
    fn find_sum() {
        assert_eq!(find_contiguous_sum(127, &EXAMPLE), Some(&[15, 25, 47, 40][..]));
    }

    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}
