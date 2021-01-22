use crate::distance::DistanceFn;
use crate::puzzle::State;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub type Algo = fn(start: State, goal: State, distance: DistanceFn) -> Option<Vec<State>>;

pub fn a_star(mut start: State, goal: State, distance: DistanceFn) -> Option<Vec<State>> {
	let mut closed_queue: Vec<State> = Vec::new();
	let mut open_queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();

	*(start.heuristic_mut()) = distance(&start, &goal);
	open_queue.push(Reverse(start));
	while let Some(Reverse(current_state)) = open_queue.pop() {
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
					.find(|Reverse(state)| state.cells() == neighbor.cells())
					.map_or(false, |Reverse(state)| state.cost() < neighbor.cost()))
			{
				*(neighbor.cost_mut()) = current_state.cost() + 1;
				*(neighbor.heuristic_mut()) = neighbor.cost() + distance(&neighbor, &goal);
				open_queue.push(Reverse(neighbor));
			}
		}
		closed_queue.push(current_state);
	}
	return None;
}
