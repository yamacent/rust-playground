mod tokenizer;

fn main() {
    let s = "select * from foo.bar";
    let t = tokenizer::Tokenizer::new(s);
    let tokens = t.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
}
