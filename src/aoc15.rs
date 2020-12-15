use std::collections::HashMap;

pub fn advent() {
    let seed = vec!(12,1,16,3,11,0);
    println!("2020: {}", memory_mapped(&seed, 2020));
    println!("30000000: {}", memory_mapped(&seed, 30000000));
}

fn memory_mapped(seed: &[i32], target: usize) -> i32 {
    let mut seen:HashMap<_,_>  = seed.iter().enumerate().map(|(i,&v)|(v,i)).collect();
    let mut prior = None;
    let mut result = None;
    for i in seed.len()..target {
        let value = match prior {
            Some(prior) => (i-prior-1) as i32,
            None => 0,
        };
        prior = seen.insert(value, i);
        result = Some(value);
    }
    result.unwrap()
}

#[cfg(test)]
pub fn memory_search(seed: &[i32], target: usize) -> i32 {
    let mut turns: Vec<_> = seed.iter().cloned().collect();
    while turns.len() < target {
        let last = turns[turns.len()-1];
        let cur = match turns.iter().enumerate().rev().skip(1).find(|&(_, &prior)| last == prior) {
            Some((i, _)) => (turns.len()-i-1) as i32,
            None => 0,
        };
        turns.push(cur);
    }
    turns[turns.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{mem2020, (seed, expected), {
        assert_eq!(memory_search(&seed, 2020), expected);
        assert_eq!(memory_mapped(&seed, 2020), expected);
    }}
    mem2020!{
      a: ([0,3,6], 436),
      b: ([1,3,2], 1),
      c: ([2,1,3], 10),
      d: ([1,2,3], 27),
      e: ([2,3,1], 78),
      f: ([3,2,1], 438),
      g: ([3,1,2], 1836),
    }

    parameterized_test::create!{mem30000000, (seed, expected), {
        assert_eq!(memory_mapped(&seed, 30000000), expected);
    }}
    #[cfg(not(debug_assertions))]
    mem30000000!{
      a: ([0,3,6], 175594),
      b: ([1,3,2], 2578),
      c: ([2,1,3], 3544142),
      d: ([1,2,3], 261214),
      e: ([2,3,1], 6895259),
      f: ([3,2,1], 18),
      g: ([3,1,2], 362),
    }
}
