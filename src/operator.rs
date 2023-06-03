#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
	Add,
	Sub,
	Mul,
	Div,
	Eq,
	Not,
	And,
	Or,
	Xor,
}

impl Operator {
	pub fn from_ascii(c: u8) -> Self {
		match c {
			b'+' => Self::Add,
			b'-' => Self::Sub,
			b'*' => Self::Mul,
			b'/' => Self::Div,
			b'=' => Self::Eq,
			b'!' => Self::Not,
			b'&' => Self::And,
			b'|' => Self::Or,
			b'^' => Self::Xor,
			_=> panic!("Can't parse an Operator from characters other than: '{}'", Self::get_ascii_samples()),
		}
	}

	pub fn get_ascii_samples() -> &'static str {
		"+-*/=!&|^"
	}
}