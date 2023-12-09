use std::{error::Error, fs};

type Z = i32;

fn parse_input(contents: String) -> Vec<Vec<Z>> {
    contents
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|digits| digits.parse().ok())
                .collect()
        })
        .collect()
}

fn derive(values: &Vec<Z>) -> Vec<Z> {
    let mut tail = values.iter();
    tail.next();

    values.iter().zip(tail).map(|(a, b)| b - a).collect()
}

fn all_zero(values: &Vec<Z>) -> bool {
    values.iter().all(|v| v == &0)
}

fn derives(values: &Vec<Z>) -> Vec<Vec<Z>> {
    let mut ret = Vec::new();

    let mut current_values = values.clone();
    while !all_zero(&current_values) {
        let next_values = derive(&current_values);
        ret.push(current_values);
        current_values = next_values;
    }
    ret.push(current_values);

    return ret;
}

fn extrapolate_last(derives: &Vec<Vec<Z>>) -> Z {
    derives
        .iter()
        .map(|values| values.last().unwrap_or(&0))
        .sum()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/09/example-1.txt", "./inputs/09/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        let sum = input
            .iter()
            .map(|values| extrapolate_last(&derives(values)))
            .sum::<Z>();

        println!("Sum: {}", sum);
    }

    Ok(())
}

fn extrapolate_first(derives: &Vec<Vec<Z>>) -> Z {
    let mut firsts = derives
        .iter()
        .map(|values| values.first().unwrap_or(&0))
        .collect::<Vec<_>>();
    firsts.reverse();

    firsts.iter().fold(0, |acc, value| -> Z {
        acc - **value
    })
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/09/example-1.txt", "./inputs/09/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        let sum = input
            .iter()
            .map(|values| extrapolate_first(&derives(values)))
            .sum::<Z>();

        println!("Sum: {}", sum);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("09-1:");
    first()?;
    println!("09-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::derives;

    #[test]
    fn derives_should_match_examples() -> Result<(), Box<dyn Error>> {
        let examples = [
            [0, 3, 6, 9, 12, 15].to_vec(),
            [1, 3, 6, 10, 15, 21].to_vec(),
            [10, 13, 16, 21, 30, 45].to_vec(),
        ]
        .to_vec();

        let [expected_1, expected_2, expected_3] = [
            [
                [0, 3, 6, 9, 12, 15].to_vec(),
                [3, 3, 3, 3, 3].to_vec(),
                [0, 0, 0, 0].to_vec(),
            ]
            .to_vec(),
            [
                [1, 3, 6, 10, 15, 21].to_vec(),
                [2, 3, 4, 5, 6].to_vec(),
                [1, 1, 1, 1].to_vec(),
                [0, 0, 0].to_vec(),
            ]
            .to_vec(),
            [
                [10, 13, 16, 21, 30, 45].to_vec(),
                [3, 3, 5, 9, 15].to_vec(),
                [0, 2, 4, 6].to_vec(),
                [2, 2, 2].to_vec(),
                [0, 0].to_vec(),
            ]
            .to_vec(),
        ];

        let actual_derives = examples.iter().map(derives).collect::<Vec<_>>();
        let [actual_1, actual_2, actual_3] = actual_derives.as_slice() else {
            todo!("test could not match actual outputs as expected")
        };

        assert_eq!(actual_1, &expected_1);
        assert_eq!(actual_2, &expected_2);
        assert_eq!(actual_3, &expected_3);

        Ok(())
    }
}
