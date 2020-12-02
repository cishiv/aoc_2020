use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let lines = file_to_vec("c:/Users/mshiv/Desktop/Development/aoc_2020/day_1/src/input")?;

    // iter docs indicates that iteration consumes the collection, not sure what that means
    'outer: for line_outer in lines.iter() {
        for line_inner in lines.iter() {
            // parse to int
            let a = line_outer.parse::<i32>().unwrap();
            let b = line_inner.parse::<i32>().unwrap();
            if (a + b) == 2020 {
                println!("Part 1: {}", a * b);
                break 'outer;
            }
        }
    }

    // o(n^3) :) 
    'n: for line_n in lines.iter() {
            for line_m in lines.iter() {
                for line_o in lines.iter() {
                    let a = line_n.parse::<i32>().unwrap();
                    let b = line_m.parse::<i32>().unwrap();
                    let c = line_o.parse::<i32>().unwrap();
                    if (a + b + c) == 2020 {
                        println!("Part 2: {}", a * b * c);
                        break 'n;
                    }
                }
            }
        }

    Ok(())
}


fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    // ? -> immediately return err if fails
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

