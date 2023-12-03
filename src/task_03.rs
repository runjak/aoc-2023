use std::{error::Error, fs};

/*
 * Despite my intention to use structs in a Haskell-like manner
 * with structural equality and immutability
 * it appeared that I required Clone to get concat() to work.
 *
 * I'd love to better understand the underlying situation here and how to clean it up.
 */

#[derive(Clone, PartialEq, Debug)]
struct Coordinate {
    line_index: i32,
    char_index: i32,
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

/*
 * span_digit_parts is a helper function for parse_schematic.
 *
 * The intention is to aid in grouping digit_parts for the construction of the part_numbers of a Schematic.
 * digit_parts are given as a alice of tuples of char_index and char for a line.
 * span_digit_parts is not concerned with filtering out chars by character class,
 * but instead only looks at the char_index and cuts when there's a hole between indices.
 *
 * The return value is a tuple of the first consecutive run of digits, followed by remaining digits (to be cut later).
 *
 * I'd have prefered something like a group_by here, but did not manage to get that working.
 */
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
                            line_index: line_index.try_into().unwrap(),
                            char_index: (*char_index).try_into().unwrap(),
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
                                line_index: line_index.try_into().unwrap(),
                                char_index: (*char_index).try_into().unwrap(),
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

fn neighbours(coordinates: &Vec<Coordinate>) -> Vec<Coordinate> {
    coordinates
        .iter()
        .flat_map(|center| {
            Vec::from([
                Coordinate {
                    line_index: center.line_index - 1,
                    char_index: center.char_index - 1,
                },
                Coordinate {
                    line_index: center.line_index - 1,
                    char_index: center.char_index,
                },
                Coordinate {
                    line_index: center.line_index - 1,
                    char_index: center.char_index + 1,
                },
                Coordinate {
                    line_index: center.line_index,
                    char_index: center.char_index - 1,
                },
                Coordinate {
                    line_index: center.line_index,
                    char_index: center.char_index + 1,
                },
                Coordinate {
                    line_index: center.line_index + 1,
                    char_index: center.char_index - 1,
                },
                Coordinate {
                    line_index: center.line_index + 1,
                    char_index: center.char_index,
                },
                Coordinate {
                    line_index: center.line_index + 1,
                    char_index: center.char_index + 1,
                },
            ])
        })
        .filter(|candidate| coordinates.iter().all(|existing| candidate != existing))
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/03/example-1.txt", "./inputs/03/input.txt"];

    for path in paths {
        println!("File {}:", path);
        let contents = fs::read_to_string(path)?;
        let schematic = parse_schematic(contents);

        let symbol_coordinates: Vec<_> = schematic
            .symbols
            .iter()
            .flat_map(|symbol| symbol.coordinates.iter())
            .collect();

        let part_numbers_with_symbol = schematic.part_numbers.iter().filter(|part_number| {
            let neighbour_coordinates = neighbours(&part_number.coordinates);

            return neighbour_coordinates.iter().any(|neighbour_coordinate| {
                symbol_coordinates
                    .iter()
                    .any(|symbol_coordinate| neighbour_coordinate == *symbol_coordinate)
            });
        });

        let sum: u32 = part_numbers_with_symbol
            .map(|part_number| part_number.number)
            .sum();

        println!("Sum of part numbers with symbols: {}", sum);
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/03/example-1.txt", "./inputs/03/input.txt"];

    for path in paths {
        println!("File {}:", path);
        let contents = fs::read_to_string(path)?;
        let schematic = parse_schematic(contents);

        let gears = schematic
            .symbols
            .iter()
            .filter(|symbol| symbol.label == "*");

        let gear_part_numbers = gears
            .map(|gear| {
                let gear_neighbours = neighbours(&gear.coordinates);

                let adjacent_part_numbers: Vec<_> = schematic
                    .part_numbers
                    .iter()
                    .filter(|part_number| {
                        part_number.coordinates.iter().any(|part_coordinate| {
                            gear_neighbours.iter().any(|gear_neighbour_coordinate| {
                                part_coordinate == gear_neighbour_coordinate
                            })
                        })
                    })
                    .collect();

                return adjacent_part_numbers;
            })
            .filter(|adjacent_part_numbers| adjacent_part_numbers.len() == 2);

        let gear_ratios = gear_part_numbers.map(|part_numbers| {
            part_numbers
                .iter()
                .map(|part_number| part_number.number)
                .product::<u32>()
        });

        println!("Sum of gear ratios: {}", gear_ratios.sum::<u32>());
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("03-1:");
    first()?;
    println!("03-2:");
    second()
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use crate::task_03::{neighbours, parse_schematic, Coordinate, PartNumber, Schematic, Symbol};

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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
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
                    ]
                    .to_vec(),
                },
            ]
            .to_vec(),
            symbols: [
                Symbol {
                    label: "*".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 1,
                        char_index: 3,
                    }]
                    .to_vec(),
                },
                Symbol {
                    label: "#".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 3,
                        char_index: 6,
                    }]
                    .to_vec(),
                },
                Symbol {
                    label: "*".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 4,
                        char_index: 3,
                    }]
                    .to_vec(),
                },
                Symbol {
                    label: "+".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 5,
                        char_index: 5,
                    }]
                    .to_vec(),
                },
                Symbol {
                    label: "$".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 8,
                        char_index: 3,
                    }]
                    .to_vec(),
                },
                Symbol {
                    label: "*".to_owned(),
                    coordinates: [Coordinate {
                        line_index: 8,
                        char_index: 5,
                    }]
                    .to_vec(),
                },
            ]
            .to_vec(),
        };

        let actual = parse_schematic(example_data);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn neighbours_should_be_empty_for_an_empty_list() {
        assert_eq!(neighbours(&Vec::new()), Vec::new());
    }

    #[test]
    fn neighbours_should_be_as_expected() {
        let expected = [
            Coordinate {
                line_index: -1,
                char_index: 0,
            },
            Coordinate {
                line_index: -1,
                char_index: 1,
            },
            Coordinate {
                line_index: -1,
                char_index: 2,
            },
            Coordinate {
                line_index: 0,
                char_index: 0,
            },
            Coordinate {
                line_index: 1,
                char_index: 0,
            },
            Coordinate {
                line_index: 1,
                char_index: 1,
            },
            Coordinate {
                line_index: 1,
                char_index: 2,
            },
            Coordinate {
                line_index: -1,
                char_index: 1,
            },
            Coordinate {
                line_index: -1,
                char_index: 2,
            },
            Coordinate {
                line_index: -1,
                char_index: 3,
            },
            Coordinate {
                line_index: 0,
                char_index: 3,
            },
            Coordinate {
                line_index: 1,
                char_index: 1,
            },
            Coordinate {
                line_index: 1,
                char_index: 2,
            },
            Coordinate {
                line_index: 1,
                char_index: 3,
            },
        ]
        .to_vec();

        let actual = neighbours(
            &[
                Coordinate {
                    line_index: 0,
                    char_index: 1,
                },
                Coordinate {
                    line_index: 0,
                    char_index: 2,
                },
            ]
            .to_vec(),
        );

        assert_eq!(actual, expected);
    }
}
