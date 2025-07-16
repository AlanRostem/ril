mod tokenizer;
mod stack;
mod interpreter;

fn main() -> Result<(), String> {
    let mut i = interpreter::Interpreter::new();
    i.interpret_file("./testdata/main.ril")?;
    Ok(())
}
