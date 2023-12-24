use std::{error::Error, fs};

// A pattern is made up of several lines of strings.
type Pattern = Vec<String>;

fn parse_input(contents: String) -> Vec<Pattern> {
    contents
        .split("\n\n")
        .map(|lines| -> Pattern { lines.split("\n").map(|s| s.to_string()).collect::<Vec<_>>() })
        .collect()
}

fn transpose_pattern(pattern: &Pattern) -> Pattern {
    // Inspired by https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust

    if pattern.is_empty() {
        return Vec::new();
    }

    let width = pattern[0].len();
    let mut iters = pattern
        .into_iter()
        .map(|line| line.chars())
        .collect::<Vec<_>>();

    (0..width)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|i| i.next())
                .collect::<String>()
        })
        .collect()
}

type N = u32;

fn filter_symmetries(line: &String, candidates: Vec<usize>) -> Vec<usize> {
    let mut filtered_candidates: Vec<usize> = Vec::new();

    for candidate in candidates {
        let (prefix, suffix) = line.split_at(candidate + 1);
        let prefix = prefix.chars().rev().collect::<String>();

        let is_symmetric = if prefix.len() > suffix.len() {
            prefix.starts_with(suffix)
        } else {
            suffix.starts_with(&prefix)
        };

        if is_symmetric {
            filtered_candidates.push(candidate);
        }
    }

    filtered_candidates
}

fn find_horizontal_symmetry(pattern: &Pattern) -> Option<N> {
    if pattern.is_empty() {
        return None;
    }

    let width = pattern[0].len();
    let mut column_candidates = (0..width - 1).collect::<Vec<_>>();

    // Filter column_candidates against all rows of a pattern
    for line in pattern {
        // Early exit where possible
        if column_candidates.is_empty() {
            return None;
        }

        column_candidates = filter_symmetries(line, column_candidates);
    }

    // Extract found symmetry
    let symmetry = column_candidates.first()? + 1;
    N::try_from(symmetry).ok()
}

fn find_vertical_symmetry(pattern: &Pattern) -> Option<N> {
    find_horizontal_symmetry(&transpose_pattern(pattern))
}

fn score_pattern(pattern: &Pattern) -> N {
    find_horizontal_symmetry(pattern).unwrap_or_else(|| {
        find_vertical_symmetry(pattern)
            .map(|n| 100 * n)
            .unwrap_or(0)
    })
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/13/example-1.txt", "./inputs/13/input.txt"];

    for path in paths {
        println!("File {}", path);

        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        let sum = input
            .iter()
            .map(|pattern| score_pattern(pattern))
            .sum::<N>();
        println!("Sum: {}", sum);
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("13-1:");
    first()?;
    println!("13-2:");
    second()?;

    Ok(())
}
