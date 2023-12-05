use regex::Regex;
use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
struct CategoryMap {
    from: String,
    to: String,
    mappings: Vec<(u32, u32, u32)>,
}

#[derive(Debug, PartialEq)]
struct TaskInput {
    seeds: Vec<u32>,
    category_maps: Vec<CategoryMap>,
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let prefix = "seeds: ";

    if !line.starts_with(prefix) {
        return Vec::new();
    }

    line[prefix.len()..]
        .split(" ")
        .filter_map(|digits| digits.parse::<u32>().ok())
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
        .filter_map(|line| -> Option<(u32, u32, u32)> {
            let numbers = line
                .split(" ")
                .filter_map(|digits| digits.parse::<u32>().ok())
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

fn map_seed(seed: u32, category_map: &CategoryMap) -> u32 {
    let mapping = category_map
        .mappings
        .iter()
        .filter(|(source, _destination, length)| *source <= seed && seed <= *source + *length)
        .next();

    match mapping {
        None => seed,
        Some((source, destination, _length)) => destination + (seed - source),
    }
}

fn map_seeds(seeds: Vec<u32>, category_map: &CategoryMap) -> Vec<u32> {
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

        println!("Locations: {:?}", locations);

        let lowest_location = locations.iter().min().unwrap_or(&0);

        println!("Lowest location: {}", lowest_location);

        break;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("05-1:");
    first()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::task_05::map_seed;

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
        assert_eq!(map_seed(55, category_map), 47);
        assert_eq!(map_seed(13, category_map), 13);
    }
}
