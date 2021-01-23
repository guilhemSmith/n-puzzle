mod algorithm;
mod distance;
mod generation;
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
    if let Some(solution) = algorithm::a_star(start, goal, distance::manhattan) {
        println!("solution found !");
        for step in solution {
            println!("\n --- \n{}", step);
        }
    } else {
        println!("no solution found !");
    }
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
