use std::cmp::{self, Eq, Ord, PartialEq, PartialOrd};
use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct State {
	cells: Vec<u8>,
	size: usize,
	cost: i32,
	heuristic: i32,
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
			cost: 0,
			heuristic: 0,
		}
	}

	pub fn goal(size: usize) -> Self {
		let mut cells = vec![0; size * size];
		let mut x = -1;
		let mut y = 0;
		let mut x_dir = 1;
		let mut y_dir = 0;
		let mut val = 0;
		for n in 0..(size as i32) * 2 {
			for _ in 0..(size as i32 - (n + 1) / 2) {
				val += 1;
				cells[index(x + x_dir, y + y_dir, size)] = val % (size * size) as u8;
				x += x_dir;
				y += y_dir;
			}
			mem::swap(&mut x_dir, &mut y_dir);
			x_dir *= -1;
		}
		State {
			cells,
			size,
			cost: 0,
			heuristic: 0,
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

	pub fn access(&self, x: i32, y: i32) -> u8 {
		self.cells[index(x, y, self.size)]
	}

	pub fn coord(&self, value: u8) -> (i32, i32) {
		let index = self.cells.iter().position(|val| val == &value).unwrap();
		return ((index % self.size) as i32, (index / self.size) as i32);
	}

	pub fn cost(&self) -> &i32 {
		&self.cost
	}

	pub fn cost_mut(&mut self) -> &mut i32 {
		&mut self.cost
	}

	pub fn _heuristic(&self) -> &i32 {
		&self.heuristic
	}

	pub fn heuristic_mut(&mut self) -> &mut i32 {
		&mut self.heuristic
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.heuristic.cmp(&other.heuristic)
	}
}

impl PartialEq for State {
	fn eq(&self, other: &Self) -> bool {
		self.heuristic == other.heuristic
	}
}

impl Eq for State {}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = format!("  g: {}\n  h: {}\n", self.cost, self.heuristic);
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
