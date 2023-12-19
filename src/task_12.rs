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

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/12/example-1.txt", "./inputs/12/input.txt"];

    for path in paths {
        println!("File {}", path);
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        println!("parsed input:\n{:?}", input);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

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

    use super::SpringData;

    #[test]
    fn generate_candidates_should_produce_example_candidates() {
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
}