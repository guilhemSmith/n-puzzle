mod algorithm;
mod puzzle;

fn main() {
    let start = puzzle::State::new(3, vec![1, 2, 0, 3, 4, 5, 6, 7, 8]);
    if let Some(solution) = algorithm::solve(start) {
        println!("solution found !");
        for step in solution {
            println!("\n --- \n{}", step);
        }
    } else {
        println!("no solution found !");
    }
}
