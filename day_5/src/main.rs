use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main()  -> io::Result<()>  {
    let passes = file_to_vec("c:/Users/mshiv/Desktop/Development/aoc_2020/day_5/src/input")?;
    
    let mut highest_pass : usize = 0;
    let mut all : Vec<usize> = Vec::new();

    for pass in passes.iter() {
        let mut current_pass: usize = search(pass);
        println!("pass_value {}", current_pass);
        if current_pass > highest_pass {
            highest_pass = current_pass;
        }
        all.push(current_pass);
    }
    println!("Part 1: {}", highest_pass);

    // Part 2
    all.sort();
    let missing_id : usize = all.iter().enumerate().fold(0,
        |q, (i, v)| {
            if(i == 0) {
                q
            } else {
                if v - all[i-1] == 1 {
                    q
                } else {
                    v - 1
                }
            }
        }
    );
    println!("Part 2: {}", missing_id);
    Ok(())
}


fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    // ? -> immediately return err if fails
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn search(boarding_pass: &str) -> usize {
    // rows
    let cols = &boarding_pass.to_string()[7..boarding_pass.len()];
    let rows: &str = &boarding_pass.to_string()[..7];
    // println!("rows: {} cols: {}", rows, cols);
    let row = bin(rows, 0, 127, 0);
    let col = bin(cols, 0, 7, 0);
    return row * 8 + col;
}

fn bin(search_str: &str, start: usize, end: usize, current_idx: usize) -> usize {
    if current_idx == search_str.len() {
        println!("values on last iteration\ns={},e={},search={}", start, end, search_str);
        if search_str.to_string()[current_idx-1..current_idx] == "F".to_string() || search_str.to_string()[current_idx-1..current_idx] == "L".to_string() {
            println!("selected start {}", start);
            return start;
        } else {
            println!("selected end {}", end);
            return end;
        }
    } else {
        if search_str.to_string()[current_idx..current_idx + 1] == "F".to_string() || search_str.to_string()[current_idx..current_idx+1] == "L".to_string() {
            // lower
            let mut range: usize = end - start;
            let mut next_end = start + (range / 2);
            bin(search_str, start, next_end, current_idx + 1)
        } else {
            // upper
            let mut range: usize = end - start;
            let mut next_start = end - (range / 2);
            bin(search_str, next_start, end, current_idx + 1)
        }
    }
}