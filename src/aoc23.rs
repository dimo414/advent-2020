use std::fmt;

pub fn advent() {
    let input_u = vec!(4,6,7,5,2,8,1,9,3);

    let mut cups = Cups::create(&input_u);
    elapsed! {
        for _ in 0..100 {
            cups.play_round();
        }
    }
    println!("After 100 rounds: {}",
             cups.iter_from1().skip(1).map(|v| format!("{}", v)).collect::<Vec<_>>().join(""));

    let mut cups = Cups::create(&input_u.iter().copied().chain(10..=1000000).collect::<Vec<_>>());
    elapsed! {
        for _ in 0..10000000 {
            cups.play_round();
        }
    }
    println!("After CRAB rounds; cups product: {}",
             cups.iter_from1().skip(1).take(2).map(|v| v as u64).product::<u64>());
}

#[derive(Debug, Clone)]
struct Cups {
    next: Vec<usize>,
    head: usize,
}

impl Cups {
    fn create(initial: &[usize]) -> Cups {
        use std::convert::TryFrom;
        let mut next: Vec<isize> = vec![-1; initial.len()+1];
        next[0] = 0; // [0] is unused
        for i in 0..initial.len()-1 {
            next[initial[i]] = initial[i+1] as isize;
        }
        next[initial[initial.len()-1]] = initial[0] as isize;
        Cups{
            next:next.iter()
                .map(|&v| usize::try_from(v).expect("Incomplete sequence of cups")).collect(),
            head:initial[0],
        }
    }

    pub fn iter(&self) -> CupsIterator<'_> {
        CupsIterator{ cups: self, ptr: Some(self.head), start: self.head }
    }

    pub fn iter_from1(&self) -> CupsIterator<'_> {
        CupsIterator{ cups: self, ptr: Some(1), start: 1 }
    }

    fn play_round(&mut self) {
        let to_move: Vec<_> = self.iter().skip(1).take(3).collect();
        self.next[self.head] = self.next[to_move[2]]; // "remove" [1..3]
        for dest in (1..self.head).rev().chain((self.head..self.next.len()).rev()) {
            if to_move.contains(&dest) { continue; }
            let after = self.next[dest];
            self.next[dest] = to_move[0];
            self.next[to_move[2]] = after;
            break;
        }
        debug_assert_eq!(self.iter().count(), self.next.len()-1);
        self.head = self.next[self.head];
    }
}

impl fmt::Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.iter().collect::<Vec<_>>())
    }
}

pub struct CupsIterator<'a> {
    cups: &'a Cups,
    ptr: Option<usize>,
    start: usize,
}

impl<'a> Iterator for CupsIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ptr) = self.ptr {
            let next = self.cups.next[ptr];
            self.ptr = if next == self.start { None } else { Some(next) };
            return Some(ptr);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut cups = Cups::create(&vec!(3,8,9,1,2,5,4,6,7));

        for _ in 0..10 {
            cups.play_round();
        }
        assert_eq!(cups.iter().collect::<Vec<_>>(), vec!(8, 3, 7, 4, 1, 9, 2, 6, 5));
        assert_eq!(cups.iter_from1().collect::<Vec<_>>(), vec!(1, 9, 2, 6, 5, 8, 3, 7, 4));

        for _ in 10..100 {
            cups.play_round();
        }
        assert_eq!(cups.iter_from1().collect::<Vec<_>>(), vec!(1, 6, 7, 3, 8, 4, 5, 2, 9));
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)] // too slow to run without --release
    fn crab_numbers() {
        let mut cups = Cups::create(
            &vec!(3,8,9,1,2,5,4,6,7).into_iter().chain(10..=1000000).collect::<Vec<_>>());
        for _ in 0..10000000 {
            cups.play_round();
        }
        let result = cups.iter_from1().take(3).collect::<Vec<_>>();
        assert_eq!(result, vec!(1, 934001, 159792));
    }
}
