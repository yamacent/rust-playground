mod ast;

use std::{iter::Peekable, slice::Iter};

use crate::tokenizer::{Keyword, TokenValue};

use super::tokenizer;
use ast::*;

pub struct Parser {
    tokens: Vec<tokenizer::Token>,
}

impl Parser {
    pub fn new(tokens: Vec<tokenizer::Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Ast, String> {
        let mut iter = self.tokens.iter().peekable();
        let stmt = self.parse_stmt(&mut iter)?;
        Ok(Ast { stmt })
    }

    fn parse_stmt(
        &self,
        iter: &mut Peekable<Iter<tokenizer::Token>>,
    ) -> Result<SelectStatement, String> {
        if iter.peek().unwrap().value == TokenValue::Keyword(Keyword::SELECT) {
            iter.next();
            let select = self.parse_select_clause(iter)?;
            return Ok(SelectStatement {
                select,
                // from: FromClause {
                //     value: Value {
                //         value: String::from("bar"),
                //     },
                // },
                from: None,
            });
        }
        Err(String::from("not starts with select"))
    }

    fn parse_select_clause(
        &self,
        iter: &mut Peekable<Iter<tokenizer::Token>>,
    ) -> Result<SelectClause, String> {
        let mut r = vec![];
        loop {
            match iter.peek() {
                None => break,
                Some(t) if t.value == TokenValue::Keyword(Keyword::FROM) => break,
                _ => {
                    let exp = self.parse_expression(iter)?;
                    r.push(exp);
                }
            }
        }
        Ok(SelectClause { exps: r })
    }

    fn parse_expression(
        &self,
        iter: &mut Peekable<Iter<tokenizer::Token>>,
    ) -> Result<Expression, String> {
        let token = iter.next().unwrap();
        match &token.value {
            TokenValue::Ident(i) => Ok(Expression {
                value: Value { value: i.clone() },
            }),
            _ => Err(String::from("cannot parse for now")),
        }
    }
}
