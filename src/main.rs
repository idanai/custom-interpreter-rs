// Interpreter

use std::env;

mod token;
mod operator;
use token::Token;
use operator::Operator;

fn main() {
    let mut stack = vec![];

	let mut s = String::new();
	for arg in env::args().skip(1) {
		s += &arg;
		s.push(' ');
	}

	token::tokenize(&s.as_bytes(), &mut stack, None);

	dbg!(stack);
}

