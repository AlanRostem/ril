mod stack;
mod tokenizer;
mod parser;

fn main() -> Result<(), &'static str> {
    let t = tokenizer::Tokenizer::new();
    let mut p = parser::Parser::new();
    let tokens = match t.tokenize_file("testdata/main.ril") {
        Ok(tks) => tks,
        Err(_) => return Err("failed to tokenize file"),
    };
    if !p.parse(&tokens) {
        std::println!("failed to parse tokens")
    }
    Ok(())
}
