mod tokenizer;
mod stack;
mod interpreter;

fn main() -> Result<(), &'static str> {
    let mut i = interpreter::Interpreter::new();
    i.interpret_file("./testdata/main.ril")?;
    Ok(())
}
