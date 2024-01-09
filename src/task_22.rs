use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

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

fn max_z(brick: &BrickOfCubes) -> N {
    brick.iter().map(|(_, _, z)| *z).max().unwrap_or(0)
}

fn min_z(brick: &BrickOfCubes) -> N {
    brick.iter().map(|(_, _, z)| *z).min().unwrap_or(0)
}

type XY = (N, N);

fn project_xy(brick: &BrickOfCubes) -> HashSet<XY> {
    brick.iter().map(|(x, y, _)| (*x, *y)).collect()
}

fn count_safe_to_disintegrate(bricks: &Vec<InputBrick>) -> N {
    let bricks = bricks.iter().map(into_cubes).collect::<Vec<_>>();

    let mut xy_to_brick_index: HashMap<XY, Vec<usize>> = HashMap::new();
    for (brick_index, brick) in bricks.iter().enumerate() {
        for xy in project_xy(&brick) {
            match xy_to_brick_index.get_mut(&xy) {
                Some(brick_indices) => {
                    brick_indices.push(brick_index);
                }
                None => {
                    xy_to_brick_index.insert(xy, Vec::from([brick_index]));
                }
            }
        }
    }

    for (brick_index, brick) in bricks.iter().enumerate() {
        let related_bricks = project_xy(&brick)
            .iter()
            .flat_map(|xy| -> Vec<&BrickOfCubes> {
                let Some(related_indices) = xy_to_brick_index.get(xy) else {
                    return Vec::new();
                };

                related_indices
                    .iter()
                    .filter_map(|index| bricks.get(*index))
                    .collect()
            })
            .collect::<HashSet<_>>();
    }

    todo!("Rest of the owl")
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
