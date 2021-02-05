use crate::puzzle;

pub type Method = fn(&puzzle::StateUnknown, &puzzle::StateUnknown) -> i32;

pub fn manhattan(a: &puzzle::StateUnknown, b: &puzzle::StateUnknown) -> i32 {
	let mut manhattan = 0;
	let n = a.size() as i32;
	for x in 0..n {
		for y in 0..n {
			let value = a.access(x, y);
			if value != 0 {
				let (x_target, y_target) = b.coord(value);
				let dx = x_target - x;
				let dy = y_target - y;
				manhattan += dx.abs() + dy.abs();
			}
		}
	}
	return manhattan;
}

pub fn hamming(a: &puzzle::StateUnknown, b: &puzzle::StateUnknown) -> i32 {
	let mut hamming = 0;
	let n = a.size() as i32;
	for x in 0..n {
		for y in 0..n {
			let value = a.access(x, y);
			if value != 0 {
				let (x_target, y_target) = b.coord(value);
				if x_target != x && y_target != y {
					hamming += 1;
				}
			}
		}
	}
	return hamming;
}
