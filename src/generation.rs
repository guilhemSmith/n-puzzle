use crate::puzzle;

use std::convert;
use std::error;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::num;

pub fn random(size: usize, solvable: bool, iteration: usize) -> puzzle::State {
	let mut state = puzzle::State::goal(size);

	for _ in 0..iteration {
		state.shuffle();
	}
	if !solvable {
		state.corrupt();
	}

	return state;
}

pub fn from_file(filename: &str) -> Result<(usize, puzzle::State), Error> {
	let file = fs::File::open(filename)?;
	let reader = io::BufReader::new(file);
	let mut size = None;
	let mut cells = Vec::new();

	for line in reader.lines() {
		let line = line?;
		if size.is_none() {
			size = read_size(line)?;
			if size.is_some() {
				let vec_size = size
					.ok_or(Error::NoSizeSpecified)
					.and_then(|s| (s > 2).then(|| s).ok_or(Error::SizeNotBigEnough(s)))?;
				cells = Vec::with_capacity(vec_size * vec_size);
			}
		} else {
			cells.append(&mut read_cells(line, size.ok_or(Error::NoSizeSpecified)?)?);
		}
	}
	let fixed_size = size.ok_or(Error::NoSizeSpecified)?;
	if cells.len() != fixed_size * fixed_size {
		return Err(Error::BadPuzzle);
	}
	return Ok((fixed_size, puzzle::State::new(fixed_size, cells)));
}

fn read_size(line: String) -> Result<Option<usize>, Error> {
	let cleared_line = clear_line(&line);

	if cleared_line.len() > 0 {
		if cleared_line.len() > 1 {
			return Err(Error::NoSizeSpecified);
		}
		return Ok(Some(cleared_line[0].parse()?));
	}
	return Ok(None);
}

fn read_cells(line: String, size: usize) -> Result<Vec<u8>, Error> {
	let mut row = Vec::with_capacity(size);
	let cleared_line = clear_line(&line);

	let row_size = cleared_line.len();
	if row_size > 0 {
		if row_size != size {
			return Err(Error::SizeNotRespected(size, row_size));
		}
		for cell in cleared_line {
			row.push(cell.parse()?);
		}
	}
	return Ok(row);
}

fn clear_line<'a>(line: &'a String) -> Vec<&'a str> {
	line.split('#').next().unwrap().split_whitespace().collect()
}

#[derive(Debug)]
pub enum Error {
	NoSizeSpecified,
	FailedFileReading(io::Error),
	SizeNotRespected(usize, usize),
	SizeNotBigEnough(usize),
	InvalidNumber(num::ParseIntError),
	BadPuzzle,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return match self {
			Error::NoSizeSpecified => write!(f, "An invalid size was specified for this puzzle"),
			Error::SizeNotBigEnough(size) => write!(
				f,
				"This puzzle has a size too small (got {}, expected 3 or more)",
				size
			),
			Error::FailedFileReading(io_err) => write!(f, "Could not read the file: {}", io_err),
			Error::SizeNotRespected(expected, found) => write!(
				f,
				"File do not respect specified line size, expected {} numbers, got {}",
				expected, found
			),
			Error::InvalidNumber(parse_error) => {
				write!(f, "Invalid Number could not be parsed: {}", parse_error)
			}
			Error::BadPuzzle => write!(
				f,
				"An invalid amount of values was specified for this puzzle"
			),
		};
	}
}

impl convert::From<io::Error> for Error {
	fn from(io_error: io::Error) -> Error {
		return Error::FailedFileReading(io_error);
	}
}

impl convert::From<num::ParseIntError> for Error {
	fn from(parse_error: num::ParseIntError) -> Error {
		return Error::InvalidNumber(parse_error);
	}
}

impl error::Error for Error {}
