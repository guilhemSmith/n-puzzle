use colored::*;
use rand::prelude::*;
use std::cmp::{self, Eq, Ord, PartialEq, PartialOrd};
use std::fmt;
use std::mem;

#[derive(Debug, Clone)]
pub struct StateUnknown {
	cells: Vec<u8>,
	size: usize,
	cost: i32,
	score: i32,
	moved: Option<(i32, i32)>,
	predecessor: Option<Vec<u8>>,
}

fn index(x: i32, y: i32, size: usize) -> usize {
	return x as usize + y as usize * size;
}

impl StateUnknown {
	pub fn new(size: usize, cells: Vec<u8>) -> Self {
		if cells.len() != size * size {
			panic!("Trying to create incorrectly sized state.")
		}
		StateUnknown {
			cells,
			size,
			cost: 0,
			score: 0,
			moved: None,
			predecessor: None,
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
		StateUnknown {
			cells,
			size,
			cost: 0,
			score: 0,
			moved: None,
			predecessor: None,
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

	fn neighbor(
		&self,
		x_empty: i32,
		y_empty: i32,
		x_neighbor: i32,
		y_neighbor: i32,
	) -> StateUnknown {
		let mut cells = self.cells.clone();

		cells.swap(
			index(x_empty, y_empty, self.size),
			index(x_empty + x_neighbor, y_empty + y_neighbor, self.size),
		);

		return StateUnknown {
			cells,
			size: self.size,
			cost: 0,
			score: 0,
			moved: Some((x_empty, y_empty)),
			predecessor: Some(self.cells.clone()),
		};
	}

	pub fn shuffle(&mut self) {
		let mut neighbors = self.neighbors();
		let choice = random::<usize>() % neighbors.len();
		self.cells = neighbors.swap_remove(choice).cells;
	}

	pub fn corrupt(&mut self) {
		if self.cells[0] == 0 || self.cells[1] == 0 {
			self.cells.swap(self.size - 1, self.size - 2);
		} else {
			self.cells.swap(0, 1);
		}
	}

	pub fn predecessor(&self) -> &Option<Vec<u8>> {
		&self.predecessor
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

	pub fn moved(&self) -> Option<(i32, i32)> {
		self.moved
	}

	pub fn _score(&self) -> &i32 {
		&self.score
	}

	pub fn score_mut(&mut self) -> &mut i32 {
		&mut self.score
	}

	pub fn count_inversion(&self, goal: &Self) -> i32 {
		let mut inv = 0;

		for (i, fake_val) in self.cells.iter().enumerate() {
			if *fake_val != 0 {
				let (x_i, y_i) = goal.coord(*fake_val);
				let val_i = index(x_i, y_i, self.size);
				for j in 0..i {
					let (x_j, y_j) = goal.coord(self.cells[j]);
					let val_j = index(x_j, y_j, self.size);
					if self.cells[j] > 0 && val_j > val_i {
						inv += 1;
					}
				}
			}
		}
		return inv;
	}

	pub fn row_of_empty(&self, goal: &Self) -> i32 {
		let (_, y0) = self.coord(0);
		let (_, y1) = goal.coord(0);
		return (y1 - y0).abs();
	}
}

impl PartialOrd for StateUnknown {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for StateUnknown {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.score.cmp(&other.score)
	}
}

impl PartialEq for StateUnknown {
	fn eq(&self, other: &Self) -> bool {
		self.score == other.score
	}
}

impl Eq for StateUnknown {}

impl fmt::Display for StateUnknown {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = String::new();
		for i in 0..self.size as i32 {
			output = format!("{}{}", output, "\n|".dimmed());
			for j in 0..self.size as i32 {
				let value = self.access(j, i);
				let value_colored = match self.moved {
					Some((x, y)) if x == j && y == i || value == 0 => format!("{}", value).green(),
					_ => format!("{}", value).white(),
				};
				output = format!(
					"{} {:^size$} ",
					output,
					value_colored,
					size = if self.size > 3 { 3 } else { 1 }
				)
			}
			output = format!("{}{}", output, "|".dimmed());
		}
		write!(f, "{}", output)
	}
}
