use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn main() {
    let mut i = 0;
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    let lines = lines_from_file("input.txt");

    for line in lines {
        let mut stack = String::new().to_owned();
        let instruction: &str;
        let value: i32;

        let mut j = 0;

        while j < line.len() {
            // println!("{}", line.len());
            let b: u8 = line.as_bytes()[j];
            let c: char = b as char;

            stack.push(c);
            j += 1;
        }

        instruction = &stack[..];

        if &instruction[..2] == "up" {
            value = FromStr::from_str(&instruction[3..]).unwrap();

            depth -= value;
        } else if &instruction[..4] == "down" {
            value = FromStr::from_str(&instruction[5..]).unwrap();

            depth += value;
        } else if &instruction[..7] == "forward" {
            value = FromStr::from_str(&instruction[8..]).unwrap();
            horizontal += value;
        }
        i += 1;
    }
    println!(
        "Part 1:\nHorizontal: {}\nDepth: {}\nAnswer: {}",
        horizontal,
        depth,
        horizontal * depth
    );

    part_2();
}

fn part_2() {
    let lines = lines_from_file("input.txt");

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in lines {
        let mut stack = String::new().to_owned();
        let instruction: &str;
        let value: i32;

        let mut i = 0;
        let mut j = 0;

        while j < line.len() {
            // println!("{}", line.len());
            let b: u8 = line.as_bytes()[j];
            let c: char = b as char;

            stack.push(c);
            j += 1;
        }

        instruction = &stack[..];

        if &instruction[..2] == "up" {
            value = FromStr::from_str(&instruction[3..]).unwrap();

            aim -= value;
        } else if &instruction[..4] == "down" {
            value = FromStr::from_str(&instruction[5..]).unwrap();

            aim += value;
        } else if &instruction[..7] == "forward" {
            value = FromStr::from_str(&instruction[8..]).unwrap();
            horizontal += value;
            depth += aim * value;
        }
        i += 1;
    }
    println!(
        "\nPart 2:\nHorizontal: {}\nDepth: {}\nAnswer: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
