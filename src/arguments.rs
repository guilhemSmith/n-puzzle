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
				.possible_values(&crate::heuristic::STR_LIST)
				.default_value(crate::heuristic::DEFAULT)
				.number_of_values(1)
				.multiple(false)
				.help("The heuristic used to estimate score of each state"),
		)
		.arg(
			clap::Arg::with_name("size")
				.short("s")
				.long("size")
				.value_name("NUMBER")
				.possible_values(&["3", "4", "5", "6", "7"])
				.hide_possible_values(true)
				.number_of_values(1)
				.multiple(false)
				.conflicts_with("file")
				.help("The size of the puzzle to generate, it will have size x size cells"),
		)
		.arg(
			clap::Arg::with_name("without_solution")
				.short("w")
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
