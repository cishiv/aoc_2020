use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let lines = file_to_vec("c:/Users/mshiv/Desktop/Development/aoc_2020/day_2/src/input")?;
    let mut count = 0;
    let mut second_count = 0;

    for line in lines.iter() {
        // iterate over the passwords and policies
        let splits = line.split_whitespace();
        let password_and_policy = splits.collect::<Vec<&str>>();
        // idx 0 = policy, idx 1 = character, idx 2 = password
        // how many times does string 2 contain string 1?
        // break down the policy into a range

        let policy = password_and_policy[0].split("-").collect::<Vec<&str>>();

        let r_0 = policy[0].parse::<usize>().unwrap();
        let r_1 = policy[1].parse::<usize>().unwrap();

        let char_vec : Vec<char> = password_and_policy[2].chars().collect();
        let ch_target = password_and_policy[1].chars().nth(0).unwrap();
        let mut c = 0;
        
        // XOR
        if !((password_and_policy[2].chars().nth(r_0-1).unwrap() == ch_target) == (password_and_policy[2].chars().nth(r_1-1).unwrap() == ch_target)) {
            second_count = second_count + 1;
        }

        for ch in char_vec {
            if ch == ch_target {
                c = c + 1;
            }
        }
        if c >= r_0 && c <= r_1 {
            count = count + 1;
        }
    }

    println!("Part 1 {}", count);
    println!("Part 2 {}", second_count);
    Ok(())
}


fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    // ? -> immediately return err if fails
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
