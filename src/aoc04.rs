use crate::error::ParseError;
use regex::Regex;

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

fn valid_values(p: &str) -> Result<(), ParseError> {
    lazy_static! {
        static ref BYR_RE: Regex = Regex::new(r"byr:(\d{4})\b").unwrap();
        static ref IYR_RE: Regex = Regex::new(r"iyr:(\d{4})\b").unwrap();
        static ref EYR_RE: Regex = Regex::new(r"eyr:(\d{4})\b").unwrap();
        static ref HGT_CM_RE: Regex = Regex::new(r"hgt:(\d+)cm\b").unwrap();
        static ref HGT_IN_RE: Regex = Regex::new(r"hgt:(\d+)in\b").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"hcl:#([0-9a-f]{6})\b").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
        static ref PID_RE: Regex = Regex::new(r"pid:(\d{9})\b").unwrap();
    }

    let birth_year = capture_group!(regex_captures!(BYR_RE, p)?, 1).parse::<i32>()?;
    if birth_year < 1920 || birth_year > 2002 { return Err(ParseError::Malformed("byr".into())); }

    let issue_year = capture_group!(regex_captures!(IYR_RE, p)?, 1).parse::<i32>()?;
    if issue_year < 2010 || issue_year > 2020 { return Err(ParseError::Malformed("iyr".into())); }

    let expr_year = capture_group!(regex_captures!(EYR_RE, p)?, 1).parse::<i32>()?;
    if expr_year < 2020 || expr_year > 2030 { return Err(ParseError::Malformed("eyr".to_string())); }

    //capture_group!(regex_captures!(HGT_CM_RE, p)?, 1);
    let height_cm_capture = regex_captures!(HGT_CM_RE, p);
    let height_in_capture = regex_captures!(HGT_IN_RE, p);
    if height_cm_capture.is_ok() {
        let height_cm = capture_group!(height_cm_capture.unwrap(), 1).parse::<i32>()?;
        if height_cm < 150 || height_cm > 193 { return Err(ParseError::Malformed("hgt".to_string())); }
    } else if height_in_capture.is_ok() {
        let height_in = capture_group!(height_in_capture.unwrap(), 1).parse::<i32>()?;
        if height_in < 59 || height_in > 76 { return Err(ParseError::Malformed("hgt".to_string())); }
    } else {
        return Err(ParseError::Malformed("hgt".to_string()));
    }

    regex_captures!(HCL_RE, p)?;
    regex_captures!(ECL_RE, p)?;
    regex_captures!(PID_RE, p)?;

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

    parameterized_test::create!{invalid, (passport, err), {
      assert_eq!(valid_values(passport), Err(err));
    }}
    invalid!{
      a: ("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926", ParseError::Malformed("eyr".into())),
      b: ("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946", ParseError::Malformed("eyr".into())),
      c: ("hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
          ParseError::Malformed("`hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277` did not match `hcl:#([0-9a-f]{6})\\b`".into())),
      d: ("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007", ParseError::Malformed("byr".into())),
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
