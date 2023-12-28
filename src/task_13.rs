use std::{collections::HashMap, error::Error, fs};

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

fn filter_symmetries(line: &String, candidates: &Vec<usize>) -> Vec<usize> {
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
            filtered_candidates.push(*candidate);
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

        column_candidates = filter_symmetries(line, &column_candidates);
    }

    // Extract found symmetry
    let symmetry = column_candidates.first()?;
    N::try_from(*symmetry).ok()
}

fn find_vertical_symmetry(pattern: &Pattern) -> Option<N> {
    find_horizontal_symmetry(&transpose_pattern(pattern))
}

fn score_pattern(pattern: &Pattern) -> N {
    find_horizontal_symmetry(pattern)
        .map(|n| n + 1)
        .unwrap_or_else(|| {
            find_vertical_symmetry(pattern)
                .map(|n| 100 * (n + 1))
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

fn symmetry_violations(line: &String, candidates: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut violations: HashMap<usize, usize> = HashMap::new();

    for candidate in candidates.keys() {
        let (prefix, suffix) = line.split_at(candidate + 1);

        let difference_count = prefix
            .chars()
            .rev()
            .zip(suffix.chars())
            .filter(|(a, b)| a != b)
            .count();

        violations.insert(*candidate, difference_count);
    }

    violations
}

fn find_smudge_horizontal_symmetry(pattern: &Pattern) -> Option<N> {
    if pattern.is_empty() {
        return None;
    }

    let width = pattern[0].len();
    let mut candidates: HashMap<usize, usize> = (0..width - 1).map(|k| (k, 0)).collect();

    for line in pattern {
        let violations = symmetry_violations(line, &candidates);

        for (k, v) in violations {
            let v = v + candidates.get(&k).unwrap_or(&0);

            if v >= 2 {
                candidates.remove(&k);
            } else {
                candidates.insert(k, v);
            }
        }
    }

    candidates
        .iter()
        .filter(|(_, v)| **v == 1)
        .filter_map(|(k, _)| N::try_from(*k).ok())
        .next()
}

fn find_smudge_vertical_symmetry(pattern: &Pattern) -> Option<N> {
    find_smudge_horizontal_symmetry(&transpose_pattern(pattern))
}

fn score_smudge_pattern(pattern: &Pattern) -> N {
    find_smudge_horizontal_symmetry(pattern)
        .map(|n| n + 1)
        .unwrap_or_else(|| {
            find_smudge_vertical_symmetry(pattern)
                .map(|n| 100 * (n + 1))
                .unwrap_or(0)
        })
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/13/example-1.txt", "./inputs/13/input.txt"];

    for path in paths {
        println!("File {}", path);

        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        let sum = input
            .iter()
            .map(|pattern| score_smudge_pattern(pattern))
            .sum::<N>();
        println!("Sum: {}", sum);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("13-1:");
    first()?;
    println!("13-2:");
    second()?;

    Ok(())
}
