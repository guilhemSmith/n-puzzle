mod algorithm;
mod puzzle;

fn main() {
    let start = puzzle::State::new(3, vec![1, 2, 3, 7, 8, 4, 6, 5, 0]);
    if let Some(solution) = algorithm::solve(start) {
        println!("solution found !");
        for step in solution {
            println!("\n --- \n{}", step);
        }
    } else {
        println!("no solution found !");
    }
}
