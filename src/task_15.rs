use std::{error::Error, fs};

fn hash(input: &str) -> u8 {
    let mut current_value: u8 = 0;

    let ascii_chars = input.chars().filter_map(|c| u8::try_from(c).ok());

    for c in ascii_chars {
        current_value = current_value.wrapping_add(c);
        current_value = current_value.wrapping_mul(17);
    }

    current_value
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/15/example-1.txt", "./inputs/15/input.txt"];

    for path in paths {
        println!("Handling file: {}", path);

        let contents = fs::read_to_string(path)?;
        let contents = contents
            .split(",")
            .map(|item| item.to_string())
            .collect::<Vec<_>>();

        let sum: u32 = contents.iter().map(|c| u32::from(hash(c))).sum();
        println!("Sum: {}", sum);
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

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn hash_should_hash_single_example_string() {
        let expected: u8 = 52;
        let actual = hash("HASH");

        assert_eq!(actual, expected);
    }
}
