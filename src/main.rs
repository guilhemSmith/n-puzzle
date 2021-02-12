mod algorithm;
mod arguments;
mod generation;
mod puzzle;

use algorithm::Tool;
use colored::*;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
	let args = arguments::get();

	let (size, start, heuristic, search_type, weight) = setup(&args)?;
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
	let solution = algorithm::w_a_star(start, goal, heuristic, search_type, weight);
	if let Some(moves) = solution.moves() {
		println!("\n{}\n\nsolution moves:", split_line);
		for step in moves.iter().rev() {
			println!("{}\n\n{}", step, split_line);
		}
		if let Some(search_type_arg) = args.value_of("search_type") {
			if search_type_arg == algorithm::SearchType::DEFAULT {
				print!(
					"\nweight used:\t\t{}",
					match weight {
						w if w < 2.0 => weight.to_string().blue(),
						w if w < 3.0 => weight.to_string().purple(),
						_ => weight.to_string().red(),
					}
				);
			}
		}
		println!(
			"\nheuristic used:\t\t{}\nsearch type used:\t{}\n\n{}\n\npuzzle solved in {} moves.",
			algorithm::Heuristic::pretty_name(args.value_of("heuristic").unwrap()).unwrap(),
			algorithm::SearchType::pretty_name(args.value_of("search_type").unwrap()).unwrap(),
			split_line,
			moves.len() - 1
		);
	} else {
		println!("\n{}\n\npuzzle unsolvable.", split_line);
	}
	println!(
		"\ntime complexity:\t{}\nsize complexity:\t{}",
		solution.time_complexity(),
		solution.size_complexity()
	);
	Ok(())
}
fn setup(
	args: &clap::ArgMatches,
) -> Result<
	(
		usize,
		puzzle::StateUnknown,
		algorithm::Heuristic,
		algorithm::SearchType,
		f32,
	),
	Box<dyn error::Error>,
> {
	let heuristic = algorithm::Heuristic::get(args.value_of("heuristic").unwrap()).unwrap();
	let search_type = algorithm::SearchType::get(args.value_of("search_type").unwrap()).unwrap();
	let (size, start) = if let Some(filename) = args.value_of("file") {
		let (size, start) = generation::from_file(filename)?;
		println!("puzzle parsed:\n{}", start);
		(size, start)
	} else {
		let size = args.value_of("size").unwrap_or("3").parse()?;
		let without_solution = args.is_present("without_solution");
		let iterations = args.value_of("iterations").unwrap_or("1000").parse()?;
		let start = generation::random(size, !without_solution, iterations);
		println!("puzzle generated:\n{}", start);
		(size, start)
	};
	let weight = args
		.value_of("weight")
		.map(|s| s.parse().unwrap())
		.unwrap_or(crate::algorithm::dynamic_weight(size));
	return Ok((size, start, heuristic, search_type, weight));
}
