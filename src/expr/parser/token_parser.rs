use crate::{expr::DyadOp, Expr};
use super::{ParseError, ParseRlt, Token};


// Token Parser

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
						Token::Plus => Expr::Dyad(
							DyadOp::Plus,
							Box::new(expr),
							Box::new(right)
						),
						Token::Minus => Expr::Dyad(
							DyadOp::Minus,
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
						Token::Times => Expr::Dyad(
							DyadOp::Times,
							Box::new(expr), 
							Box::new(right)
						),
						Token::Frac => Expr::Dyad(
							DyadOp::Frac,
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
