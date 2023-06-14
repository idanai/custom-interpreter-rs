use std::{collections::HashMap, fmt::Debug};


#[derive(/* Debug, */ Clone, Copy, PartialEq)]
pub enum Token<'a> {
	Operator(&'a [u8]), // punctuation symbols, defining the syntax
	Number(&'a [u8]), // Digits only
	Name(&'a [u8]), // not a string literal, but a variable's name
	String(&'a [u8]), // Literally a string literal(ly)
}

impl<'a> Token<'a> {
	pub fn bytes(&'a self) -> &'a [u8] {
		match self {
			Self::Operator(s) => *s,
			Self::Number(s) => *s,
			Self::Name(s) => *s,
			Self::String(s) => *s,
		}
	}
}

impl<'a> Debug for Token<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = String::from_utf8_lossy(self.bytes());
		match self {
			Self::Operator(_) => write!(f, "Token::Operator({})", s),
			Self::Number(_) => write!(f, "Token::Number({})", s),
			Self::Name(_) => write!(f, "Token::Name({})", s),
			Self::String(_) => write!(f, "Token::String({})", s),
		}
	}
}

// Scanning/Lexing function. It receives a string of bytes and outputs a steam of tokens (vector)
pub fn lex<'a>(input: &'a [u8], output: &mut Vec<Token<'a>>) {
	let mut iter = input.iter().enumerate().peekable();

	// TODO most match cases do the same thing after matching. So write a generic handler (closure) for those
	while let Some((index, &byte)) = iter.next() {
		match byte {
			b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
				while iter.by_ref().next_if(|(_,&b)| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_')).is_some() {}
				let Some((end,_)) = iter.peek() else {
					output.push(Token::Name(&input[index..]));
					break;
				};
				output.push(Token::Name(&input[index..*end]));
			}
			// IMPORTANT: must come before punctuation (operator) checking because 'quotes' are considered punctuation
			b'"' | b'\'' => {
				while iter.by_ref().next_if(|(_,b)| **b != byte).is_some() {}
				let Some((end, _)) = iter.next() else {
					panic!("Reached end of data and didn't find any closing/matching ({byte})");
				};
				output.push(Token::String(&input[index+1..end]));
			}
			b'0'..=b'9' => {
				while iter.by_ref().next_if(|(_,b)| b.is_ascii_digit()).is_some() {}
				let Some((end, _)) = iter.peek() else {
					output.push(Token::Number(&input[index..]));
					break;
				};
				output.push(Token::Number(&input[index..*end]));
			}
			_ if byte.is_ascii_punctuation() => {
				output.push(Token::Operator(&input[index..=index])); // todo? byte instead of slice?
			}
			_ if byte.is_ascii_whitespace() => continue, // skips whitespaces
			_ => {}
		}
	}
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