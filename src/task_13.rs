use std::{error::Error, fs};

// A pattern is made up of several lines of strings.
type Pattern = Vec<String>;

fn parse_input(contents: String) -> Vec<Pattern> {
    contents
        .split("\n\n")
        .map(|lines| -> Pattern { lines.split("\n").map(|s| s.to_string()).collect::<Vec<_>>() })
        .collect()
}

fn transpose_pattern(pattern: &Pattern) -> Pattern {
    // Inspired by https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust

    if pattern.is_empty() {
        return Vec::new();
    }

    let width = pattern[0].len();
    let mut iters = pattern
        .into_iter()
        .map(|line| line.chars())
        .collect::<Vec<_>>();

    (0..width)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|i| i.next())
                .collect::<String>()
        })
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/13/example-1.txt", "./inputs/13/input.txt"];

    for path in paths {
        println!("File {}", path);

        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        println!("Got inputs: {:?}", input);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("13-1:");
    first()?;
    println!("13-2:");
    second()?;

    Ok(())
}
