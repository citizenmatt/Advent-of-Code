use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use regex::Regex;

pub struct Passport {
    entries: HashMap<String, String>
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Passport> {
    let mut passports = vec![];

    let mut entries = HashMap::new();

    for line in input.lines() {
        if !line.is_empty() {
            for entry in line.split(' ') {
                let s: Vec<&str> = entry.split(':').collect();
                entries.insert(s[0].to_string(), s[1].to_string());
            }
        }
        else {
            passports.push(Passport {
                entries
            });
            entries = HashMap::new()
        }
    }

    passports.push(Passport {
        entries
    });

    return passports
}

#[aoc(day4, part1)]
pub fn part1(passports: &Vec<Passport>) -> usize {
    return passports.iter().filter(|p| {
        p.entries.keys().count() == 8
        || (p.entries.keys().count() == 7 && !p.entries.contains_key("cid"))
    }).count()
}

#[aoc(day4, part2)]
pub fn part2(passports: &Vec<Passport>) -> usize {
    return passports.iter().filter(|p| {
        if p.entries.keys().count() == 8
            || (p.entries.keys().count() == 7 && !p.entries.contains_key("cid")) {

            return p.entries.iter().all(|(k,v)| {
                return match k.as_str() {
                    "byr" => match_year(v, 1920, 2002),
                    "iyr" => match_year(v, 2010, 2020),
                    "eyr" => match_year(v, 2020, 2030),
                    "hgt" => match_height(v),
                    "hcl" => matches(v, r"^\#([0-9a-f]){6}$"),
                    "ecl" => matches(v, r"^(amb|blu|brn|gry|grn|hzl|oth)$"),
                    "pid" => matches(v, r"^\d{9}$"),
                    "cid" => true,
                    _ => false
                }
            })
        }
        return false
    }).count()
}

fn match_year(value: &String, min: i32, max: i32) -> bool {
    matches(value, r"\d{4}") && match_num(value, min, max)
}

fn match_height(value: &String) -> bool {
    let regex = Regex::new(r"(?P<height>\d+)(?P<unit>cm|in)").unwrap();
    return regex.captures(value).map_or(false, |c| {
        return match &c["unit"] {
            "cm" => match_num(&c["height"], 150, 193),
            "in" => match_num(&c["height"], 59, 76),
            _ => false
        }
    })
}

fn match_num(value: &str, min: i32, max: i32) -> bool {
    let v: i32 = value.parse().unwrap();
    return v >= min && v <= max
}

fn matches(value: &String, re: &str) -> bool {
    Regex::new(re).unwrap().is_match(value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let input = parse_input(input);

        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn example_part2() {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let input = parse_input(input);

        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day4.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), 256);
        assert_eq!(part2(&input), 198);
    }
}
