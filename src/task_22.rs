use std::error::Error;

fn first() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("22-1:");
    first()?;
    println!("22-2:");
    second()?;

    Ok(())
}
