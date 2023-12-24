use std::{error::Error, fs};

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/15/example-1.txt", "./inputs/15/input.txt"];

    for path in paths {
        println!("Handling file: {}", path);

        let contents = fs::read_to_string(path)?;
        let contents = contents
            .split(",")
            .map(|item| item.to_string())
            .collect::<Vec<_>>();

        println!("Got items: {:?}", contents);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("15-1:");
    first()?;
    println!("15-2:");
    second()?;

    Ok(())
}
