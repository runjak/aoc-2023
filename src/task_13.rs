use std::error::Error;

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/13/example-1.txt", "./inputs/13/input.txt"];

    for path in paths {
        println!("File {}", path);

        println!("To be implemented.");
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

