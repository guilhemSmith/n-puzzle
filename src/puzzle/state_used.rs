use colored::*;
use std::borrow::Borrow;
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::hash::{self, Hash};

#[derive(Debug, Clone)]
pub struct StateUsed {
	cells: Vec<u8>,
	size: usize,
	moved: Option<(i32, i32)>,
	predecessor: Option<Vec<u8>>,
}

fn index(x: i32, y: i32, size: usize) -> usize {
	return x as usize + y as usize * size;
}

impl StateUsed {
	pub fn predecessor(&self) -> &Option<Vec<u8>> {
		&self.predecessor
	}

	pub fn cells(&self) -> &Vec<u8> {
		&self.cells
	}

	pub fn access(&self, x: i32, y: i32) -> u8 {
		self.cells[index(x, y, self.size)]
	}
}

impl From<&super::StateUnknown> for StateUsed {
	fn from(other: &super::StateUnknown) -> StateUsed {
		StateUsed {
			cells: other.cells().clone(),
			size: other.size(),
			moved: other.moved(),
			predecessor: other.predecessor().clone(),
		}
	}
}

impl Hash for StateUsed {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.cells.hash(state);
	}
}

impl PartialEq for StateUsed {
	fn eq(&self, other: &Self) -> bool {
		self.cells == other.cells
	}
}

impl Eq for StateUsed {}

impl Borrow<Vec<u8>> for StateUsed {
	fn borrow(&self) -> &Vec<u8> {
		&self.cells
	}
}

impl fmt::Display for StateUsed {
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
