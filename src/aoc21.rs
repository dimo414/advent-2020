use std::collections::{HashSet, HashMap, BTreeMap};
use std::str::FromStr;
use anyhow::{Error,Result};
use crate::parsing::*;

pub fn advent() {
    let food = parse_data().unwrap();
    let candidates = associate_allergens(&food);

    let safe_ingredients: HashSet<_> = safe_ingredients(&food, &candidates);

    println!("Safe ingredient usages: {}", food.iter()
        .flat_map(|f| f.ingredients.iter()).filter(|i| safe_ingredients.contains(i)).count());

    let dangerous = reduce_candidates(candidates);
    println!("Dangerous ingredients: {}", dangerous.values().map(|s|s.to_string()).collect::<Vec<_>>().join(","));
}

fn associate_allergens(food: &[Food]) -> HashMap<String, HashSet<String>> {
    let all_ingredients: HashSet<_> = food.iter().flat_map(|f| f.ingredients.iter()).collect();
    let mut candidates: HashMap<_, _> =
        food.iter().flat_map(|f| f.allergens.iter()).map(|a| (a, all_ingredients.clone())).collect();
    for item in food.iter() {
        for allergen in item.allergens.iter() {
            candidates.get_mut(&allergen).unwrap().retain(|&i| item.ingredients.contains(i));
        }
    }
    candidates.into_iter()
        .map(|(a, i)| (a.to_string(), i.iter().map(|i| i.to_string()).collect())).collect()
}

fn safe_ingredients<'a>(food: &'a [Food], candidates: &'a HashMap<String, HashSet<String>>) -> HashSet<&'a String> {
    let mut ret: HashSet<_> = food.iter().flat_map(|f| f.ingredients.iter()).collect();
    for unsafe_ingr in candidates.values().flat_map(|v| v.iter()) {
        ret.remove(&unsafe_ingr);
    }
    ret
}

fn reduce_candidates(mut candidates: HashMap<String, HashSet<String>>) -> BTreeMap<String, String> {
    let mut ret = BTreeMap::new();
    while !candidates.is_empty() {
        let singletons: HashMap<_,_> = candidates.iter().filter(|(_, v)| v.len() == 1).collect();
        let to_remove: HashSet<_> =
            singletons.values().flat_map(|v| v.iter().map(|s|s.to_string())).collect();

        ret.extend(singletons.iter()
            .map(|(k,v)|(k.to_string(), v.iter().next().unwrap().to_string())));

        candidates.values_mut().for_each(|v| v.retain(|i|!to_remove.contains(i)));
        candidates = candidates.into_iter().filter(|(_, v)| !v.is_empty()).collect();
    }
    ret
}

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self> {
        let regex = static_regex!(r"(.*) \(contains (.*)\)");
        let caps = regex_captures(regex, &line)?;
        let ingredients = capture_group(&caps, 1).split(" ").map(|s|s.to_string()).collect();
        let allergens = capture_group(&caps, 2).split(", ").map(|s|s.to_string()).collect();
        Ok(Food{ingredients, allergens})
    }
}

fn parse_data() -> Result<Vec<Food>> {
    include_str!("../data/day21.txt").trim().split("\n").map(|s|s.parse()).collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example() {
        let food: Vec<Food> = vec!(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)")
            .iter().map(|s|s.parse().unwrap()).collect();

        let candidates = associate_allergens(&food);
        let expected: HashMap<String, HashSet<String>> =
            vec!(("dairy", vec!("mxmxvkd")), ("soy", vec!("sqjhc", "fvjkl")), ("fish", vec!("sqjhc", "mxmxvkd")))
                .into_iter().map(|(k, v)| (k.to_string(), v.iter().map(|s|s.to_string()).collect()))
                .collect();
        assert_eq!(candidates, expected);

        let safe: HashSet<_> = safe_ingredients(&food, &candidates).iter().map(|s|s.to_string()).collect();
        let expected = vec!("trh", "sbzzf", "kfcds", "nhms").iter().map(|&s|s.to_string()).collect();
        assert_eq!(safe, expected);

        let dangerous = reduce_candidates(candidates);
        let expected = vec!(("dairy", "mxmxvkd"), ("fish", "sqjhc"), ("soy", "fvjkl")).iter()
            .map(|(k, v)| (k.to_string(), v.to_string())).collect();
        assert_eq!(dangerous, expected);
    }

    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}
