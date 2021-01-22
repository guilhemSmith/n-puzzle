use crate::puzzle::State;

use std::collections::BinaryHeap;

pub fn solve(start: State) -> Option<Vec<State>> {
	let goal = State::goal(start.size());
	let mut closed_queue: Vec<State> = Vec::new();
	let mut open_queue: BinaryHeap<State> = BinaryHeap::new();
	open_queue.push(start);
	while let Some(current_state) = open_queue.pop() {
		if current_state.cells() == goal.cells() {
			closed_queue.push(current_state);
			return Some(closed_queue);
		}
		for mut neighbor in current_state.neighbors() {
			if !(closed_queue
				.iter()
				.any(|state| state.cells() == neighbor.cells())
				|| open_queue
					.iter()
					.find(|state| state.cells() == neighbor.cells())
					.map_or(false, |state| state.cost_g() < neighbor.cost_g()))
			{
				*(neighbor.cost_g_mut()) = current_state.cost_g() + 1;
				*(neighbor.dist_h_mut()) = neighbor.cost_g() + distance(&neighbor, &goal);
				open_queue.push(neighbor);
			}
		}
		closed_queue.push(current_state);
	}
	return None;
}

fn distance(_a: &State, _b: &State) -> i32 {
	return 1; // use real function instead
}
