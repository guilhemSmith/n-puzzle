mod heuristic;
pub use heuristic::Heuristic;
mod search_type;
pub use search_type::SearchType;

use crate::puzzle;

use colored::*;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;

const DYN_WEIGHTS: [f32; 5] = [1.0, 1.35, 2.5, 4.0, 10.0];

pub fn has_solution(start: &puzzle::State, goal: &puzzle::State) -> bool {
	let inversions = start.count_inversion(&goal);
	if start.size() % 2 == 0 {
		let empty_row = start.row_of_empty(&goal);
		return (inversions + empty_row) % 2 == 0;
	} else {
		return inversions % 2 == 0;
	}
}

pub fn dynamic_weight(size: usize) -> f32 {
	if size > 7 {
		10.0
	} else {
		DYN_WEIGHTS[size - 3]
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

	fn pretty_name(argname: &str) -> Option<ColoredString> {
		let lvl = Self::STR_LIST.iter().position(|s| *s == argname)?;
		let clean_name = argname.replace("_", " ").replace("+", " + ");
		let colored_name = match lvl {
			0 => clean_name.red(),
			1 => clean_name.purple(),
			_ => clean_name.blue(),
		};
		return Some(colored_name);
	}
}

pub fn w_a_star(
	mut start: puzzle::State,
	goal: puzzle::State,
	distance: Heuristic,
	score: SearchType,
	weight: f32,
) -> puzzle::Solution {
	let mut closed_set: HashSet<puzzle::State> = HashSet::new();
	let mut open_queue: PriorityQueue<puzzle::State, Reverse<i32>> = PriorityQueue::new();
	let mut solution = puzzle::Solution::new();
	let weight_scaled: i32 = (100.0 * weight).round() as i32;

	let s_score = score(0, distance(&start, &goal), weight_scaled);
	*(start.score_mut()) = s_score;
	open_queue.push(start, Reverse(s_score));
	while let Some((current_state, _)) = open_queue.pop() {
		if current_state.cells() == goal.cells() {
			return solution.build_solution(closed_set, current_state);
		}
		for mut neighbor in current_state.neighbors() {
			*(neighbor.cost_mut()) = current_state.cost() + 1;
			let n_score = score(*neighbor.cost(), distance(&neighbor, &goal), weight_scaled);
			*(neighbor.score_mut()) = n_score;
			if !closed_set.contains(neighbor.cells()) {
				if let Some((state_existing, _)) = open_queue.get(&neighbor) {
					if state_existing.cost() > neighbor.cost() {
						open_queue.push(neighbor, Reverse(n_score));
					}
				} else {
					open_queue.push(neighbor, Reverse(n_score));
				}
			}
		}
		if !closed_set.insert(current_state) {
			return solution;
		}
		solution.update_complexity(closed_set.len() + open_queue.len());
	}
	return solution;
}
