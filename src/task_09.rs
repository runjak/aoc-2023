use std::error::Error;

fn first() -> Result<(), Box<dyn Error>> {
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
