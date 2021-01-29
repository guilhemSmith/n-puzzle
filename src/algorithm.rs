use crate::heuristic;
use crate::puzzle;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn a_star(
	mut start: puzzle::State,
	goal: puzzle::State,
	heuristic: heuristic::Method,
) -> puzzle::Solution {
	let mut closed_queue: Vec<puzzle::State> = Vec::new();
	let mut seen: HashSet<Vec<u8>> = HashSet::new();
	let mut open_queue: BinaryHeap<Reverse<puzzle::State>> = BinaryHeap::new();
	let mut tocheck: HashSet<Vec<u8>> = HashSet::new();
	let mut solution = puzzle::Solution::new();

	*(start.score_mut()) = heuristic(&start, &goal);
	tocheck.insert(start.cells().clone());
	open_queue.push(Reverse(start));
	while let Some(Reverse(current_state)) = open_queue.pop() {
		tocheck.remove(current_state.cells());
		if current_state.cells() == goal.cells() {
			closed_queue.push(current_state);
			return solution.build_solution(closed_queue);
		}
		for mut neighbor in current_state.neighbors() {
			*(neighbor.cost_mut()) = current_state.cost() + 1;
			*(neighbor.score_mut()) = neighbor.cost() + heuristic(&neighbor, &goal);
			if !(seen.contains(neighbor.cells()) || tocheck.contains(neighbor.cells())) {
				if !tocheck.insert(neighbor.cells().clone()) {
					let mut tmp = open_queue.into_sorted_vec();
					if let Some(index) = tmp.iter().position(|Reverse(state)| {
						state.cells() == neighbor.cells() && state.cost() <= neighbor.cost()
					}) {
						tmp.remove(index);
					}
					open_queue = BinaryHeap::from(tmp);
				}
				*(neighbor.predecessor_mut()) = Some(closed_queue.len());
				open_queue.push(Reverse(neighbor));
			}
		}
		if !seen.insert(current_state.cells().clone()) {
			return solution;
		}
		closed_queue.push(current_state);
		solution.update_complexity(closed_queue.len() + open_queue.len());
	}
	return solution;
}
