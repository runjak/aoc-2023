use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/01/example-1.txt", "./inputs/01/input.txt"];

    for path in paths {
        let file = fs::read_to_string(path)?;

        println!("Reading file> {}", path);

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
