use super::TokenParser;
use crate::Expr;

// ParseError type

#[derive(Debug)]
pub enum ParseError {
	InvalidNumber(String),
	InvalidCharacter(char),
	UnexpectedToken,
	UnmatchedParenthesis,
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:#?}")
	}
}

impl std::error::Error for ParseError {}



// Tokens to Expr translation

impl<T> std::str::FromStr for Expr<T> 
where 
	T: std::str::FromStr + Clone + PartialEq
{
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
	    let tokens: TokenParser<T> = s.parse()?;
	    Expr::try_from(tokens)
	}
}

impl<T> TryFrom<TokenParser<T>> for Expr<T> 
where T: Clone + PartialEq
{
	type Error = ParseError;

	fn try_from(mut parser: TokenParser<T>) -> Result<Self, Self::Error> {
	    parser.run()
	}
}

// Token type

#[derive(Debug, Clone, PartialEq)]
pub enum Token<T> {
	Num(T),
	Name(String),
	Plus,
	Frac,
	Times,
	Minus,
	Power,
	LParen,
	RParen,
	Equals,
	Space,
}

// Scanner

impl<T> std::str::FromStr for TokenParser<T> 
where T: std::str::FromStr + Clone + PartialEq
{
	type Err = ParseError;

	fn from_str(input: &str) -> Result<TokenParser<T>, ParseError> {
		use Token::*;
		let mut tokens = Vec::new();
		let mut chars = input.chars().peekable();

		while let Some(c) = chars.next() {
			match c {
				' ' => continue,
				'+' => tokens.push(Plus),
				'-' => tokens.push(Minus),
				'/' => tokens.push(Frac),
				'*' => tokens.push(Times),
				'^' => tokens.push(Power),
				'(' => tokens.push(LParen),
				')' => tokens.push(RParen),
				'=' => tokens.push(Equals),
				'0'..='9' | '.' => {
					let mut num = String::from(c);
					let mut scalar = false;
					while let Some(next_c) = chars.peek() {
						if next_c.is_ascii_digit() || *next_c == '.' {
							num.push(*next_c);
							chars.next();
						} else if next_c.is_alphabetic() {
							scalar = true;
							break;
						} else {
							break;
						}
					}
					let value = num.parse::<T>()
						.map_err(|_| ParseError::InvalidNumber(num))?;
					tokens.push(Num(value));
					if scalar {
						tokens.push(Times);
					}
				},
				'a'..='z' | 'A'..'Z' => {
					let mut ident = String::from(c);
					while let Some(next_c) = chars.peek() {
						if next_c.is_alphanumeric() || *next_c == '_' {
							ident.push(*next_c);
							chars.next();
						} else {
							break;
						}
					}
					tokens.push(Name(ident));
				},
				_ => return Err(ParseError::InvalidCharacter(c)),
			}
		}
		Ok(TokenParser::new(tokens))
	}

}
