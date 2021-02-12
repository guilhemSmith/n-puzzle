mod heuristic;
pub use heuristic::Heuristic;
mod search_type;
pub use search_type::SearchType;

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

pub trait Tool: Sized + Copy {
	const DEFAULT: &'static str;
	const STR_LIST: [&'static str; 3];
	const FN_LIST: [Self; 3];

	fn get(arg: &str) -> Option<Self> {
		if let Some(index) = Self::STR_LIST.iter().position(|txt| txt == &arg) {
			return Some(Self::FN_LIST[index]);
		}
		return None;
	}
}

pub fn a_star(
	mut start: puzzle::StateUnknown,
	goal: puzzle::StateUnknown,
	distance: Heuristic,
	score: SearchType,
) -> puzzle::Solution {
	let mut closed_set: HashSet<puzzle::StateUsed> = HashSet::new();
	let mut open_queue: BinaryHeap<puzzle::StateUnknown> = BinaryHeap::new();
	let mut solution = puzzle::Solution::new();

	*(start.score_mut()) = score(0, distance(&start, &goal));
	open_queue.push(start);
	while let Some(current_state) = open_queue.pop() {
		let current_used = (&current_state).into();
		if current_state.cells() == goal.cells() {
			return solution.build_solution(closed_set, current_used);
		}
		for mut neighbor in current_state.neighbors() {
			*(neighbor.cost_mut()) = current_state.cost() + 1;
			*(neighbor.score_mut()) = score(*neighbor.cost(), distance(&neighbor, &goal));
			if !(closed_set.contains(neighbor.cells())
				|| open_queue.iter().any(|state| {
					state.cells() == neighbor.cells() && state.cost() <= neighbor.cost()
				})) {
				if let Some(index) = open_queue
					.iter()
					.position(|state| state.cells() == neighbor.cells())
				{
					let mut tmp = open_queue.into_vec();
					tmp.remove(index);
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