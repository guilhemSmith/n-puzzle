mod algorithm;
mod distance;
mod generation;
mod puzzle;

fn main() {
    let filename = "toast.txt";
    let (size, start) = generation::from_file(filename).unwrap();
    let goal = puzzle::State::goal(size);
    if let Some(solution) = algorithm::a_star(start, goal, distance::manhattan) {
        println!("solution found !");
        for step in solution {
            println!("\n --- \n{}", step);
        }
    } else {
        println!("no solution found !");
    }
}
