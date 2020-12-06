use std::collections::HashSet;

pub fn advent() {
    let groups = parse_data();
    let qs: usize = groups.iter().map(|g| questions(g)).sum();
    println!("All Answers: {}", qs);
    let qs: usize = groups.iter().map(|g| all_questions(g)).sum();
    println!("All Answers within each group: {}", qs);
}

fn parse_data() -> Vec<&'static str> {
    return include_str!("../data/day06.txt").trim().split("\n\n").collect();
}

fn questions(group: &str) -> usize {
    group.chars().filter(|c| *c != '\n').collect::<HashSet<_>>().len()
}

fn all_questions(group: &str) -> usize {
    let people: Vec<_> = group.split("\n").map(|p| p.chars().collect::<HashSet<_>>()).collect();
    let all_answered: HashSet<_> = group.chars().filter(|c| *c != '\n').collect();
    people.iter().fold(all_answered, |all, cur| &all & cur).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group() {
        assert_eq!(questions("abcx\nabcy\nabcz"), 6);
    }

    parameterized_test::create!{example, (group, any, all), {
        assert_eq!(questions(group), any);
        assert_eq!(all_questions(group), all);
    }}
    example!{
      a: ("abc", 3, 3),
      b: ("a\nb\nc", 3, 0),
      c: ("ab\nac", 3, 1),
      d: ("a\na\na\na", 1, 1),
      e: ("b", 1, 1),
    }

    #[test]
    fn parse_file() {
        assert!(parse_data().len() > 0);
    }
}
