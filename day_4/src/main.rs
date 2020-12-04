use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let required_fields: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().copied().collect();
    let req: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let count = std::fs::read_to_string("c:/Users/mshiv/Desktop/Development/aoc_2020/day_4/src/input")?
                        .split("\n\r")
                        .fold(0, |valid , pass| {
                                let fields: HashSet<&str> = pass.split_whitespace()
                                .map(|field| field.split(":").next().unwrap())
                                .collect();
                                if required_fields.is_subset(&fields) {
                                    valid + 1
                                } else {
                                    valid
                                }
                            }
                        );
        let second_count = std::fs::read_to_string("c:/Users/mshiv/Desktop/Development/aoc_2020/day_4/src/input")?
                        .split("\n\r")
                        .fold(0, |valid , pass| {
                                let fields: HashMap<_, _> = pass.split_whitespace()
                                .map(|field| {
                                    let mut parts = field.split(":");
                                    (parts.next().unwrap(), parts.next().unwrap())
                                })
                                .into_iter()
                                .collect();
                                let mut t = true;
                                for f in req.iter() {
                                    if fields.contains_key(f) {
                                        if validators(f, fields.get(f).unwrap()) {
                                            continue;
                                        } else {
                                            t = false;
                                        }
                                    } else {
                                        t = false;
                                    }
                                }
                                if t {
                                    valid + 1
                                } else {
                                    valid
                                }
                            }
                        );
    println!("Part 1 {}", count);
    println!("Part 2 {}", second_count);
    Ok(())
}

// fn file_to_buf(filename: &str) -> io::Result<BufReader> {
//     // ? -> immediately return err if fails
//     let file = File::open(filename)?;
//     let file_reader = BufReader::new(file);
//     Ok(file_reader)
// }


fn validators(field: &str, value: &str) -> bool {
    match field {
        "byr" => matches!(value.parse::<usize>(), Ok(number) if (1920..=2002).contains(&number)),
        "iyr" => matches!(value.parse::<usize>(), Ok(number) if (2010..=2020).contains(&number)),
        "eyr" => matches!(value.parse::<usize>(), Ok(number) if (2020..=2030).contains(&number)),
        "hgt" => {
            let (height, unit) = value.split_at(value.len() - 2);
            let range = match unit {
                "cm" => 150..=193,
                "in" => 59..=76,
                _ => return false
            };
            matches!(height.parse::<usize>(), Ok(number) if range.contains(&number))
        }
        "hcl" => value.starts_with('#') && value[1..].chars().all(|byte| byte.is_digit(16)),
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => value.len() == 9 && value.parse::<usize>().is_ok(),
        _ => panic!("bad field")
    }
}