pub struct Solution {
	time_complexity: usize,
	size_complexity: usize,
	moves: Option<Vec<super::State>>,
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

	pub fn build_solution(self, closed_queue: Vec<super::State>) -> Self {
		let time_complexity = self.time_complexity;
		let size_complexity = self.size_complexity;
		let mut moves = Vec::new();

		let mut current_state = closed_queue.last().unwrap();
		while let Some(index) = *current_state.predecessor() {
			moves.push(current_state.clone());
			current_state = &closed_queue[index];
		}
		moves.push(current_state.clone());
		let moves = Some(moves);

		return Solution {
			time_complexity,
			size_complexity,
			moves,
		};
	}

	pub fn moves(&self) -> &Option<Vec<super::State>> {
		&self.moves
	}

	pub fn time_complexity(&self) -> usize {
		self.time_complexity
	}

	pub fn size_complexity(&self) -> usize {
		self.size_complexity
	}
}
