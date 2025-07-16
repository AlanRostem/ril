use crate::stack;
use crate::tokenizer;

const STACK_SIZE: usize = 256;
type ProgramStack = stack::Stack<u32, STACK_SIZE>;

pub struct Interpreter {
    tokenizer: tokenizer::Tokenizer,
    stack: ProgramStack,
    program_counter: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            tokenizer: tokenizer::Tokenizer::new(),
            stack: ProgramStack::new(),
            program_counter: 0,
        }
    }

    pub fn interpret_file(&mut self, path: &'static str) -> Result<(), &'static str> {
        let tokens = match self.tokenizer.tokenize_file(path) {
            Ok(tks) => tks,
            Err(e) => return Err(e),
        };
        self.interpret_tokens(&tokens)?;
        Ok(())
    }

    fn interpret_tokens(&mut self, tokens: &tokenizer::VecTokens) -> Result<(), &'static str> {
        self.program_counter = 0;
        while self.program_counter < tokens.len() {
            let token = tokens.get(self.program_counter).expect("panic");
            match token {
                tokenizer::Token::Operation(op_token) => self.do_op(&op_token, &tokens)?,
                tokenizer::Token::ValueInt(_) => panic!("unexpected integer"),
            }
            self.program_counter += 1
        }
        Ok(())
    }

    fn do_op(
        &mut self,
        op: &tokenizer::Operation,
        tokens: &tokenizer::VecTokens,
    ) -> Result<(), &'static str> {
        match op {
            tokenizer::Operation::Add => {
                let rhs = self.stack.pop()?;
                let lhs = self.stack.pop()?;
                self.stack.push(rhs + lhs)?;
            }
            tokenizer::Operation::Push => {
                self.program_counter += 1;
                if self.program_counter == tokens.len() {
                    panic!("expected an integer at the end of file")
                }
                let next_token = tokens.get(self.program_counter).expect("panic");
                if let tokenizer::Token::ValueInt(int_token) = next_token {
                    self.stack.push(*int_token)?;
                } else {
                    panic!("expected next token to be an integer")
                }
            }
            tokenizer::Operation::Print => {
                let val = self.stack.pop()?;
                println!("{}", val);
            }
        }
        Ok(())
    }
}
