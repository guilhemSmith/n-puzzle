use crate::puzzle;

use std::fs;
use std::io::{self, BufRead};

pub fn from_file(filename: &str) -> Option<(usize, puzzle::State)> {
	let file = fs::File::open(filename).ok()?;
	let reader = io::BufReader::new(file);
	let mut size = None;
	let mut cells = Vec::new();

	for line in reader.lines() {
		let line = line.ok()?;
		if size.is_none() {
			size = read_size(line);
			if size.is_some() {
				cells = Vec::with_capacity(size? * size?);
			}
		} else {
			cells.append(&mut read_cells(line, size?));
		}
	}
	return Some((size?, puzzle::State::new(size?, cells)));
}

fn read_size(line: String) -> Option<usize> {
	let cleared_line = clear_line(&line);

	if cleared_line.len() == 1 {
		return cleared_line[0].parse().ok();
	}
	return None;
}

fn read_cells(line: String, size: usize) -> Vec<u8> {
	let mut row = Vec::with_capacity(size);
	let cleared_line = clear_line(&line);

	if cleared_line.len() == size {
		for cell in cleared_line {
			row.push(cell.parse().unwrap());
		}
	}
	return row;
}

fn clear_line<'a>(line: &'a String) -> Vec<&'a str> {
	line.split('#').next().unwrap().split_whitespace().collect()
}
