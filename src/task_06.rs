use std::{error::Error, fs};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn parse_input(contents: String) -> Vec<Race> {
    let lines = contents.lines().collect::<Vec<_>>();
    let [line1, line2] = lines.as_slice() else {
        return Vec::new();
    };

    if !line1.starts_with("Time:") || !line2.starts_with("Distance:") {
        return Vec::new();
    }

    let times = line1
        .split(" ")
        .filter_map(|token| token.parse::<i32>().ok());
    let distances = line2
        .split(" ")
        .filter_map(|token| token.parse::<i32>().ok());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn count_ways_to_beat(race: &Race) -> i32 {
    let mut count = 0;

    for speed in 0..race.time {
        let remaining_time = race.time - speed;
        let distance = remaining_time * speed;

        if distance > race.distance {
            count += 1;
        }
    }

    return count;
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/06/example-1.txt", "./inputs/06/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let races = parse_input(contents);

        let solution = races.iter().map(count_ways_to_beat).product::<i32>();
        println!("Product of winnable races: {}", solution);
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
