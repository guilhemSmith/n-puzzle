use std::cmp::{self, Eq, Ord, PartialEq, PartialOrd};
use std::fmt;

#[derive(Debug)]
pub struct State {
	cells: Vec<u8>,
	size: usize,
	cost_g: i32,
	dist_h: i32,
}

fn index(x: i32, y: i32, size: usize) -> usize {
	return x as usize + y as usize * size;
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

	pub fn goal(size: usize) -> Self {
		let mut cells = Vec::with_capacity(size * size);
		for i in 0..size * size {
			cells.push(i as u8);
		}
		State {
			cells,
			size,
			cost_g: 0,
			dist_h: 0,
		}
	}

	pub fn neighbors(&self) -> Vec<Self> {
		let mut neighbors = Vec::new();
		let index_empty_cell = self.cells.iter().position(|val| val == &0_u8).unwrap();
		let x_empty_cell = (index_empty_cell % self.size) as i32;
		let y_empty_cell = (index_empty_cell / self.size) as i32;

		if x_empty_cell > 0 {
			neighbors.push(self.neighbor(x_empty_cell, y_empty_cell, -1, 0));
		}
		if x_empty_cell < (self.size - 1) as i32 {
			neighbors.push(self.neighbor(x_empty_cell, y_empty_cell, 1, 0));
		}
		if y_empty_cell > 0 {
			neighbors.push(self.neighbor(x_empty_cell, y_empty_cell, 0, -1));
		}
		if y_empty_cell < (self.size - 1) as i32 {
			neighbors.push(self.neighbor(x_empty_cell, y_empty_cell, 0, 1));
		}
		return neighbors;
	}

	fn neighbor(&self, x_empty: i32, y_empty: i32, x_neighbor: i32, y_neighbor: i32) -> State {
		let mut cells = self.cells.clone();

		cells.swap(
			index(x_empty, y_empty, self.size),
			index(x_empty + x_neighbor, y_empty + y_neighbor, self.size),
		);

		return State::new(self.size, cells);
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn cells(&self) -> &Vec<u8> {
		&self.cells
	}

	fn access(&self, x: i32, y: i32) -> u8 {
		self.cells[index(x, y, self.size)]
	}

	pub fn cost_g(&self) -> &i32 {
		&self.cost_g
	}

	pub fn cost_g_mut(&mut self) -> &mut i32 {
		&mut self.cost_g
	}

	pub fn _dist_h(&self) -> &i32 {
		&self.dist_h
	}

	pub fn dist_h_mut(&mut self) -> &mut i32 {
		&mut self.dist_h
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.dist_h.cmp(&other.dist_h)
	}
}

impl PartialEq for State {
	fn eq(&self, other: &Self) -> bool {
		self.dist_h == other.dist_h
	}
}

impl Eq for State {}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = format!("  cost_g: {}\n  dist_h: {}\n", self.cost_g, self.dist_h);
		for i in 0..self.size as i32 {
			output.push_str("\n|");
			for j in 0..self.size as i32 {
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
