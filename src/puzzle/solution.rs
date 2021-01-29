use std::collections::HashSet;

pub struct Solution {
	time_complexity: usize,
	size_complexity: usize,
	moves: Option<Vec<super::StateUsed>>,
}

impl Solution {
	pub fn new() -> Self {
		Solution {
			time_complexity: 0,
			size_complexity: 0,
			moves: None,
		}
	}

	pub fn update_complexity(&mut self, current_size: usize) {
		self.time_complexity += 1;
		if self.size_complexity < current_size {
			self.size_complexity = current_size;
		}
	}

	pub fn build_solution(
		self,
		mut closed_set: HashSet<super::StateUsed>,
		last_state: super::StateUsed,
	) -> Self {
		let time_complexity = self.time_complexity;
		let size_complexity = self.size_complexity;
		let mut moves = Vec::new();

		let mut current_state = last_state;
		while let Some(prev) = current_state.predecessor() {
			let next = closed_set.take(prev);
			moves.push(current_state);
			current_state = next.unwrap();
		}
		moves.push(current_state);
		let moves = Some(moves);

		return Solution {
			time_complexity,
			size_complexity,
			moves,
		};
	}

	pub fn moves(&self) -> &Option<Vec<super::StateUsed>> {
		&self.moves
	}

	pub fn time_complexity(&self) -> usize {
		self.time_complexity
	}

	pub fn size_complexity(&self) -> usize {
		self.size_complexity
	}
}
