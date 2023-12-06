use std::{error::Error, fs};

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_input_1(contents: String) -> Vec<Race> {
    let lines = contents.lines().collect::<Vec<_>>();
    let [line1, line2] = lines.as_slice() else {
        return Vec::new();
    };

    if !line1.starts_with("Time:") || !line2.starts_with("Distance:") {
        return Vec::new();
    }

    let times = line1
        .split(" ")
        .filter_map(|token| token.parse::<i64>().ok());
    let distances = line2
        .split(" ")
        .filter_map(|token| token.parse::<i64>().ok());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn count_ways_to_beat(race: &Race) -> i64 {
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
        let races = parse_input_1(contents);

        let solution = races.iter().map(count_ways_to_beat).product::<i64>();
        println!("Product of winnable races: {}", solution);
    }

    Ok(())
}

fn parse_input_2(contents: String) -> Option<Race> {
    let lines = contents
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .filter_map(|digits| digits.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let [time, distance] = lines.as_slice() else {
        return None;
    };

    Some(Race {
        time: *time,
        distance: *distance,
    })
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/06/example-1.txt", "./inputs/06/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let race = parse_input_2(contents).unwrap();

        let solution = count_ways_to_beat(&race);
        println!("Ways to win the race: {}", solution);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("06-1:");
    first()?;
    println!("06-2:");
    second()?;

    Ok(())
}
