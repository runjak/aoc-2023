use std::{env, error::Error};
mod task_01;
mod task_02;
mod task_03;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args()
        .filter_map(|arg| arg.parse::<usize>().ok())
        .collect();

    let tasks = Vec::from([task_01::main, task_02::main, task_03::main]);

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
