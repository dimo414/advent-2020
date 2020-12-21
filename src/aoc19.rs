use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use anyhow::{Error, Result};
use crate::parsing;
use regex::Regex;

pub fn advent(args: &[String]) {
    let (mut rules, expressions) = read_data().unwrap();
    let regex = elapsed!("Construct regex", rules.to_regex().unwrap());

    // This really isn't necessary (it still runs in less than a second) but it shrinks the number
    // of rule elements by ~50% and does appear to improve speeds somewhat.
    elapsed!(rules.reduce());

    println!("Initially valid: {}", elapsed!("Initial", rules.check_all(&expressions).len()));
    rules.make_recursive();
    println!("With recursive rules: {}", elapsed!("Recursive", rules.check_all(&expressions).len()));

    println!("-----");
    println!("Initially valid (regex): {}",
             elapsed!("Initial (regex)", expressions.iter().filter(|e| regex.is_match(e)).count()));
    // Empirically, 5-deep is sufficient to get the right answer. 8 deep causes Rust to fail after
    // attempting to allocate ~28GB while evaluating the regex. At 15 it attempts to allocate
    // ~318GB(!), any deeper and the compiled regex itself exceeds the ~10MB limit.
    let depth = args.get(0).map(|a| a.parse::<usize>().unwrap()).unwrap_or(5);
    rules.make_pseduo_recursive(depth);
    println!("With pseudo-recursive ({}) rules: {}", depth,
             elapsed!("Pseduo-recursive", rules.check_all(&expressions).len()));
    let recursive_regex = rules.to_regex().unwrap();
    println!("With pseudo-recursive ({}) rules (regex): {}", depth,
             elapsed!("Pseduo-recursive (regex)",
             expressions.iter().filter(|e| recursive_regex.is_match(e)).count()));

}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Rule {
    Literal(String),
    Reference(u32),
    Sequence(Vec<Rule>),
    Disjunction(Vec<Rule>),
}

impl Rule {
    fn reduce(self) -> Rule {
        if let Rule::Sequence(parts) = self {
            let mut reduced = parts.into_iter().map(|r| r.reduce())
                .flat_map(|r|
                    if let Rule::Sequence(seq) = r { seq.into_iter()
                    } else { vec!(r).into_iter() }
                ).fold(vec!(), |mut v, r| {
                    if v.is_empty() {
                        v.push(r);
                    } else if let (Rule::Literal(t1), Rule::Literal(t2)) = (&v[v.len()-1], &r) {
                        let idx=v.len()-1;
                        v[idx] = Rule::Literal(format!("{}{}", t1, t2));
                    } else {
                        v.push(r);
                    }
                    return v;
                });
            if reduced.len() == 1 {
                return reduced.pop().expect("len=1");
            }
            Rule::Sequence(reduced)
        } else {
            self
        }
    }
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(data: &str) -> Result<Self> {
        let disjunctions: Vec<_> = data.split(" | ").collect();
        if disjunctions.len() > 1 {
            return Ok(Rule::Disjunction(disjunctions.iter().map(|r| r.parse()).collect::<Result<_>>()?));
        }
        if data.starts_with("\"") && data.ends_with("\"") {
            return Ok(Rule::Literal(data[1..data.len()-1].to_string()));
        }
        anyhow::ensure!(!data.contains("\""));
        let sequence: Vec<_> = data.split(" ").collect();
        if sequence.len() > 1 {
            return Ok(Rule::Sequence(sequence.iter().map(|r| r.parse()).collect::<Result<_>>()?));
        }
        Ok(Rule::Reference(data.parse()?))
    }
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<u32, Rule>,
}

impl Rules {
    fn size_of(&self, rule_id: u32) -> usize {
        fn rule_size(slf: &Rules, rule: &Rule) -> usize {
            match rule {
                Rule::Literal(_) => 1,
                Rule::Reference(id) => 1 + slf.size_of(*id),
                Rule::Sequence(rs) | Rule::Disjunction(rs) =>
                    1 + rs.iter().map(|r| rule_size(slf, r)).sum::<usize>(),
            }
        }
        rule_size(self, &self.rules[&rule_id])
    }

    fn size(&self) -> usize {
        self.rules.keys().map(|&id| self.size_of(id)).sum()
    }

    fn reduce(&mut self) {
        fn reduce_once(slf: &mut Rules) {
            fn reduce_rule(rule: &mut Rule, literals: &HashMap<u32, Rule>) {
                match rule {
                    Rule::Reference(id) => if let Some(lit) = literals.get(id) {
                        *rule = lit.clone();
                    },
                    Rule::Disjunction(rs) | Rule::Sequence(rs) => {
                        for rule in rs.iter_mut() {
                            reduce_rule(rule, literals);
                        }
                    }
                    _ => {},
                }
                *rule = rule.clone().reduce();
            }

            let literals: HashMap<_, _> = slf.rules.iter()
                .filter(|(_, r)| match r {
                    Rule::Literal(_) => true,
                    _ => false
                })
                .map(|(&id, r)| (id, r.clone())).collect();
            for rule in slf.rules.values_mut() {
                reduce_rule(rule, &literals);
            }
        }

        loop {
            let size = self.size();
            reduce_once(self);
            if size == self.size() { break; }
        }
        self.gc();
    }

    fn gc(&mut self) {
        fn visit(rule: &Rule) -> HashSet<u32> {
            match rule {
                Rule::Literal(_) => HashSet::new(),
                Rule::Reference(id) => vec!(*id).into_iter().collect(),
                Rule::Disjunction(rs) | Rule::Sequence(rs) =>
                    rs.iter().flat_map(|r| visit(r).into_iter()).collect(),
            }
        }

        let mut referenced: HashSet<_> = self.rules.values()
            .flat_map(|r| visit(r).into_iter()).collect();
        referenced.insert(0);

        let garbage: Vec<_> = self.rules.keys()
            .filter(|id| !referenced.contains(id)).cloned().collect();
        for g in garbage {
            self.rules.remove(&g);
        }
    }

    fn make_recursive(&mut self) {
        self.rules.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
        self.rules.insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());
    }

    fn make_pseduo_recursive(&mut self, depth: usize) {
        let mut parts_8 = Vec::new();
        let mut parts_11 = Vec::new();
        for repetitions in 1..=depth {
            let reps_42 = vec!["42"; repetitions].join(" ");
            parts_11.push(format!("{} {}", reps_42, vec!["31"; repetitions].join(" ")));
            parts_8.push(reps_42);
        }
        self.rules.insert(8, parts_8.join(" | ").parse::<Rule>().unwrap());
        self.rules.insert(11, parts_11.join(" | ").parse::<Rule>().unwrap());
    }

    fn to_regex(&self) -> Result<Regex> {
        fn regex_id(slf: &Rules, rule_id: u32) -> String {
            regex(slf, &slf.rules[&rule_id])
        }

        fn regex(slf: &Rules, rule: &Rule) -> String {
            match rule {
                Rule::Literal(pat) => pat.to_string(),
                Rule::Reference(id) => regex_id(slf, *id),
                Rule::Sequence(seq) =>
                    seq.iter().map(|r| regex(slf, r)).collect::<Vec<_>>().join(""),
                Rule::Disjunction(dis) =>
                    format!("({})", dis.iter().map(|r| regex(slf, r)).collect::<Vec<_>>().join("|")),
            }
        }

        Ok(Regex::new(&format!("^{}$", regex_id(self, 0)))?)
    }

    // Checks the text against rule 0
    fn check(&self, text: &str) -> bool {
        fn check_id<'a>(slf: &Rules, rule_id: u32, text: &'a str) -> Vec<&'a str> {
            // TODO return Result<>
            check(slf, slf.rules.get(&rule_id).expect("No such rule"), text)
        }

        fn check_seq<'a>(slf: &Rules, seq: &[Rule], text: &'a str) -> Vec<&'a str> {
            let mut remainders = vec!(text);
            for rule in seq {
                remainders = remainders.into_iter()
                    .flat_map(|r| check(slf, rule, r).into_iter()).collect();
            }
            remainders
        }

        fn check<'a>(slf: &Rules, rule: &Rule, text: &'a str) -> Vec<&'a str> {
            match rule {
                Rule::Literal(pat) =>
                    if text.starts_with(pat) { vec!(&text[pat.len()..]) } else { vec!() },
                Rule::Reference(id) => check_id(slf, *id, text),
                Rule::Sequence(seq) => {
                    assert!(seq.len() > 1);
                    check_seq(slf, seq, text)
                },
                Rule::Disjunction(vec) =>
                    vec.iter().flat_map(|r| check(slf, r, text).into_iter()).collect(),
            }
        }

        let remainders =
            check(self, &self.rules.get(&0).expect("Rule 0 must be set"), text);
        // found a rule that matches the whole string
        remainders.iter().find(|r| r.is_empty()).is_some()
    }

    fn check_all<'a>(&self, all: &'a [String]) -> Vec<&'a String> {
        all.iter().filter(|t| self.check(t)).collect()
    }
}

impl FromStr for Rules {
    type Err = Error;
    fn from_str(data: &str) -> Result<Self> {
        fn parse_rule(line: &str) -> Result<(u32, Rule)> {
            let regex = static_regex!(r"(\d+): (.*)");
            let caps = parsing::regex_captures(regex, line)?;
            Ok((
                parsing::capture_group(&caps, 1).parse()?,
                parsing::capture_group(&caps, 2).parse()?))
        }
        let rules = data.split("\n").map(|l| parse_rule(l)).collect::<Result<HashMap<_, _>>>()?;
        anyhow::ensure!(rules.contains_key(&0));
        Ok(Rules{rules})
    }
}

fn parse_data(input: &str) -> Result<(Rules, Vec<String>)> {
    let data: Vec<_> = input.split("\n\n").collect();
    anyhow::ensure!(data.len() == 2);
    Ok((data[0].parse()?, data[1].split("\n").map(|s|s.to_string()).collect()))
}

fn read_data() -> Result<(Rules, Vec<String>)> {
    parse_data(include_str!("../data/day19.txt").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_example1() -> (Rules, Vec<String>) {
        parse_data(include_str!("../data/day19_example1.txt").trim()).unwrap()
    }

    fn parse_example2() -> (Rules, Vec<String>) {
        parse_data(include_str!("../data/day19_example2.txt").trim()).unwrap()
    }

    #[test]
    fn example1() {
        let (mut rules, texts) = parse_example1();

        let expected = vec!("ababbb", "abbbab");
        assert_eq!(rules.check_all(&texts), expected);
        rules.reduce();
        assert_eq!(rules.check_all(&texts), expected);
        rules.make_recursive();
        assert_eq!(rules.check_all(&texts), expected); // no change for example one
    }

    #[test]
    fn example2() {
        let (mut rules, texts) = parse_example2();

        let expected = vec!("bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba");
        assert_eq!(rules.check_all(&texts), expected);
        rules.reduce();
        assert_eq!(rules.check_all(&texts), expected);

        rules.make_recursive();
        let expected = vec!(
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba");
        assert_eq!(rules.check_all(&texts), expected);
    }

    #[test]
    fn parse_file() {
        read_data().unwrap();
    }
}
