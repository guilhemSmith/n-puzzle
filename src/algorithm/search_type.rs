pub type SearchType = fn(i32, i32) -> i32;

impl super::Tool for SearchType {
	const DEFAULT: &'static str = "best_first";
	const STR_LIST: [&'static str; 3] = ["uniform_cost", "greedy", "best_first"];
	const FN_LIST: [Self; 3] = [uniform_cost, greedy, best_first];
}

pub fn best_first(g: i32, h: i32) -> i32 {
	return g + h;
}

pub fn uniform_cost(g: i32, _h: i32) -> i32 {
	return g;
}

pub fn greedy(_g: i32, h: i32) -> i32 {
	return h;
}
