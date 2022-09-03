use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    SELECT,
    FROM,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Ident(String),
    Keyword(Keyword),
    Asterisk,
    Period,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub pos: usize,
    pub value: TokenValue,
}

pub struct Tokenizer {
    str: String,
}

impl Tokenizer {
    pub fn new(str: &str) -> Tokenizer {
        Tokenizer {
            str: String::from(str),
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, String> {
        let mut iter = self.str.chars().enumerate().peekable();
        let mut r = vec![];
        while iter.peek().is_some() {
            let (i, c) = iter.peek().unwrap().clone();
            match c {
                ' ' => self.skip_whitespaces(&mut iter),
                'A'..='Z' | 'a'..='z' | '_' => {
                    let pos = i;
                    let w = self.take_word(&mut iter);
                    r.push(Token {
                        pos,
                        value: match is_keyword(&w[..]) {
                            Some(kw) => TokenValue::Keyword(kw),
                            None => TokenValue::Ident(w),
                        },
                    });
                }
                '*' | '.' => {
                    r.push(Token {
                        pos: i,
                        value: match c {
                            '*' => TokenValue::Asterisk,
                            '.' => TokenValue::Period,
                            _ => return Err(String::from("")),
                        },
                    });
                    iter.next();
                }
                _ => return Err(format!("{} is not supported", c)),
            }
        }
        Ok(r)
    }

    fn skip_whitespaces(&self, iter: &mut Peekable<Enumerate<Chars>>) {
        loop {
            let val = iter.peek();
            match val {
                Some((_, ' ')) => iter.next(),
                _ => break,
            };
        }
    }

    fn take_word(&self, iter: &mut Peekable<Enumerate<Chars>>) -> String {
        let mut r = String::new();
        loop {
            let val = iter.peek();
            match val {
                Some((_, c)) => {
                    if is_idnt(*c) {
                        r.push(*c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        r
    }
}

fn is_idnt(c: char) -> bool {
    match c {
        'A'..='Z' | 'a'..='z' | '_' => true,
        _ => false,
    }
}

fn is_keyword(s: &str) -> Option<Keyword> {
    match &s.to_lowercase()[..] {
        "select" => Some(Keyword::SELECT),
        "from" => Some(Keyword::FROM),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_tokens(items: &[(usize, TokenValue)]) -> Vec<Token> {
        items
            .iter()
            .map(|(pos, value)| Token {
                pos: *pos,
                value: value.clone(),
            })
            .collect()
    }

    fn assert_tokens(code: &str, expect: &[(usize, TokenValue)]) {
        assert_eq!(
            make_tokens(expect),
            Tokenizer::new(code).tokenize().unwrap()
        );
    }

    #[test]
    fn test_tokenize() {
        assert_tokens(
            "select * from foo.bar",
            &[
                (0, TokenValue::Keyword(Keyword::SELECT)),
                (7, TokenValue::Asterisk),
                (9, TokenValue::Keyword(Keyword::FROM)),
                (14, TokenValue::Ident(String::from("foo"))),
                (17, TokenValue::Period),
                (18, TokenValue::Ident(String::from("bar"))),
            ],
        );
    }

    #[test]
    fn test_many_spaces() {
        assert_tokens(
            "  select  *  from   foo  .   bar   ",
            &[
                (2, TokenValue::Keyword(Keyword::SELECT)),
                (10, TokenValue::Asterisk),
                (13, TokenValue::Keyword(Keyword::FROM)),
                (20, TokenValue::Ident(String::from("foo"))),
                (25, TokenValue::Period),
                (29, TokenValue::Ident(String::from("bar"))),
            ],
        );
    }
}
