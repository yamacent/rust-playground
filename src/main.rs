fn main() {
    let s = "select   *            from     foo.bar";
    let t = tokenizer::Tokenizer::new(s);
    let tokens = t.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
}

mod tokenizer {
    use std::iter::{Enumerate, Peekable};
    use std::str::Chars;

    #[derive(Debug)]
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

        pub fn tokenize(&self) -> Vec<Token> {
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
                    _ => {
                        r.push(Token {
                            pos: i,
                            value: String::from(c.to_string()),
                        });
                        iter.next();
                    }
                }
            }
            r
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
}
