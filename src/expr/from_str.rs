
use super::Expr;



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

type ParseRlt<T> = Result<Expr<T>, ParseError>;

impl<T> Token<T> 
where T: Clone
{
	pub fn parse(self) -> ParseRlt<T> {
		match self {
			Token::Num(n) => Ok(Expr::Val(n.clone())),
			Token::Name(n) => Ok(Expr::Var(n.clone())),
			_ => Err(ParseError::UnexpectedToken),
		}
	}
}

#[derive(Debug, Clone)]
pub struct TokenParser<T> {
	tokens: Vec<Token<T>>,
	ptr: usize,
}

impl<T: Clone + PartialEq> TokenParser<T> {
	pub fn new(tokens: Vec<Token<T>>) -> Self {
		TokenParser { 
			tokens, 
			ptr: 0, 
		}
	}

	pub fn split_off(&mut self) -> Self {
		let vec_right = self.tokens.split_off(self.ptr);
		TokenParser::new(vec_right)
	}

	pub fn consume(&mut self) -> Option<&Token<T>> {
		let tok = self.tokens.get(self.ptr);
		self.ptr += 1;
		tok
	}

	pub fn peek(&self) -> Option<&Token<T>> {
		self.tokens.get(self.ptr)
	}

	pub fn expect(&mut self, expected: Token<T>) -> Result<(), ParseError> {
		match self.consume() {
			Some(t) if *t == expected => Ok(()),
			_ => Err(ParseError::UnexpectedToken),
		}
	}

	pub fn run(&mut self) -> ParseRlt<T> {
		self.parse_expr()
	}

	fn parse_expr(&mut self) -> ParseRlt<T> {
		let mut expr = self.parse_term()?;
		while let Some(tok) = self.peek().cloned() {
			match tok {
				Token::Plus | Token::Minus => {
					self.consume();
					let right = self.parse_term()?;
					expr = match tok {
						Token::Plus => Expr::Plus(
							Box::new(expr),
							Box::new(right)
						),
						Token::Minus => Expr::Minus(
							Box::new(expr), 
							Box::new(right)
						),
						_ => unreachable!(),
					};
				}
				_ => break,
			}
		}
		Ok(expr)
	}

	fn parse_term(&mut self) -> ParseRlt<T> {
		let mut expr = self.parse_factor()?;
		while let Some(tok) = self.peek().cloned() {
			match tok {
				Token::Times | Token::Frac => {
					self.consume();
					let right = self.parse_factor()?;
					expr = match tok {
						Token::Times => Expr::Times(
							Box::new(expr), 
							Box::new(right)
						),
						Token::Frac => Expr::Frac(
							Box::new(expr), 
							Box::new(right)
						),
						_ => unreachable!(),
					};

				},
				_ => break,
			}
		}
		Ok(expr)
	}

	fn parse_factor(&mut self) -> ParseRlt<T> {
		match self.consume() {
			Some(Token::Num(n)) => Ok(Expr::Val(n.clone())),
			Some(Token::Name(n)) => Ok(Expr::Var(n.clone())),
			Some(Token::LParen) => {
				let expr = self.parse_expr()?;
				self.expect(Token::RParen)?;
				Ok(expr)
			},
			Some(Token::Minus) => Ok(Expr::Neg(
				Box::new(self.parse_factor()?)
			)),
			_ => Err(ParseError::UnexpectedToken),
		}
	}
}



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
