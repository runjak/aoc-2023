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

fn derive(values: &Vec<Z>) -> Vec<Z> {
    let mut tail = values.iter();
    tail.next();

    values.iter().zip(tail).map(|(a, b)| b - a).collect()
}

fn all_zero(values: &Vec<Z>) -> bool {
    values.iter().all(|v| v == &0)
}

fn derives(values: &Vec<Z>) -> Vec<Vec<Z>> {
    let mut ret = Vec::new();

    let mut current_values = values.clone();
    while !all_zero(&current_values) {
        let next_values = derive(&current_values);
        ret.push(current_values);
        current_values = next_values;
    }
    ret.push(current_values);

    return ret;
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/09/example-1.txt", "./inputs/09/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        println!("Input:\n{:?}", derives(input.get(0).unwrap()));

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
