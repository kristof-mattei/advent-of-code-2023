use shared::Day;

mod day_01;
mod shared;
mod utils;

fn print_answer(day: usize, part: u32, result: &str) {
    println!("Answer to Day {}, part {} is ... {}", day, part, result);
}

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let solutions: Vec<Box<dyn Day>> = vec![Box::new(day_01::Solution {})];

    for (i, solution) in solutions.iter().enumerate() {
        print_answer(i + 1, 1, &solution.part_1().to_string());
        print_answer(i + 1, 2, &solution.part_2().to_string());
    }

    Ok(())
}
