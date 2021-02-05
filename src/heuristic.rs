use crate::puzzle;

pub const DEFAULT: &'static str = "linear_conflict+manhattan";
pub const STR_LIST: [&'static str; 3] = ["manhattan", "hamming", "linear_conflict+manhattan"];
pub const FN_LIST: [Method; 3] = [manhattan, hamming, linear_conflict_manhattan];

pub type Method = fn(&puzzle::StateUnknown, &puzzle::StateUnknown) -> i32;

pub fn arg_to_method(arg: &str) -> Option<Method> {
	if let Some(index) = STR_LIST.iter().position(|txt| txt == &arg) {
		return Some(FN_LIST[index]);
	}
	return None;
}

pub fn manhattan(a: &puzzle::StateUnknown, b: &puzzle::StateUnknown) -> i32 {
	let mut manhattan = 0;
	let n = a.size() as i32;
	for x in 0..n {
		for y in 0..n {
			let value = a.access(x, y);
			if value != 0 {
				let (x_target, y_target) = b.coord(value);
				let dx = (x_target - x).abs();
				let dy = (y_target - y).abs();
				manhattan += dx + dy;
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

pub fn linear_conflict_manhattan(a: &puzzle::StateUnknown, b: &puzzle::StateUnknown) -> i32 {
	let mut manhattan = 0;
	let mut linear_confict = 0;
	let n = a.size() as i32;
	let d = (n * n) as usize;
	let mut aligned: Vec<(bool, bool)> = Vec::with_capacity(d);
	aligned.resize(d, (false, false));
	for x in 0..n {
		for y in 0..n {
			let value = a.access(x, y);
			if value != 0 {
				let (x_target, y_target) = b.coord(value);
				let dx = (x_target - x).abs();
				let dy = (y_target - y).abs();
				manhattan += dx + dy;
				aligned[(x + y * n) as usize] = (dx == 0, dy == 0);
				if dx == 0 {
					for r in y..n {
						a.access(x, r);
					}
				}
				if dy == 0 {}
			}
		}
	}
	for x in 0..n {
		for y in 0..n {
			let value = a.access(x, y);
			if value != 0 {
				let (x_target, y_target) = b.coord(value);
				let (align_x, align_y) = aligned[(x + y * n) as usize];
				if align_x {
					for r in y..n {
						let other = a.access(x, r);
						if other != 0 {
							if let (true, _) = aligned[(x + r * n) as usize] {
								let (_, y_other_target) = b.coord(other);
								if y_other_target < y_target {
									linear_confict += 1;
								}
							}
						}
					}
				}
				if align_y {
					for c in x..n {
						let other = a.access(c, y);
						if other != 0 {
							if let (_, true) = aligned[(c + y * n) as usize] {
								let (x_other_target, _) = b.coord(other);
								if x_other_target < x_target {
									linear_confict += 1;
								}
							}
						}
					}
				}
			}
		}
	}
	return manhattan + 2 * linear_confict;
}
