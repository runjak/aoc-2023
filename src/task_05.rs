use std::error::Error;

pub fn first() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("05-1:");
    first()?;

    Ok(())
}
