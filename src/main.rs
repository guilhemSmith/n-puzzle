mod algorithm;
mod distance;
mod puzzle;

fn main() {
    let start = puzzle::State::new(3, vec![1, 2, 3, 7, 8, 4, 6, 5, 0]);
    let goal = puzzle::State::goal(3);
    if let Some(solution) = algorithm::a_star(start, goal, distance::manhattan) {
        println!("solution found !");
        for step in solution {
            println!("\n --- \n{}", step);
        }
    } else {
        println!("no solution found !");
    }
}
