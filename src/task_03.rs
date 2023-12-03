use std::{error::Error, fs};

struct Coordinate {
    line_index: usize,
    char_index: usize,
}

struct PartNumber {
    number: u32,
    coordinates: Vec<Coordinate>,
}

struct Symbol {
    label: String,
    coordinates: Vec<Coordinate>,
}

struct Schematic {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

fn span_digit_parts(digit_parts: &[(usize, char)]) -> (&[(usize, char)], &[(usize, char)]) {
    if digit_parts.len() < 2 {
        return (digit_parts, &[]);
    }

    let mut cut: usize = 0;
    for (index, current_digit_part) in digit_parts.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let previous_digit_part = digit_parts[index - 1];
        if current_digit_part.0 != previous_digit_part.0 + 1 {
            cut = index;
            break;
        }
    }

    if cut <= 0 {
        return (digit_parts, &[]);
    }

    return (&digit_parts[..cut], &digit_parts[cut..]);
}

fn parse_schematic(input: &str) -> Schematic {
    return input
        .lines()
        .enumerate()
        .map(|(line_index, line)| -> Schematic {
            let (digit_parts, symbols): (Vec<_>, Vec<_>) = line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .partition(|line_part| line_part.1.is_digit(10));

            let symbols: Vec<_> = symbols
                .iter()
                .map(|(char_index, label)| -> Symbol {
                    Symbol {
                        label: label.to_string(),
                        coordinates: Vec::from([Coordinate {
                            line_index,
                            char_index: *char_index,
                        }]),
                    }
                })
                .collect();

            let mut digit_parts = digit_parts.as_slice();
            let mut part_numbers: Vec<PartNumber> = Vec::new();

            while digit_parts.len() > 0 {
                let span = span_digit_parts(digit_parts);
                let current_parts = span.0;
                digit_parts = span.1;

                part_numbers.push(PartNumber {
                    number: digit_parts
                        .iter()
                        .map(|(_, digit)| digit)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                    coordinates: digit_parts
                        .iter()
                        .map(|(char_index, _)| -> Coordinate {
                            Coordinate {
                                line_index,
                                char_index: *char_index,
                            }
                        })
                        .collect(),
                })
            }

            Schematic {
                part_numbers,
                symbols,
            }
        })
        .fold(
            Schematic {
                part_numbers: Vec::new(),
                symbols: Vec::new(),
            },
            |a, b| {
                let part_numbers = a.part_numbers.iter().chain(b.part_numbers.iter()).collect();
                let symbols: Vec<_> = a.symbols.iter().chain(b.symbols.iter()).collect();

                return Schematic {
                    part_numbers,
                    symbols,
                };
            },
        );
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/03/example-1.txt", "./inputs/03/input.txt"];

    for path in paths {
        println!("File {}:", path);
        let contents = fs::read_to_string(path)?;

        println!("{}", contents);

        break;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("03-1:");
    first()
}
