use core::panic;

mod parser;
mod stack;
mod tokenizer;

const STACK_SIZE: usize = 32;
type ProgramStack = stack::Stack<u32, STACK_SIZE>;

fn do_operation(
    i: &mut usize,
    op: &tokenizer::Operation,
    tokens: &tokenizer::VecTokens,
    stack: &mut ProgramStack,
) -> Result<(), &'static str> {
    match op {
        tokenizer::Operation::Add => {
            let rhs = stack.pop()?;
            let lhs = stack.pop()?;
            stack.push(rhs + lhs)?;
        },
        tokenizer::Operation::Push => {
            *i += 1;
            if *i == tokens.len() {
                panic!("expected an integer at the end of file")
            }
            let next_token = tokens.get(*i).expect("panic");
            if let tokenizer::Token::ValueInt(int_token) = next_token {
                stack.push(*int_token)?;
            } else {
                panic!("expected next token to be an integer")
            }
        }
        tokenizer::Operation::Print => {
            let val = stack.pop()?;
            println!("{}", val);
        },
    }
    Ok(())
}

fn main() -> Result<(), &'static str> {
    let t = tokenizer::Tokenizer::new();
    let tokens = match t.tokenize_file("./testdata/main.ril") {
        Ok(tks) => tks,
        Err(e) => return Err(e),
    };
    let mut program_stack = ProgramStack::new();
    let mut i: usize = 0;
    while i < tokens.len() {
        let token = tokens.get(i).expect("panic");
        match token {
            tokenizer::Token::Operation(op_token) => {
                do_operation(&mut i, op_token, &tokens, &mut program_stack)?
            }
            tokenizer::Token::ValueInt(_) => panic!("unexpected integer"),
        }
        i += 1
    }

    Ok(())
}
