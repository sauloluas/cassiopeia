#[derive(Debug, Clone, PartialEq)]
pub enum DyadOp {
	Plus,
	Times,
	Minus,
	Frac,
	Power,
}

impl std::fmt::Display for DyadOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use DyadOp::*;
		
		let op = match self {
			Plus => "+",
			Times => "*",
			Minus => "-",
			Frac => "/",
			Power => "^",
		};
		write!(f, "{op}")
	}
}