use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;
use crate::parsing;
use anyhow::{Context, Error, Result};
use std::str::FromStr;

pub fn advent() {
    let bags = parse_data();
    println!("Our bag can go in {} bag(s)", valid_containers(&bags, "shiny gold").len());
    println!("Our bag can contain {} bag(s)", count_contents(&bags, "shiny gold"));
}

fn valid_containers(bags: &HashMap<String, Bag>, root: &str) -> HashSet<String> {
    let mut frontier: VecDeque<_> = vec!(root.to_string()).into_iter().collect();
    let mut containers = HashSet::new();
    while let Some(bag) = frontier.pop_front() {
        for candidate in bags.values() {
            if candidate.contents.contains_key(&bag) {
                frontier.push_back(candidate.name.clone());
                containers.insert(candidate.name.clone());
            }
        }
    }
    containers
}

fn count_contents(bags: &HashMap<String, Bag>, root: &str) -> u32 {
    let mut count = 0;
    let mut frontier: VecDeque<_> = vec!((root.to_string(), 1)).into_iter().collect();
    while let Some((bag, bag_count)) = frontier.pop_front() {
        for (contained, contained_count) in bags.get(&bag).expect("Unknown bag").contents.iter() {
            frontier.push_back((contained.to_string(), bag_count * contained_count));
        }
        count += bag_count;
    }
    count-1
}

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    name: String,
    contents: HashMap<String, u32>,
}

impl Bag {
    fn build_map(bags: &[&str]) -> Result<HashMap<String, Bag>> {
        let bags = bags.iter().map(|b| b.parse()).collect::<Result<Vec<Bag>>>()?;
        Ok(bags.into_iter().map(|b| (b.name.clone(), b)).collect())
    }
}

impl FromStr for Bag {
    type Err = Error;

    fn from_str(entry: &str) -> Result<Self> {
        lazy_static! {
            static ref ENTRY_RE: Regex = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
            static ref BAG_RE: Regex = Regex::new(r"^([0-9]+) (.*) bags?$").unwrap();
        }

        let entry_caps = parsing::regex_captures(&ENTRY_RE, entry)?;
        let name = parsing::capture_group(&entry_caps, 1).to_string();

        let mut contents_txt = parsing::capture_group(&entry_caps, 2)
            .split(", ").collect::<Vec<_>>();
        if contents_txt.len() == 1 && contents_txt[0] == "no other bags" {
            contents_txt.remove(0);
        }
        let mut contents = HashMap::new();
        for contained_bag in contents_txt {
            let contents_caps = parsing::regex_captures(&BAG_RE, contained_bag)?;
            let num = parsing::capture_group(&contents_caps, 1)
                .parse::<u32>().with_context(|| format!("{}", contained_bag))?;
            let dep = parsing::capture_group(&contents_caps, 2);
            contents.insert(dep.to_string(), num);
        }

        return Ok(Bag{name, contents});
    }
}

fn parse_data() -> HashMap<String, Bag> {
    Bag::build_map(&include_str!("../data/day07.txt")
        .trim().split("\n").collect::<Vec<_>>()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example1() -> HashMap<String, Bag> {
        Bag::build_map(&include_str!("../data/day07_example1.txt")
            .trim().split("\n").collect::<Vec<_>>()).unwrap()
    }

    fn parse_example2() -> HashMap<String, Bag> {
        Bag::build_map(&include_str!("../data/day07_example2.txt")
            .trim().split("\n").collect::<Vec<_>>()).unwrap()
    }

    fn make_bag(name: &str, contents: Vec<(&str, u32)>) -> Bag {
        Bag { name: name.to_string(),
              contents: contents.iter().map(|(k,v)|(k.to_string(), *v)).collect() }
    }

    parameterized_test::create!{parse, (text, bag), {
      let parsed: Bag = text.parse().unwrap();
      assert_eq!(parsed, bag);
    }}
    parse!{
        a: ("light red bags contain 1 bright white bag, 2 muted yellow bags.",
            make_bag("light red", vec!(("bright white", 1), ("muted yellow", 2)))),
        b: ("dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            make_bag("dark orange", vec!(("muted yellow", 4), ("bright white", 3)))),
        c: ("bright white bags contain 1 shiny gold bag.",
            make_bag("bright white", vec!(("shiny gold", 1)))),
        d: ("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            make_bag("muted yellow", vec!(("faded blue", 9), ("shiny gold", 2)))),
        e: ("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            make_bag("shiny gold", vec!(("dark olive", 1), ("vibrant plum", 2)))),
        f: ("dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            make_bag("dark olive", vec!(("faded blue", 3), ("dotted black", 4)))),
        g: ("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            make_bag("vibrant plum", vec!(("faded blue", 5), ("dotted black", 6)))),
        h: ("faded blue bags contain no other bags.", make_bag("faded blue", vec!())),
        i: ("dotted black bags contain no other bags.", make_bag("dotted black", vec!())),
    }

    #[test]
    fn containers() {
        assert_eq!(valid_containers(&parse_example1(), "shiny gold").len(), 4);
        assert_eq!(valid_containers(&parse_example2(), "shiny gold").len(), 0);
    }

    #[test]
    fn contains() {
        assert_eq!(count_contents(&parse_example1(), "shiny gold"), 32);
        assert_eq!(count_contents(&parse_example2(), "shiny gold"), 126);
    }

    #[test]
    fn parse_file() {
        assert!(parse_data().len() > 0);
    }
}
