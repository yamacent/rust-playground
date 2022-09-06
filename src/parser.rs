use super::tokenizer;

#[derive(Debug)]
pub struct Ast {}

pub struct Parser {
    tokens: Vec<tokenizer::Token>,
}

impl Parser {
    pub fn new(tokens: Vec<tokenizer::Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Ast, String> {
        Ok(Ast {})
    }
}
