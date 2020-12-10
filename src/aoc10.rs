use std::num::ParseIntError;
use std::collections::HashMap;

pub fn advent() {
    let adapters = parse_data().unwrap();
    let counts = adapter_deltas(&adapters);
    println!("Adapter delta histogram: {:?} - delta-1*3: {}", counts, counts[0] * (counts[2]));
    println!("Possible valid combinations: {}", adapter_combos(&adapters));
}

fn adapter_deltas(adapters: &[i64]) -> [i64; 3] {
    let mut counts = [0, 0, 0];
    let mut input_jolts = 0;
    for a in adapters[1..].iter() {
        let delta = a - input_jolts;
        assert!(delta >= 1 && delta <= 3, "Unexpected delta {}-{} = {}", a, input_jolts, delta);
        counts[delta as usize -1] += 1;
        input_jolts = *a;
    }
    counts
}

// From https://old.reddit.com/r/adventofcode/comments/ka9pc3/2020_day_10_part_2_suspicious_factorisation/gf94sxy/
#[cfg(test)]
fn linear_adapter_combos(adapters: &[i64]) -> i64 {
    let (mut pow2, mut pow7) = (0, 0);
    for i in 1..adapters.len()-1 {
        if i >= 3 && adapters[i+1] - adapters[i-3] == 4 {
            pow7 += 1;
            pow2 -= 2;
        } else if adapters[i+1] - adapters[i-1] == 2 {
            pow2 += 1;
        }
    }
    2_i64.pow(pow2) * 7_i64.pow(pow7)
}

fn adapter_combos(adapters: &[i64]) -> i64 {
    let mut cache = HashMap::new();
    return adapter_combos_cached(adapters, 0, &mut cache);
}

fn adapter_combos_cached(adapters: &[i64], i: usize, cache: &mut HashMap<usize, i64>) -> i64 {
    if i == adapters.len() -1 { return 1; }
    if cache.contains_key(&i) { return cache[&i]; }

    let mut sum = 0;
    let mut j = i+1;
    while j < adapters.len() && adapters[j] <= adapters[i] + 3 {
        sum += adapter_combos_cached(adapters, j, cache);
        j += 1;
    }
    assert_ne!(sum, 0);
    cache.insert(i, sum);
    sum
}

fn prepare_data(mut data: Vec<i64>) -> Vec<i64> {
    data.push(0);
    data.sort();
    assert_eq!(data[0], 0);
    data.push(data[data.len()-1]+3);
    data
}

fn parse_data() -> Result<Vec<i64>, ParseIntError> {
    include_str!("../data/day10.txt").trim().split("\n")
        .map(|n| n.parse()).collect::<Result<Vec<_>, _>>()
        .map(|v| prepare_data(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static!{
        static ref EXAMPLE_A: Vec<i64> = prepare_data(vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4));
        static ref EXAMPLE_B: Vec<i64> = prepare_data(vec!(
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19,
        38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3));
    }

    #[test]
    fn count_deltas() {
        assert_eq!(adapter_deltas(&EXAMPLE_A), [7, 0, 5]);
        assert_eq!(adapter_deltas(&EXAMPLE_B), [22, 0, 10]);
    }
    
    #[test]
    fn count_adapters() {
        assert_eq!(adapter_combos(&EXAMPLE_A), 8);
        assert_eq!(linear_adapter_combos(&EXAMPLE_A), 8);
        assert_eq!(adapter_combos(&EXAMPLE_B), 19208);
        assert_eq!(linear_adapter_combos(&EXAMPLE_B), 19208);
    }

    #[test]
    fn prepare() {
        let nums = vec!(4, 2, 10, 7, 3);
        assert_eq!(prepare_data(nums), vec!(0, 2, 3, 4, 7, 10, 13));
    }
    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}
