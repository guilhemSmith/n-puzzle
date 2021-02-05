mod algorithm;
mod arguments;
mod generation;
mod heuristic;
mod puzzle;

use colored::*;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = arguments::get();
    let heuristic_method =
        heuristic::arg_to_method(arguments.value_of("heuristic").unwrap()).unwrap();
    let (size, start) = if let Some(filename) = arguments.value_of("file") {
        let (size, start) = generation::from_file(filename)?;
        println!("puzzle parsed:\n{}", start);
        (size, start)
    } else {
        let size = arguments.value_of("size").unwrap_or("3").parse()?;
        let without_solution = arguments.is_present("without_solution");
        let iterations = arguments.value_of("iterations").unwrap_or("1000").parse()?;
        let start = generation::random(size, !without_solution, iterations);
        println!("puzzle generated:\n{}", start);
        (size, start)
    };
    let goal = puzzle::StateUnknown::goal(size);
    let split_line = format!(
        " {:-^size$} ",
        "",
        size = size * if size > 3 { 5 } else { 3 }
    )
    .dimmed();
    if !algorithm::has_solution(&start, &goal) {
        println!("\n{}\n\npuzzle unsolvable.", split_line);
        return Ok(());
    }
    let solution = algorithm::a_star(start, goal, heuristic_method);
    if let Some(moves) = solution.moves() {
        println!("\n{}\n\nsolution moves:", split_line);
        for step in moves.iter().rev() {
            println!("{}\n\n{}", step, split_line);
        }
        println!("\npuzzle solved in {} moves.", moves.len() - 1);
    } else {
        println!("\n{}\n\npuzzle unsolvable.", split_line);
    }
    println!(
        "\n{}\n\ntime complexity: {}\nsize complexity: {}",
        split_line,
        solution.time_complexity(),
        solution.size_complexity()
    );
    Ok(())
}
