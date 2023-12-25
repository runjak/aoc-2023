use std::{
    collections::{HashMap, LinkedList},
    error::Error,
    fs,
};

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

#[derive(Debug, Clone, Copy)]
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

fn input_per_box(input: &Input) -> HashMap<u8, Input> {
    let mut per_box: HashMap<u8, Input> = HashMap::new();

    for (label, command) in input {
        let box_number = hash(label);

        match per_box.get_mut(&box_number) {
            Some(existing) => {
                existing.push((label.to_string(), *command));
            }
            None => {
                per_box.insert(box_number, Vec::from([(label.to_string(), *command)]));
            }
        }
    }

    per_box
}

type Slot = (String, i32);
type Slots = Vec<Slot>;

fn apply_input(input: &Input) -> Slots {
    let mut slots: Slots = Vec::new();

    for (label, command) in input {
        match command {
            Command::Remove => {
                slots = slots
                    .iter()
                    .filter(|(existing_label, _)| !existing_label.eq(label))
                    .map(|(label, lens)| (label.to_string(), *lens))
                    .collect();
            }
            Command::Set(lens) => {
                let mut found = false;

                slots = slots
                    .iter()
                    .map(|(existing_label, existing_lens)| -> Slot {
                        if existing_label == label {
                            found = true;

                            (existing_label.to_string(), *lens)
                        } else {
                            (existing_label.to_string(), *existing_lens)
                        }
                    })
                    .collect();

                if !found {
                    slots.push((label.to_string(), *lens));
                }
            }
        }
    }

    slots
}

fn focusing_power(input: &Input) -> i32 {
    let per_box = input_per_box(input);
    let mut power: i32 = 0;

    for (box_number, input) in per_box {
        let box_number = i32::try_from(box_number).unwrap() + 1;
        let slots = apply_input(&input);

        for (slot_number, (_, lens)) in slots.iter().enumerate() {
            let slot_number = i32::try_from(slot_number).unwrap() + 1;

            power += box_number * slot_number * *lens;
        }
    }

    power
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/15/example-1.txt", "./inputs/15/input.txt"];

    for path in paths {
        println!("Handling file: {}", path);

        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let power = focusing_power(&input);

        println!("Computed focusing power: {}", power);
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
