mod parser;
mod tokenizer;

fn main() -> Result<(), String> {
    // let s = "select * from foo.bar";
    let s = "select piyo";
    let t = tokenizer::Tokenizer::new(s);
    let tokens = t.tokenize()?;
    for token in &tokens {
        println!("{:?}", token);
    }
    let parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    println!("{:#?}", ast);
    Ok(())
}
