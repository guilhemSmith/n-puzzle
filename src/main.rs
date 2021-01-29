mod algorithm;
mod generation;
mod heuristic;
mod puzzle;

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = read_arguments();
    let (size, start) = if let Some(filename) = arguments.value_of("file") {
        generation::from_file(filename)?
    } else {
        println!("puzzle generation not done yet");
        let size = arguments.value_of("size").unwrap_or("3").parse()?;
        let _without_solution = arguments.is_present("without_solution");
        (size, puzzle::State::goal(size))
    };
    let goal = puzzle::State::goal(size);
    let solution = algorithm::a_star(start, goal, heuristic::manhattan);
    let split_line = format!(
        " {:-^size$} ",
        "",
        size = size * if size > 3 { 5 } else { 3 }
    );
    if let Some(moves) = solution.moves() {
        for step in moves.iter().rev() {
            println!("\n{}\n{}", split_line, step);
        }
        println!(
            "\n{}\n\npuzzle solved in {} moves.",
            split_line,
            moves.len() - 1
        );
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

fn read_arguments<'a>() -> clap::ArgMatches<'a> {
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
            clap::Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("NUMBER")
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
        .get_matches()
}
