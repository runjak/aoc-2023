use std::{collections::HashMap, error::Error, fs};

type Coordinate = (i32, i32);
type Input = HashMap<Coordinate, char>;

fn first() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./inputs/11/example-1.txt",
        "./inputs/11/input.txt",
    ];

    println!("To be implemented.");

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("11-1:");
    first()?;
    println!("11-2:");
    second()?;

    Ok(())
}
