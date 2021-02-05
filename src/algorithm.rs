use crate::heuristic;
use crate::puzzle;

use std::collections::{BinaryHeap, HashSet};

pub fn has_solution(start: &puzzle::StateUnknown, goal: &puzzle::StateUnknown) -> bool {
	let inversions = start.count_inversion(&goal);
	if start.size() % 2 == 0 {
		let empty_row = start.row_of_empty(&goal);
		return (inversions + empty_row) % 2 == 0;
	} else {
		return inversions % 2 == 0;
	}
}

pub fn a_star(
	mut start: puzzle::StateUnknown,
	goal: puzzle::StateUnknown,
	heuristic: heuristic::Method,
) -> puzzle::Solution {
	let mut closed_set: HashSet<puzzle::StateUsed> = HashSet::new();
	let mut open_queue: BinaryHeap<puzzle::StateUnknown> = BinaryHeap::new();
	let mut solution = puzzle::Solution::new();

	*(start.score_mut()) = heuristic(&start, &goal);
	open_queue.push(start);
	while let Some(current_state) = open_queue.pop() {
		let current_used = (&current_state).into();
		if current_state.cells() == goal.cells() {
			return solution.build_solution(closed_set, current_used);
		}
		for mut neighbor in current_state.neighbors() {
			*(neighbor.cost_mut()) = current_state.cost() + 1;
			*(neighbor.score_mut()) = neighbor.cost() + heuristic(&neighbor, &goal);
			if !(closed_set.contains(neighbor.cells())
				|| open_queue.iter().any(|state| {
					state.cells() == neighbor.cells() && state.cost() <= neighbor.cost()
				})) {
				if open_queue
					.iter()
					.any(|state| state.cells() == neighbor.cells())
				{
					let mut tmp = open_queue.into_vec();
					if let Some(index) = tmp
						.iter()
						.position(|state| state.cells() == neighbor.cells())
					{
						tmp.remove(index);
					}
					open_queue = BinaryHeap::from(tmp);
				}
				open_queue.push(neighbor);
			}
		}
		if !closed_set.insert(current_used) {
			return solution;
		}
		solution.update_complexity(closed_set.len() + open_queue.len());
	}
	return solution;
}
