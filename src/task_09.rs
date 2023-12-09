use std::{error::Error, fs};

type Z = i32;

fn parse_input(contents: String) -> Vec<Vec<Z>> {
    contents
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|digits| digits.parse().ok())
                .collect()
        })
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/09/example-1.txt", "./inputs/09/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        println!("Input:\n{:?}", input);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("09-1:");
    first()?;
    println!("09-2:");
    second()?;

    Ok(())
}
