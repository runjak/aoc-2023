use regex::Regex;
use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
struct CategoryMap {
    from: String,
    to: String,
    mappings: Vec<(i64, i64, i64)>,
}

#[derive(Debug, PartialEq)]
struct TaskInput {
    seeds: Vec<i64>,
    category_maps: Vec<CategoryMap>,
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let prefix = "seeds: ";

    if !line.starts_with(prefix) {
        return Vec::new();
    }

    line[prefix.len()..]
        .split(" ")
        .filter_map(|digits| digits.parse::<i64>().ok())
        .collect()
}

fn parse_category_map(lines: &str) -> Option<CategoryMap> {
    let lines = &mut lines.lines();

    let first_line = lines.next()?;
    let first_line_regex = Regex::new(r"(?<from>[a-z]+)-to-(?<to>[a-z]+)\smap:").ok()?;
    let captures = first_line_regex.captures(first_line)?;

    let from = captures.name("from")?.as_str().to_string();
    let to = captures.name("to")?.as_str().to_string();

    let mappings = lines
        .filter_map(|line| -> Option<(i64, i64, i64)> {
            let numbers = line
                .split(" ")
                .filter_map(|digits| digits.parse::<i64>().ok())
                .collect::<Vec<_>>();

            let [x, y, z] = numbers.as_slice() else {
                return None;
            };

            Some((*x, *y, *z))
        })
        .collect();

    Some(CategoryMap { from, to, mappings })
}

fn parse_task_input(input: String) -> Option<TaskInput> {
    let items = &mut input.split("\n\n");

    let seeds = parse_seeds(items.next()?);
    let category_maps = items.filter_map(parse_category_map).collect::<Vec<_>>();

    Some(TaskInput {
        seeds,
        category_maps,
    })
}

fn map_seed(seed: i64, category_map: &CategoryMap) -> i64 {
    let mapping = category_map
        .mappings
        .iter()
        .filter(|(_destination, source, length)| seed >= *source && seed < *source + *length)
        .next();

    match mapping {
        None => seed,
        Some((destination, source, _length)) => destination + (seed - source),
    }
}

fn map_seeds(seeds: Vec<i64>, category_map: &CategoryMap) -> Vec<i64> {
    seeds
        .iter()
        .map(|seed| map_seed(*seed, category_map))
        .collect()
}

pub fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/05/example-1.txt", "./inputs/05/input.txt"];

    for path in paths {
        println!("Reading file {}", path);

        let contents = fs::read_to_string(path)?;
        let task_input = parse_task_input(contents).unwrap();

        let locations = task_input
            .category_maps
            .iter()
            .fold(task_input.seeds, |seeds, category_map| {
                map_seeds(seeds, category_map)
            });

        let lowest_location = locations.iter().min().unwrap_or(&0);

        println!("Lowest location: {}", lowest_location);
    }

    Ok(())
}

fn map_seed_range(seed_range: (i64, i64), category_map: &CategoryMap) -> Vec<(i64, i64)> {
    let mappings = &mut category_map.mappings.clone();
    mappings.sort_by(|(_, source_a, _), (_, source_b, _)| source_a.cmp(source_b));

    let (mut seed_start, mut seed_length) = seed_range;

    let mut mapped_ranges: Vec<(i64, i64)> = mappings
        .iter()
        .flat_map(
            |(destination, source_start, mapping_length)| -> Vec<(i64, i64)> {
                let seed_end = seed_start + seed_length - 1;
                let source_end = source_start + mapping_length - 1;

                // No seed range present to map
                if seed_length <= 0 {
                    return Vec::new();
                }

                // Seed range entirely later than current mapping
                if seed_start > source_end {
                    return Vec::new();
                }

                // Seed range entirely before source
                if seed_end < *source_start {
                    let result = [(seed_start, seed_length)].to_vec();
                    seed_length = 0;
                    return result;
                }

                let seed_starts_before_mapping = seed_start < *source_start;
                let seed_ends_after_mapping = seed_end > source_end;

                match (seed_starts_before_mapping, seed_ends_after_mapping) {
                    (true, true) => {
                        // seed range overlapping mapping entirely
                        let before_length = source_start - seed_start;
                        let before_mapping = (seed_start, before_length);
                        seed_length -= before_length;

                        let inner_mapping = (*destination, *mapping_length);
                        seed_start = source_start + mapping_length;
                        seed_length -= mapping_length;

                        return [before_mapping, inner_mapping].to_vec();
                    }
                    (true, false) => {
                        // seed range precedes mapping range, but doesn't continue after
                        let before_length = source_start - seed_start;
                        let before_mapping = (seed_start, before_length);
                        seed_length -= before_length;

                        let inner_mapping = (*destination, seed_length);
                        seed_length = 0;

                        return [before_mapping, inner_mapping].to_vec();
                    }
                    (false, true) => {
                        // seed range starts inside mapping range and continues after

                        let inner_length = mapping_length - (seed_start - *source_start);
                        let inner_mapping = (*destination, inner_length);
                        seed_length -= inner_length;

                        return [inner_mapping].to_vec();
                    }
                    (false, false) => {
                        // seed range entirely contained
                        let result = [(seed_start, seed_length)].to_vec();
                        seed_length = 0;

                        return result;
                    }
                }
            },
        )
        .collect();

    if seed_length > 0 {
        mapped_ranges.push((seed_start, seed_length))
    }

    return mapped_ranges;
}

pub fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/05/example-1.txt", "./inputs/05/input.txt"];

    for path in paths {
        println!("Reading file {}", path);

        let contents = fs::read_to_string(path)?;
        let task_input = parse_task_input(contents).unwrap();

        let seed_chunks = task_input
            .seeds
            .chunks(2)
            .filter_map(|chunk| -> Option<(i64, i64)> {
                let [a, b] = chunk else {
                    return None;
                };

                Some((*a, *b))
            });

        let seeds = seed_chunks
            .flat_map(|(from, length)| -> Vec<i64> { (from..from + length).collect() })
            .collect::<Vec<_>>();

        let locations = task_input
            .category_maps
            .iter()
            .fold(seeds, |seeds, category_map| map_seeds(seeds, category_map));

        let lowest_location = locations.iter().min().unwrap_or(&0);

        println!("lowest valid location: {}", lowest_location);

        break;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("05-1:");
    first()?;
    println!("05-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::task_05::{map_seed, map_seed_range};

    use super::CategoryMap;

    #[test]
    fn map_seed_should_behave() {
        let category_map = &CategoryMap {
            from: "seed".to_string(),
            to: "soil".to_string(),
            mappings: [(50, 98, 2), (52, 50, 48)].to_vec(),
        };

        assert_eq!(map_seed(79, category_map), 81);
        assert_eq!(map_seed(14, category_map), 14);
        assert_eq!(map_seed(55, category_map), 57);
        assert_eq!(map_seed(13, category_map), 13);
    }

    #[test]
    fn map_seed_range_should_behave() {
        let category_map = &CategoryMap {
            from: "seed".to_string(),
            to: "soil".to_string(),
            mappings: [(10, 3, 2), (10, 7, 2)].to_vec(),
        };

        // seed before both mappings
        assert_eq!(map_seed_range((0, 2), category_map), [(0, 2)].to_vec());
        // seed between both mappings
        assert_eq!(map_seed_range((5, 2), category_map), [(5, 2)].to_vec());
        // seed after both mappings
        assert_eq!(map_seed_range((9, 2), category_map), [(9, 2)].to_vec());

        // seed before and in first mapping
        assert_eq!(
            map_seed_range((0, 3), category_map),
            [(0, 2), (3, 1)].to_vec()
        );
        // // seed entirely in first mapping
        // assert_eq!(map_seed_range((), category_map), [()].to_vec());
        // // seed in and after first mapping
        // assert_eq!(map_seed_range((), category_map), [()].to_vec());
        // // seed spanning all mappings
        // assert_eq!(map_seed_range((), category_map), [()].to_vec());
    }

    // #[test]
    // fn map_seed_range_should_work_like_example() {
    //     let category_map = &CategoryMap {
    //         from: "seed".to_string(),
    //         to: "soil".to_string(),
    //         mappings: [(50, 98, 2), (52, 50, 48)].to_vec(),
    //     };

    //     assert_eq!(map_seed_range((79, 14), category_map), [(79, 14)].to_vec());
    //     assert_eq!(map_seed_range((55, 13), category_map), [(57, 13)].to_vec());
    // }
}
