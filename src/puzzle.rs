use std::fmt;

#[derive(Debug)]
pub struct State {
	cells: Vec<u8>,
	size: usize,
	cost_g: i32,
	dist_h: i32,
}

impl State {
	pub fn new(size: usize, cells: Vec<u8>) -> Self {
		if cells.len() != size * size {
			panic!("Trying to create incorrectly sized state.")
		}
		State {
			cells,
			size,
			cost_g: 0,
			dist_h: 0,
		}
	}

	fn access(&self, x: usize, y: usize) -> u8 {
		self.cells[x + y * self.size]
	}

	pub fn cost_g(&self) -> &i32 {
		&self.cost_g
	}

	pub fn cost_g_mut(&mut self) -> &mut i32 {
		&mut self.cost_g
	}

	pub fn dist_h(&self) -> &i32 {
		&self.dist_h
	}

	pub fn dist_h_mut(&mut self) -> &mut i32 {
		&mut self.dist_h
	}
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = format!("  cost_g: {}\n  dist_h: {}\n", self.cost_g, self.dist_h);
		for i in 0..self.size {
			output.push_str("\n|");
			for j in 0..self.size {
				output += &format!(
					" {:^size$} ",
					self.access(j, i),
					size = if self.size > 3 { 3 } else { 1 }
				)
			}
			output.push('|');
		}
		write!(f, "{}", output)
	}
}
