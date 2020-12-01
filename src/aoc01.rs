pub fn advent() {
    let data = parse_data();
    let (a, b) = find_pair(&data).expect("No result");
    println!("{}*{} = {}", a, b, a*b);
    let (a, b, c) = find_triple(&data).expect("No result");
    println!("{}*{}*{} = {}", a, b, c, a*b*c);
}

fn parse_data() -> Vec<u32> {
    return include_str!("../data/day01.txt").lines().map(|l| l.parse::<u32>().unwrap()).collect();
}

fn find_pair(data: &[u32]) -> Option<(u32, u32)> {
    for offset in 0..data.len()-1 {
        let a = data[offset];
        for b in &data[offset+1..] {
            if a + b == 2020 { return Some((a, *b)) }
        }
    }
    None
}

fn find_triple(data: &[u32]) -> Option<(u32, u32, u32)> {
    for offset_a in 0..data.len()-2 {
        let a = data[offset_a];
        for offset_b in offset_a+1..data.len()-1 {
            let b = data[offset_b];
            for c in &data[offset_b+1..] {
                if a + b + c == 2020 { return Some((a, b, *c)) }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(find_pair(&[1721, 979, 366, 299, 675, 1456]), Some((1721, 299)));
        assert_eq!(find_triple(&[1721, 979, 366, 299, 675, 1456]), Some((979, 366, 675)));
    }

    #[test]
    fn parse_file() {
        assert!(parse_data().len() > 0);
    }
}
