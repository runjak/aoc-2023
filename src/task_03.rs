use std::{error::Error, fs};

#[derive(Clone, PartialEq, Debug)]
struct Coordinate {
    line_index: usize,
    char_index: usize,
}

#[derive(Clone, PartialEq, Debug)]
struct PartNumber {
    number: u32,
    coordinates: Vec<Coordinate>,
}

#[derive(Clone, PartialEq, Debug)]
struct Symbol {
    label: String,
    coordinates: Vec<Coordinate>,
}

#[derive(PartialEq, Debug)]
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

fn parse_schematic(input: String) -> Schematic {
    let (part_numbers_per_line, symbols_per_line): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .map(|(line_index, line)| -> (Vec<PartNumber>, Vec<Symbol>) {
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
                    number: current_parts
                        .iter()
                        .map(|(_, digit)| digit)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                    coordinates: current_parts
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

            return (part_numbers, symbols);
        })
        .unzip();

    Schematic {
        part_numbers: part_numbers_per_line.concat(),
        symbols: symbols_per_line.concat(),
    }
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

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use crate::task_03::{PartNumber,parse_schematic, Schematic, Coordinate, Symbol};

    #[test]
    fn should_parse_example_as_expected() -> Result<(), Box<dyn Error>> {
        let example_data = fs::read_to_string("./inputs/03/example-1.txt")?;

        let expected = Schematic {
            part_numbers: [
                PartNumber {
                    number: 467,
                    coordinates: [
                        Coordinate {
                            line_index: 0,
                            char_index: 0,
                        },
                        Coordinate {
                            line_index: 0,
                            char_index: 1,
                        },
                        Coordinate {
                            line_index: 0,
                            char_index: 2,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 114,
                    coordinates: [
                        Coordinate {
                            line_index: 0,
                            char_index: 5,
                        },
                        Coordinate {
                            line_index: 0,
                            char_index: 6,
                        },
                        Coordinate {
                            line_index: 0,
                            char_index: 7,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 35,
                    coordinates: [
                        Coordinate {
                            line_index: 2,
                            char_index: 2,
                        },
                        Coordinate {
                            line_index: 2,
                            char_index: 3,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 633,
                    coordinates: [
                        Coordinate {
                            line_index: 2,
                            char_index: 6,
                        },
                        Coordinate {
                            line_index: 2,
                            char_index: 7,
                        },
                        Coordinate {
                            line_index: 2,
                            char_index: 8,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 617,
                    coordinates: [
                        Coordinate {
                            line_index: 4,
                            char_index: 0,
                        },
                        Coordinate {
                            line_index: 4,
                            char_index: 1,
                        },
                        Coordinate {
                            line_index: 4,
                            char_index: 2,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 58,
                    coordinates: [
                        Coordinate {
                            line_index: 5,
                            char_index: 7,
                        },
                        Coordinate {
                            line_index: 5,
                            char_index: 8,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 592,
                    coordinates: [
                        Coordinate {
                            line_index: 6,
                            char_index: 2,
                        },
                        Coordinate {
                            line_index: 6,
                            char_index: 3,
                        },
                        Coordinate {
                            line_index: 6,
                            char_index: 4,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 755,
                    coordinates: [
                        Coordinate {
                            line_index: 7,
                            char_index: 6,
                        },
                        Coordinate {
                            line_index: 7,
                            char_index: 7,
                        },
                        Coordinate {
                            line_index: 7,
                            char_index: 8,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 664,
                    coordinates: [
                        Coordinate {
                            line_index: 9,
                            char_index: 1,
                        },
                        Coordinate {
                            line_index: 9,
                            char_index: 2,
                        },
                        Coordinate {
                            line_index: 9,
                            char_index: 3,
                        },
                    ].to_vec(),
                },
                PartNumber {
                    number: 598,
                    coordinates: [
                        Coordinate {
                            line_index: 9,
                            char_index: 5,
                        },
                        Coordinate {
                            line_index: 9,
                            char_index: 6,
                        },
                        Coordinate {
                            line_index: 9,
                            char_index: 7,
                        },
                    ].to_vec(),
                },
            ].to_vec(),
            symbols: [
                Symbol {
                    label: "*".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 1,
                        char_index: 3,
                    }].to_vec(),
                },
                Symbol {
                    label: "#".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 3,
                        char_index: 6,
                    }].to_vec(),
                },
                Symbol {
                    label: "*".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 4,
                        char_index: 3,
                    }].to_vec(),
                },
                Symbol {
                    label: "+".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 5,
                        char_index: 5,
                    }].to_vec(),
                },
                Symbol {
                    label: "$".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 8,
                        char_index: 3,
                    }].to_vec(),
                },
                Symbol {
                    label: "*".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 8,
                        char_index: 5,
                    }].to_vec(),
                },
            ].to_vec(),
        };

        let actual = parse_schematic(example_data);

        assert_eq!(actual, expected);

        Ok(())
    }
}
