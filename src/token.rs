use std::collections::HashMap;

use crate::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	None,
	Operator(Operator),
	Number(f32),
	Name(String),
	String(String),
	Vec(Vec<Token>),
	Map(HashMap<String, Token>),
}

pub fn tokenize(bytes: &[u8], stack: &mut Vec<Token>, until: Option<u8>) -> usize {
    let mut skip = 0; // bytes to skip because of internal parsing between iterations
    let mut iterator = bytes.iter()
		.enumerate()
		.filter(|(_, &b)| !b.is_ascii_whitespace()); // filter out whitespaces
	while let Some((count, &byte)) = iterator.nth(skip) {
		if let Some(until) = until {
			if byte == until {
				return count + 1;
			}
		}
	    skip = 0;
		match byte {
			// parse a name
			b'a'..=b'z' | b'A'..=b'Z' => {
				skip = do_name(&bytes[count+1..]);
				let string = String::from_utf8_lossy(&bytes[count..=count+skip]).into_owned(); // todo replace with from_utf8_unchecked()
				dbg!("Got a name: {}", &string);
				stack.push(Token::Name(string));
			}
			// parse a number
			b'0'..=b'9' => {
				let mut number = (byte - b'0') as f32;
				skip = do_digits(&bytes[count+1..], &mut number);
				dbg!("Got a number: {}", &number);
				stack.push(Token::Number(number));
			}
			// read a string
			b'"' | b'\'' => {
				let start = count + 1;
				let Some(index) = bytes[start..].iter().position(|&b| b == byte) else {
					panic!("Missing quotation marks ({})", byte as char);
				};
				skip = index + 1;
				let string = String::from_utf8_lossy(&bytes[start..start+index]).into_owned(); // todo replace with from_utf8_unchecked()
				dbg!("Got a string: {}", &string);
				stack.push(Token::String(string));
			}
			b'(' => {
				skip = tokenize(&bytes[count+1..], stack, Some(b')'));
			}
			// parse operators such as: *, /, +, -, &, ...
			_ if Operator::get_ascii_samples().as_bytes().contains(&byte) => {
				let op = Operator::from_ascii(byte);
				dbg!("Got an operator: {:?}", op);
				stack.push(Token::Operator(op));
			}
			_ => {
				panic!("Unkown syntax: used a character in a weird place: ({})", byte as char);
			}
		}
	}
	bytes.len()
}

fn do_name(input: &[u8]) -> usize {
	for (count, b) in input.iter().enumerate() {
		match b {
			b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {}
			_ => return count,
		}
	}
	0
}

fn do_digits(input: &[u8], output: &mut f32) -> usize {
	let mut count = 0; // number of bytes parsed
	let mut value = *output;
	for c in input {
		match c {
			b'0'..=b'9' => {
				count += 1;
				let n = (c - b'0') as f32;
				value = value * 10.0 + n;
			}
			b'.' => {
				count += 1;
				let mut coef = 0.1;
				for c in &input[count..] {
					match c {
						b'0'..=b'9' => {
							count += 1;
							let n = (*c - b'0') as f32;
							value += n * coef;
							coef *= 0.1;
						}
						_ => {
							*output = value;
							return count;
						}
					}
				}
			}
			_ => {
				*output = value;
				return count
			}
		}
	}
	0
}