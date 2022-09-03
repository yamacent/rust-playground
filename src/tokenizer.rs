use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub pos: usize,
    pub value: String,
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
                    r.push(Token { pos, value: w });
                }
                '*' | '.' => {
                    r.push(Token {
                        pos: i,
                        value: String::from(c.to_string()),
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

#[cfg(test)]
mod test {
    use super::*;

    fn make_tokens(items: &[(usize, &str)]) -> Vec<Token> {
        items
            .iter()
            .map(|(pos, value)| Token {
                pos: *pos,
                value: String::from(*value),
            })
            .collect()
    }

    fn assert_tokens(code: &str, expect: &[(usize, &str)]) {
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
                (0, "select"),
                (7, "*"),
                (9, "from"),
                (14, "foo"),
                (17, "."),
                (18, "bar"),
            ],
        );
    }

    #[test]
    fn test_many_spaces() {
        assert_tokens(
            "  select  *  from   foo  .   bar   ",
            &[
                (2, "select"),
                (10, "*"),
                (13, "from"),
                (20, "foo"),
                (25, "."),
                (29, "bar"),
            ],
        );
    }
}
