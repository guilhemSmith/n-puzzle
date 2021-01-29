use crate::heuristic;
use crate::puzzle;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn a_star(
	mut start: puzzle::StateUnknown,
	goal: puzzle::StateUnknown,
	heuristic: heuristic::Method,
) -> puzzle::Solution {
	let mut closed_set: HashSet<puzzle::StateUsed> = HashSet::new();
	let mut open_queue: BinaryHeap<Reverse<puzzle::StateUnknown>> = BinaryHeap::new();
	let mut opened: HashSet<Vec<u8>> = HashSet::new();
	let mut solution = puzzle::Solution::new();

	*(start.score_mut()) = heuristic(&start, &goal);
	opened.insert(start.cells().clone());
	open_queue.push(Reverse(start));
	while let Some(Reverse(current_state)) = open_queue.pop() {
		let current_used: puzzle::StateUsed = (&current_state).into();
		opened.remove(current_used.cells());
		if current_state.cells() == goal.cells() {
			return solution.build_solution(closed_set, current_used);
		}
		for mut neighbor in current_state.neighbors() {
			*(neighbor.cost_mut()) = current_state.cost() + 1;
			*(neighbor.score_mut()) = neighbor.cost() + heuristic(&neighbor, &goal);
			if !(closed_set.contains(neighbor.cells()) || opened.contains(neighbor.cells())) {
				if !opened.insert(neighbor.cells().clone()) {
					let mut tmp = open_queue.into_sorted_vec();
					if let Some(index) = tmp.iter().position(|Reverse(state)| {
						state.cells() == neighbor.cells() && state.cost() <= neighbor.cost()
					}) {
						tmp.remove(index);
					}
					open_queue = BinaryHeap::from(tmp);
				}
				open_queue.push(Reverse(neighbor));
			}
		}
		if !closed_set.insert(current_used) {
			return solution;
		}
		solution.update_complexity(closed_set.len() + open_queue.len());
	}
	return solution;
}
