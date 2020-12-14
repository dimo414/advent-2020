pub fn advent() {
    // Observation: all non-x inputs are primes.
    // https://www.wolframalpha.com/input/?i=lcm+of+37%2C+41%2C+601%2C+19%2C+17%2C+23%2C+29%2C+443%2C+13

    let (timestamp, routes) = parse_data();
    let (route, wait_time) = next_bus(timestamp, &routes.iter().filter_map(|&e| e).collect::<Vec<_>>());
    println!("Route {} will arrive in {} minutes, value: {}", route, wait_time, route * wait_time);
    println!("Earliest sequential timestamp: {}", find_timestamp(&routes));

    // But, lo, WolframAlpha can just solve Part 2 directly...
    // https://www.wolframalpha.com/input/?i=7a%3Dt%2C+13b-1%3Dt%2C+59c-4%3Dt%2C+31d-6%3Dt%2C+19f-7%3Dt
    // https://www.wolframalpha.com/input/?i=37a-0%3Dt%2C+41b-27%3Dt%2C+601c-37%3Dt%2C+19d-49%3Dt%2C+17f-54%3Dt%2C+23g-60%3Dt%2C+29h-66%3Dt%2C+443i-68%3Dt%2C+13j-81%3Dt
}

fn next_bus(timestamp: i64, routes: &[i64]) -> (i64, i64) {
    routes.iter().map(|&r| (r, r - (timestamp%r))).min_by_key(|&(_,d)|d).unwrap()
}

fn find_timestamp(routes: &[Option<i64>]) -> i64 {
    let constraints: Vec<_> = routes.iter().enumerate().filter_map(|(i, r)| r.map(|r| (i as i64, r))).map(|(i, r)| ((((r-i) % r)+r)%r, r)).collect();
    rosetta::chinese_remainder(&constraints).unwrap()
}

fn parse_data() -> (i64, Vec<Option<i64>>) {
    let lines: Vec<_> = include_str!("../data/day13.txt").split("\n").collect();
    (lines[0].parse().unwrap(), lines[1].split(",").map(|e|e.parse().ok()).collect())
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
mod rosetta {
    // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    // https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
    fn mod_inv(x: i64, n: i64) -> Option<i64> {
        let (g, x, _) = egcd(x, n);
        if g == 1 {
            Some((x % n + n) % n)
        } else {
            None
        }
    }

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Using_the_existence_construction
    #[cfg(test)]
    fn chinese_remainder_original(residues: &[i64], modulii: &[i64]) -> Option<i64> {
        let prod = modulii.iter().product::<i64>();

        let mut sum = 0;

        for (&residue, &modulus) in residues.iter().zip(modulii) {
            let p = prod / modulus;
            sum += residue * mod_inv(p, modulus)? * p
        }

        Some(sum % prod)
    }

    // Rosetta's chinese_remainder refactored to take one vec of tuples instead of two vecs
    pub fn chinese_remainder(elements: &[(i64,i64)]) -> Option<i64> {
        let prod = elements.iter().map(|&(_, m)| m).product::<i64>();

        let mut sum = 0;

        for &(residue, modulus) in elements {
            let p = prod / modulus;
            sum += residue * mod_inv(p, modulus)? * p
        }

        Some(sum % prod)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn validate() {
            let residues = [2,3,2];
            let modulii = [3,5,7];

            assert_eq!(chinese_remainder_original(&residues, &modulii), Some(23));
            let zipped: Vec<_> = residues.iter().zip(&modulii).map(|(&r,&m)|(r,m)).collect();
            assert_eq!(chinese_remainder(&zipped), Some(23));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_bus() {
        let (timestamp, buses) = (939, [7,13,59,31,19]);
        assert_eq!(next_bus(timestamp, &buses), (59, 5));
    }

    parameterized_test::create!{find_timestamp, (routes, expected), {
        assert_eq!(find_timestamp(&routes), expected);
    }}
    find_timestamp!{
      a: ([Some(7),Some(13),None,None,Some(59),None,Some(31),Some(19)], 1068781),
      b: ([Some(17),None,Some(13),Some(19)], 3417),
      c: ([Some(67),Some(7),Some(59),Some(61)], 754018),
      d: ([Some(67),None,Some(7),Some(59),Some(61)], 779210),
      e: ([Some(67),Some(7),None,Some(59),Some(61)], 1261476),
      f: ([Some(1789),Some(37),Some(47),Some(1889)], 1202161486),

    }

    #[test]
    fn parse_file() {
        parse_data();
    }
}
