use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let lines = file_to_vec("c:/Users/mshiv/Desktop/Development/aoc_2020/day_3/src/input")?;
    // Part 1
    
    let mut count: u32 = 0;
    let mut pos: usize = 3;
    let mut iter = lines.iter();
    let size = iter.next().unwrap().chars().count();
    for line in iter {
        if line.chars().nth(pos).unwrap() == '#' {
            count += 1;
        }
        // this will wrap if it exceeds the length of the line (Doh!)
        pos = (pos + 3) % size;
    }
    println!("Part 1: {}", count);
    
    // Part 2
    let vectors: [(usize, usize);5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // current is the representation of (gradient:trees)
    // for each (x, y) gradient in vectors, there is an associated current (vectoridx, trees_encountered)
    let mut trees_encountered: [(usize, usize);5] = [(0,0);5];
    for (line_number, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().map(|byte| byte as char).collect();
        // zip 'zips' two iterators into a single iterator of pairs
        // i.e. the iterator (current.iter_mut) and then vectors.iter()
        // are combined to return (state, vector) [a tuple on every iteration]
        for (curr, vector) in trees_encountered.iter_mut().zip(vectors.iter()) {
            // vector.1 refers to the increment element in the vector tuple
            if line_number % vector.1 == 0 {
                // if its a curr, increment the second value in the state tuple
                if chars[curr.0] == '#' {
                    curr.1 += 1;
                }
                curr.0 = (curr.0 + vector.0) % chars.len();
            }
        }
    }
    // fold applies the function provided and produces a single final value, the first argument is the inital value
    println!("Part 2: {}", trees_encountered.iter().fold(1, |result, curr| result * curr.1 as u32));
    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    // ? -> immediately return err if fails
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}