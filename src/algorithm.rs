use crate::distance;
use crate::puzzle;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn a_star(
	mut start: puzzle::State,
	goal: puzzle::State,
	distance: distance::Method,
) -> Option<Vec<puzzle::State>> {
	let mut closed_queue: Vec<puzzle::State> = Vec::new();
	let mut open_queue: BinaryHeap<Reverse<puzzle::State>> = BinaryHeap::new();

	*(start.heuristic_mut()) = distance(&start, &goal);
	open_queue.push(Reverse(start));
	while let Some(Reverse(current_state)) = open_queue.pop() {
		if current_state.cells() == goal.cells() {
			closed_queue.push(current_state);
			return Some(build_solution(closed_queue));
		}
		for mut neighbor in current_state.neighbors() {
			if !(closed_queue
				.iter()
				.any(|state| state.cells() == neighbor.cells())
				|| open_queue
					.iter()
					.find(|Reverse(state)| state.cells() == neighbor.cells())
					.map_or(false, |Reverse(state)| state.cost() < neighbor.cost()))
			{
				*(neighbor.cost_mut()) = current_state.cost() + 1;
				*(neighbor.heuristic_mut()) = neighbor.cost() + distance(&neighbor, &goal);
				*(neighbor.predecessor_mut()) = Some(closed_queue.len());
				open_queue.push(Reverse(neighbor));
			}
		}
		closed_queue.push(current_state);
	}
	return None;
}

fn build_solution(closed_queue: Vec<puzzle::State>) -> Vec<puzzle::State> {
	let mut solution = Vec::new();
	let mut current_state = closed_queue.last().unwrap();
	while let Some(index) = *current_state.predecessor() {
		solution.push(current_state.clone());
		current_state = &closed_queue[index];
	}
	solution.push(current_state.clone());
	return solution;
}
