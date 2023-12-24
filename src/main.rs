use std::{env, error::Error};
mod task_01;
mod task_02;
mod task_03;
mod task_04;
mod task_05;
mod task_06;
mod task_07;
mod task_08;
mod task_09;
mod task_10;
mod task_11;
mod task_12;
mod task_13;
mod task_14;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args()
        .filter_map(|arg| arg.parse::<usize>().ok())
        .collect();

    let tasks = Vec::from([
        task_01::main,
        task_02::main,
        task_03::main,
        task_04::main,
        task_05::main,
        task_06::main,
        task_07::main,
        task_08::main,
        task_09::main,
        task_10::main,
        task_11::main,
        task_12::main,
        task_13::main,
        task_14::main,
    ]);

    match args.first() {
        None => {
            println!("No task given.");
            return Ok(());
        }
        Some(task_index) => match tasks.get(*task_index - 1) {
            None => {
                println!("No task for task_index {}.", *task_index);
                return Ok(());
            }
            Some(task) => task(),
        },
    }
}
