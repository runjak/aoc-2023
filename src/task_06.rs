use std::{error::Error, fs};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn parse_input(contents: String) -> Option<Vec<Race>> {
    let lines = contents.lines().collect::<Vec<_>>();
    let [line1, line2] = lines.as_slice() else {
        return None;
    };

    if !line1.starts_with("Time:") || !line2.starts_with("Distance:") {
        return None;
    }

    let times = line1
        .split(" ")
        .filter_map(|token| token.parse::<i32>().ok());
    let distances = line2
        .split(" ")
        .filter_map(|token| token.parse::<i32>().ok());

    Some(
        times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    )
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/06/example-1.txt", "./inputs/06/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let races = parse_input(contents);

        println!("{:?}", races);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("06-1:");
    first()?;
    println!("06-2:");
    second()?;

    Ok(())
}
