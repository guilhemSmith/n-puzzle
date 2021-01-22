use std::fmt;

#[derive(Debug)]
pub struct State {
	cells: Vec<u8>,
	size: usize,
}

impl State {
	pub fn new(size: usize, cells: Vec<u8>) -> Self {
		if cells.len() != size * size {
			panic!("Trying to create incorrectly sized state.")
		}
		State { cells, size }
	}

	fn access(&self, x: usize, y: usize) -> u8 {
		self.cells[x + y * self.size]
	}
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = String::from("");
		for i in 0..self.size {
			output.push('|');
			for j in 0..self.size {
				output += &format!(
					" {:^size$} ",
					self.access(j, i),
					size = if self.size > 3 { 3 } else { 1 }
				)
			}
			output.push('|');
			if i < self.size - 1 {
				output.push('\n');
			}
		}
		write!(f, "{}", output)
	}
}
