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

#[derive(Debug)]
enum Command {
    Remove,
    Set(i32),
}

type Input = Vec<(String, Command)>;

fn parse_input(contents: String) -> Input {
    contents
        .split(",")
        .filter_map(|chunk| -> Option<(String, Command)> {
            if chunk.ends_with("-") {
                let label = chunk[0..chunk.len() - 1].to_string();

                return Some((label, Command::Remove));
            }

            let Some((label, lens)) = chunk.split_once("=") else {
                return None;
            };

            let label = label.to_string();
            let lens = lens.parse::<i32>().ok()?;

            Some((label, Command::Set(lens)))
        })
        .collect()
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/15/example-1.txt", "./inputs/15/input.txt"];

    for path in paths {
        println!("Handling file: {}", path);

        let contents = fs::read_to_string(path)?;
        let contents = parse_input(contents);

        println!("Got input:\n  {:?}", contents);

        break;
    }

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
