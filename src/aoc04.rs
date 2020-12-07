use anyhow::{bail, ensure, Context, Result};
use crate::parsing;

pub fn advent() {
    let passports = parse_data();
    println!("Valid Fields: {}", passports.iter().filter(|p| valid_fields(p)).count());
    println!("Valid Values: {}", passports.iter().filter(|p| valid_values(p).is_ok()).count());
}

fn parse_data() -> Vec<String> {
    include_str!("../data/day04.txt").split("\n\n").map(|s| s.to_string()).collect()
}

fn valid_fields(p: &str) -> bool {
    p.contains("byr:") &&
        p.contains("iyr:") &&
        p.contains("eyr:") &&
        p.contains("hgt:") &&
        p.contains("hcl:") &&
        p.contains("ecl:") &&
        p.contains("pid:")
}

fn valid_values(p: &str) -> Result<()> {
    let byr_regex = static_regex!(r"byr:(\d{4})\b");
    let iyr_regex = static_regex!(r"iyr:(\d{4})\b");
    let eyr_regex = static_regex!(r"eyr:(\d{4})\b");
    let hgt_cm_regex = static_regex!(r"hgt:(\d+)cm\b");
    let hgt_in_regex = static_regex!(r"hgt:(\d+)in\b");
    let hcl_regex = static_regex!(r"hcl:#([0-9a-f]{6})\b");
    let ecl_regex = static_regex!(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b");
    let pid_regex = static_regex!(r"pid:(\d{9})\b");

    // TODO annotate all Err results .with_context()
    let birth_year = parsing::capture_group(&parsing::regex_captures(&byr_regex, p)?, 1).parse::<i32>()
        .with_context(|| p.to_string())?;
    ensure!(birth_year >= 1920 && birth_year <= 2002, "byr");

    let issue_year = parsing::capture_group(&parsing::regex_captures(&iyr_regex, p)?, 1).parse::<i32>()?;
    ensure!(issue_year >= 2010 && issue_year <= 2020, "iyr");

    let expr_year = parsing::capture_group(&parsing::regex_captures(&eyr_regex, p)?, 1).parse::<i32>()?;
    ensure!(expr_year >= 2020 && expr_year <= 2030, "eyr");

    let height_cm_capture = parsing::regex_captures(&hgt_cm_regex, p);
    let height_in_capture = parsing::regex_captures(&hgt_in_regex, p);
    if height_cm_capture.is_ok() {
        let height_cm = parsing::capture_group(&height_cm_capture.unwrap(), 1).parse::<i32>()?;
        ensure!(height_cm >= 150 && height_cm <= 193, "hgt");
    } else if height_in_capture.is_ok() {
        let height_in = parsing::capture_group(&height_in_capture.unwrap(), 1).parse::<i32>()?;
        ensure!(height_in >= 59 && height_in <= 76, "hgt");
    } else {
        bail!("hgt");
    }

    parsing::regex_captures(&hcl_regex, p)?;
    parsing::regex_captures(&ecl_regex, p)?;
    parsing::regex_captures(&pid_regex, p)?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{validate_fields, (passport, valid), {
      assert_eq!(valid_fields(passport), valid);
    }}
    validate_fields!{
      valid: ("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm", true),
      missing_hgt: ("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929", false),
      missing_cid: ("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm", true),
      missing_byr: ("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in", false),
    }

    parameterized_test::create!{invalid, passport, {
      // TODO assert on the exact error
      assert!(valid_values(passport).is_err());
    }}
    invalid!{
      a: "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
      b: "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946",
      c: "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
      d: "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007",
    }

    parameterized_test::create!{valid, passport, {
      valid_values(passport).unwrap();
    }}
    valid!{
      a: "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f",
      b: "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
      c: "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
      d: "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    }

    #[test]
    fn parse_file() {
        assert!(!parse_data().is_empty());
    }
}
