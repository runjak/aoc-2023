use std::{error::Error, fs};

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/01/example-1.txt", "./inputs/01/input.txt"];

    for path in paths {
        let file = fs::read_to_string(path)?;

        println!("Reading file: {}", path);

        let sum: u32 = file
            .lines()
            .filter_map(|line| -> Option<u32> {
                let digits: Vec<_> = line
                    .chars()
                    .filter_map(|char: char| char.to_digit(10))
                    .collect();

                Some(digits.first()? * 10 + digits.last()?)
            })
            .sum();

        println!("Sum of per-line values: {}", sum);
    }

    Ok(())
}

fn find_digits(line: &str) -> Vec<u32> {
    let cases: Vec<(&str, u32)> = Vec::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut line = &line[..];
    let mut digits = Vec::new();

    while line.len() > 0 {
        for (prefix, digit) in &cases {
            if line.starts_with(*prefix) {
                digits.push(*digit);
                break;
            }
        }

        line = &line[1..];
    }

    return digits;
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/01/example-2.txt", "./inputs/01/input.txt"];

    for path in paths {
        let file = fs::read_to_string(path)?;

        println!("Reading file: {}", path);
        let sum: u32 = file
            .lines()
            .filter_map(|line| -> Option<u32> {
                let digits = find_digits(line);

                Some(digits.first()? * 10 + digits.last()?)
            })
            .sum();

        println!("Sum of per-line values: {}", sum);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("01-1:");
    first()?;
    println!("01-2:");
    second()
}
