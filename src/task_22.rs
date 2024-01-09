use std::{error::Error, fs};

type N = i32;
type XYZ = (N, N, N);
type InputBrick = (XYZ, XYZ);

fn parse_position(position: &str) -> Option<XYZ> {
    let parts = position.split(",").collect::<Vec<_>>();

    match parts.as_slice() {
        [x, y, z] => Some((
            x.parse::<N>().ok()?,
            y.parse::<N>().ok()?,
            z.parse::<N>().ok()?,
        )),
        _ => None,
    }
}

fn parse_bricks(input: String) -> Vec<InputBrick> {
    input
        .lines()
        .filter_map(|line| -> Option<InputBrick> {
            let (from, to) = line.split_once("~")?;

            Some((parse_position(from)?, parse_position(to)?))
        })
        .collect()
}

type BrickOfCubes = Vec<XYZ>;

fn into_cubes(brick: &InputBrick) -> BrickOfCubes {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = *brick;

    let mut cubes: Vec<XYZ> = Vec::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                cubes.push((x, y, z));
            }
        }
    }

    cubes
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/22/example-1.txt", "./inputs/22/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_bricks(input);

        println!("Got input:\n{:?}", input);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("22-1:");
    first()?;
    println!("22-2:");
    second()?;

    Ok(())
}
