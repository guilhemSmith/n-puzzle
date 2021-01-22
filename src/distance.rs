use crate::puzzle::State;

pub type DistanceFn = fn(&State, &State) -> i32;

pub fn manhattan(a: &State, b: &State) -> i32 {
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
