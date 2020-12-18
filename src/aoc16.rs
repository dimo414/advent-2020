use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use anyhow::{Result, Error, Context};
use crate::parsing::{regex_captures, capture_group};

pub fn advent() {
    let mut data = parse_data().unwrap();
    println!("Error Rate: {}", data.remove_invalid_tickets());
    let labels = data.label_columns().unwrap();
    println!("Departure Product: {}", labels.iter()
        .filter(|(l, _)| l.starts_with("departure")).map(|(_, &c)| data.ticket[c]).product::<i64>());
}

struct TicketData {
    rules: HashMap<String, Box<dyn Fn(i64)->bool>>,
    ticket: Vec<i64>,
    other_tickets: Vec<Vec<i64>>,
}

impl TicketData {
    fn remove_invalid_tickets(&mut self) -> i64 {
        let unknown_tickets: Vec<_> = self.other_tickets.drain(..).collect();
        let mut error_rate = 0;
        for ticket in unknown_tickets {
            let mut valid = true;
            for cell in ticket.iter() {
                if !self.rules.values().any(|r| r(*cell)) {
                    valid = false;
                    error_rate += cell;
                }
            }
            if valid {
                self.other_tickets.push(ticket);
            }
        }
        error_rate
    }

    // Nice visualization: https://old.reddit.com/r/adventofcode/comments/ke3ypd/
    fn label_columns(&self) -> Result<HashMap<String, usize>> {
        let columns: Vec<Vec<_>> = (0..self.ticket.len()).map(|i| self.other_tickets.iter().map(|t| t[i]).collect()).collect();
        let mut candidates: HashMap<&str, HashSet<usize>> = HashMap::new();

        for (label, rule) in self.rules.iter() {
            for (i, column) in columns.iter().enumerate() {
                if column.iter().all(|&c| rule(c)) {
                    candidates.entry(label).or_insert(HashSet::new()).insert(i);
                }
            }
        }

        loop {
            let matched_columns: Vec<_> = candidates.values()
                .filter(|v| v.len() == 1).map(|v| *v.iter().next().expect("Non-empty"))
                .collect();
            if matched_columns.len() == candidates.len() { break; }
            let mut removed = false;
            for v in candidates.values_mut().filter(|v| v.len() > 1) {
                matched_columns.iter().for_each(|c| { removed |= v.remove(c); });
            }
            anyhow::ensure!(removed, "Unable to determine proper columns; got to {:?}", candidates);
        }

        Ok(candidates.into_iter()
            .map(|(k, v)| {assert_eq!(v.len(), 1); (k.to_string(), v.into_iter().next().expect("Non-empty"))})
            .collect())
    }
}

impl FromStr for TicketData {
    type Err = Error;
    fn from_str(data: &str) -> Result<Self> {
        let sections: Vec<_> = data.trim().split("\n\n").collect();
        anyhow::ensure!(sections.len() == 3);

        fn parse_rule(rule: &str) -> Result<(String, Box<dyn Fn(i64)->bool>)> {
            let regex = static_regex!(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)");
            let caps = regex_captures(regex, rule)?;
            let name = capture_group(&caps, 1);
            let r1 = capture_group(&caps, 2).parse()?;
            let r2 = capture_group(&caps, 3).parse()?;
            let r3 = capture_group(&caps, 4).parse()?;
            let r4 = capture_group(&caps, 5).parse()?;

            Ok((name.to_string(), Box::new(move |n| (n >= r1 && n <= r2) || (n >= r3 && n <= r4))))
        }
        let rules = sections[0].split("\n").map(|r| parse_rule(r)).collect::<Result<HashMap<_,_>>>()?;

        fn parse_row(row: &str) -> Result<Vec<i64>> {
            row.split(",").map(|c|c.parse().context("")).collect()
        }
        let ticket = parse_row(sections[1].split("\n").skip(1).next().context("")?)?;
        let other_tickets =
            sections[2].split("\n").skip(1).map(|r| parse_row(r)).collect::<Result<Vec<_>>>()?;

        Ok(TicketData{rules, ticket, other_tickets})
    }
}

fn parse_data() -> Result<TicketData> {
    include_str!("../data/day16.txt").trim().parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example1() -> Result<TicketData> {
        include_str!("../data/day16_example1.txt").trim().parse()
    }

    fn parse_example2() -> Result<TicketData> {
        include_str!("../data/day16_example2.txt").trim().parse()
    }

    #[test]
    fn example1() {
        let mut data = parse_example1().unwrap();
        assert_eq!(data.remove_invalid_tickets(), 71);
        assert_eq!(data.other_tickets, vec!(vec!(7,3,47)));
    }

    #[test]
    fn example2() {
        let mut data = parse_example2().unwrap();
        let initial_tickets = data.other_tickets.len();
        assert_eq!(data.remove_invalid_tickets(), 0);
        assert_eq!(data.other_tickets.len(), initial_tickets); // unchanged
        let expected =
            vec!(("row".to_string(), 0), ("class".to_string(), 1), ("seat".to_string(), 2))
                .into_iter().collect();
        assert_eq!(data.label_columns().unwrap(), expected);
    }

    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}