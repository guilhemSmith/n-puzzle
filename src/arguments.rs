use crate::algorithm::Tool;

pub fn get<'a>() -> clap::ArgMatches<'a> {
	clap::App::new("n-puzzle")
		.version("0.1.0")
		.author("Guilhem SMITH <gsmith@student.42.fr>")
		.about("n-puzzle solver using A* algorithm.")
		.arg(
			clap::Arg::with_name("file")
				.short("f")
				.long("file")
				.value_name("FILENAME")
				.help("The file to read the puzzle from")
				.number_of_values(1)
				.multiple(false),
		)
		.arg(
			clap::Arg::with_name("heuristic")
				.short("h")
				.long("heuristic")
				.value_name("HEURISTIC")
				.possible_values(&crate::algorithm::Heuristic::STR_LIST)
				.default_value(crate::algorithm::Heuristic::DEFAULT)
				.number_of_values(1)
				.multiple(false)
				.help("The heuristic used to estimate the distance of each state to the goal"),
		)
		.arg(
			clap::Arg::with_name("search_type")
				.short("s")
				.long("search_type")
				.value_name("SEARCH_TYPE")
				.possible_values(&crate::algorithm::SearchType::STR_LIST)
				.default_value(crate::algorithm::SearchType::DEFAULT)
				.number_of_values(1)
				.multiple(false)
				.help("The search type used to set the score of each state"),
		)
		.arg(
			clap::Arg::with_name("weight")
				.short("w")
				.long("weight")
				.value_name("FLOATING NUMBER")
				.validator(|raw| {
					raw.parse::<f32>()
						.map(|_| ())
						.map_err(|_| String::from("not a valid number"))
				})
				.number_of_values(1)
				.multiple(false)
				.help("The weight applied to the distance when computing the score (best first search type only), the cost has always a weight of 1. Leave this parameter unset to use a dynamic weight"),
		)
		.arg(
			clap::Arg::with_name("dimension")
				.short("d")
				.long("dimension")
				.value_name("NUMBER")
				.possible_values(&["3", "4", "5", "6", "7"])
				.hide_possible_values(true)
				.number_of_values(1)
				.multiple(false)
				.conflicts_with("file")
				.help("The dimension of the puzzle to generate, it will have dimension x dimension cells"),
		)
		.arg(
			clap::Arg::with_name("without_solution")
				.short("W")
				.long("without_solution")
				.takes_value(false)
				.conflicts_with("file")
				.help("Specify the generated puzzle to not have a solution"),
		)
		.arg(
			clap::Arg::with_name("iterations")
				.short("i")
				.long("iterations")
				.value_name("NUMBER")
				.number_of_values(1)
				.multiple(false)
				.validator(|raw| {
					raw.parse::<usize>()
						.map(|_| ())
						.map_err(|_| String::from("not a valid number"))
				})
				.conflicts_with("file")
				.help("The number of iterations to do when generating a puzzle"),
		)
		.get_matches()
}
