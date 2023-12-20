use std::{error::Error, fs};

type N = u32;
type Groups = Vec<N>;
type SpringData = (String, Groups);
type Input = Vec<SpringData>;

fn parse_input(input: String) -> Input {
    input
        .lines()
        .filter_map(|line| -> Option<SpringData> {
            let parts = line.split(" ").collect::<Vec<_>>();

            let [springs, groups] = parts.as_slice() else {
                return None;
            };

            let groups = groups
                .split(",")
                .filter_map(|group| -> Option<N> { group.parse::<N>().ok() })
                .collect::<Vec<_>>();

            Some((springs.to_string(), groups))
        })
        .collect()
}

fn count_missing_broken(spring_data: &SpringData) -> N {
    let total_broken = spring_data.1.iter().sum::<N>();
    let known_broken =
        N::try_from(spring_data.0.chars().filter(|c| *c == '#').count()).unwrap_or(0);

    total_broken - known_broken
}

fn count_unknowns(spring_data: &SpringData) -> N {
    N::try_from(spring_data.0.chars().filter(|c| *c == '?').count()).unwrap_or(0)
}

fn do_generate_candidates(missing_broken: N, missing_unbroken: N, springs: &String) -> Vec<String> {
    if missing_broken <= 0 {
        return Vec::from([springs.replace("?", ".")]);
    }

    (0..=missing_unbroken)
        .flat_map(|unbroken_at_front| {
            let remaining_missing_unbroken = missing_unbroken - unbroken_at_front;
            let candidate = springs
                .replacen("?", ".", usize::try_from(unbroken_at_front).unwrap())
                .replacen("?", "#", 1);

            return do_generate_candidates(
                missing_broken - 1,
                remaining_missing_unbroken,
                &candidate,
            );
        })
        .collect()
}

fn generate_candidates(spring_data: &SpringData) -> Vec<String> {
    let unknowns = count_unknowns(spring_data);
    let missing_broken = count_missing_broken(spring_data);
    let missing_unbroken = unknowns - missing_broken;

    do_generate_candidates(missing_broken, missing_unbroken, &spring_data.0)
}

fn is_valid_arrangement(candidate: String, groups: &Groups) -> bool {
    let mut candidate = candidate.as_str();

    for group in groups {
        // We need group as a usize here
        let Some(group) = usize::try_from(*group).ok() else {
            return false;
        };

        // Remove possible non-group prefix
        let Some(group_start) = candidate.find("#") else {
            return false;
        };
        candidate = &candidate[group_start..];

        // Check for continuous group
        let group_is_consistent = candidate.chars().take(group).all(|c| c == '#');
        if !group_is_consistent {
            return false;
        }

        // Discard current group as we just checked it
        candidate = &candidate[group..];
        // First element cannot also be broken now.
        match candidate.chars().next() {
            Some('#') => return false,
            _ => (),
        }
    }

    // No broken springs may remain after the groups:
    !candidate.contains("#")
}

fn generate_arrangements(spring_data: &SpringData) -> Vec<String> {
    let candidates = generate_candidates(spring_data);
    candidates
        .iter()
        .filter(|candidate| is_valid_arrangement((*candidate).to_string(), &spring_data.1))
        .map(|s| s.to_string())
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/12/example-1.txt", "./inputs/12/input.txt"];

    for path in paths {
        println!("File {}", path);
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let sum_of_arrangements = input
            .iter()
            .map(|spring_data| N::try_from(generate_arrangements(spring_data).len()).unwrap())
            .sum::<N>();

        println!("Sum of arrangements: {}", sum_of_arrangements);
    }

    Ok(())
}

fn unfold_spring_data(spring_data: &SpringData) -> SpringData {
    let (springs, groups) = spring_data;

    let springs = [
        springs.to_string(),
        springs.to_string(),
        springs.to_string(),
        springs.to_string(),
        springs.to_string(),
    ]
    .to_vec()
    .join("?");

    let groups = [
        groups.clone(),
        groups.clone(),
        groups.clone(),
        groups.clone(),
        groups.clone(),
    ]
    .to_vec()
    .concat();

    (springs, groups)
}

// FIXME better naming?
fn breed(spring_data: &SpringData) -> Vec<SpringData> {
    let (springs, groups) = spring_data;

    let broken: SpringData = (springs.replacen("?", "#", 1), groups.clone());
    let working: SpringData = (springs.replacen("?", ".", 1), groups.clone());

    Vec::from([broken, working])
}

fn is_plausible(spring_data: &SpringData) -> bool {
    let (springs, groups) = spring_data;

    let expected_broken = groups.iter().sum::<N>();
    let Some(current_broken) = N::try_from(springs.chars().filter(|c| *c == '#').count()).ok()
    else {
        return false;
    };

    if current_broken > expected_broken {
        return false;
    }

    // Similar code to is_valid_arrangement
    let mut candidate = spring_data.0.as_str();

    for group in groups {
        let Some(group) = usize::try_from(*group).ok() else {
            return false;
        };

        // Remove possible non-group prefix of '.'

        // Check for continues group

        // Discard current group

        // First element must now be other than '#'.
    }

    todo!("Draw the rest of the owl.")
}

// FIXME better naming?
fn ultra_step(spring_data: &SpringData) -> (Vec<String>, Vec<SpringData>) {
    let mut arrangements: Vec<String> = Vec::new();
    let mut next_sources: Vec<SpringData> = Vec::new();

    let next_candidates = breed(&spring_data);

    for candidate in next_candidates {
        if !is_plausible(&candidate) {
            continue;
        }

        let is_unfinished = candidate.0.contains("?");

        if is_unfinished {
            next_sources.push(candidate);
        } else {
            arrangements.push(candidate.0);
        }
    }

    (arrangements, next_sources)
}

// FIXME type signature and magic and such
// Behavior should match generate_arrangements and so we can test it with that.
fn ultra_function(spring_data: &SpringData) -> Vec<String> {
    let mut arrangements: Vec<String> = Vec::new();
    let mut candidate_sources: Vec<SpringData> = Vec::from([spring_data.clone()]);

    while !candidate_sources.is_empty() {
        let spring_data = candidate_sources.pop().unwrap();

        let (mut new_arrangements, mut new_candidate_sources) = ultra_step(&spring_data);

        arrangements.append(&mut new_arrangements);
        candidate_sources.append(&mut new_candidate_sources);
    }

    return arrangements;
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/12/example-1.txt", "./inputs/12/input.txt"];

    for path in paths {
        println!("File {}", path);
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let sum_of_arrangements = input
            .iter()
            .map(|spring_data| {
                N::try_from(generate_arrangements(&unfold_spring_data(spring_data)).len()).unwrap()
            })
            .sum::<N>();

        println!("Sum of arrangements: {}", sum_of_arrangements);

        break;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("12-1:");
    first()?;
    println!("12-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::task_12::generate_candidates;

    use super::{generate_arrangements, unfold_spring_data, SpringData};

    #[test]
    fn generate_candidates_should_produce_expected_candidates() {
        let example: SpringData = ("???.###".to_string(), [1, 1, 3].to_vec());

        let expected = [
            "##..###".to_string(),
            "#.#.###".to_string(),
            ".##.###".to_string(),
        ]
        .to_vec();

        let actual = generate_candidates(&example);

        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_arrangements_should_reproduce_example() {
        let example: SpringData = ("?###????????".to_string(), [3, 2, 1].to_vec());

        let expected = [
            ".###.##.#...".to_string(),
            ".###.##..#..".to_string(),
            ".###.##...#.".to_string(),
            ".###.##....#".to_string(),
            ".###..##.#..".to_string(),
            ".###..##..#.".to_string(),
            ".###..##...#".to_string(),
            ".###...##.#.".to_string(),
            ".###...##..#".to_string(),
            ".###....##.#".to_string(),
        ]
        .to_vec();

        let actual = generate_arrangements(&example);

        assert_eq!(actual, expected);
    }

    #[test]
    fn unfold_spring_data_should_behave_as_provided_example() {
        let example: SpringData = ("???.###".to_string(), [1, 1, 3].to_vec());
        let expected: SpringData = (
            "???.###????.###????.###????.###????.###".to_string(),
            [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3].to_vec(),
        );

        let actual = unfold_spring_data(&example);

        assert_eq!(actual, expected);
    }
}
