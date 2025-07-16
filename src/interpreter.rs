use crate::stack;
use crate::tokenizer;

use std::collections::HashMap;

const STACK_SIZE: usize = 256;
type ProgramStack = stack::Stack<u32, STACK_SIZE>;
type LabelMap = HashMap<String, usize>;

pub struct Interpreter {
    tokenizer: tokenizer::Tokenizer,
    stack: ProgramStack,
    program_counter: usize,
    label_map: LabelMap,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            tokenizer: tokenizer::Tokenizer::new(),
            stack: ProgramStack::new(),
            program_counter: 0,
            label_map: LabelMap::new(),
        }
    }

    pub fn interpret_file(&mut self, path: &'static str) -> Result<(), String> {
        let tokens = match self.tokenizer.tokenize_file(path) {
            Ok(tks) => tks,
            Err(e) => return Err(e),
        };
        self.interpret_tokens(&tokens)?;
        Ok(())
    }

    fn scan_labels(&mut self, tokens: &tokenizer::VecTokens) {
        let mut i: usize = 0;
        for token in tokens {
            if let tokenizer::Token::Label(label) = token {
                self.label_map.insert(label.clone(), i);
            }
            i += 1;
        }
    }

    fn interpret_tokens(&mut self, tokens: &tokenizer::VecTokens) -> Result<(), String> {
        self.scan_labels(tokens);
        self.program_counter = 0;
        while self.program_counter < tokens.len() {
            let token = tokens.get(self.program_counter).expect("panic");
            match token {
                tokenizer::Token::Operation(op_token) => self.do_op(&op_token, &tokens)?,
                tokenizer::Token::ValueInt(_) => panic!("unexpected integer"),
                tokenizer::Token::Label(label) => {
                    self.label_map.insert(label.clone(), self.program_counter);
                }
            }
            self.program_counter += 1
        }
        Ok(())
    }

    fn do_op(
        &mut self,
        op: &tokenizer::Keyword,
        tokens: &tokenizer::VecTokens,
    ) -> Result<(), String> {
        match op {
            tokenizer::Keyword::Add => {
                let rhs = self.stack.pop()?;
                let lhs = self.stack.pop()?;
                self.stack.push(rhs + lhs)?;
            }
            tokenizer::Keyword::Push => {
                if !self.increment_pc_intermediary(tokens) {
                    return Err(String::from("expected an integer at the end of file"));
                }
                let next_token = tokens.get(self.program_counter).expect("panic");
                if let tokenizer::Token::ValueInt(int_token) = next_token {
                    self.stack.push(*int_token)?;
                } else {
                    return Err(String::from("expected next token to be an integer"));
                }
            }
            tokenizer::Keyword::Print => {
                let val = self.stack.pop()?;
                println!("{}", val);
                self.stack.push(val)?;
            }
            tokenizer::Keyword::JumpIf => {
                if !self.increment_pc_intermediary(tokens) {
                    return Err(String::from("expected a label at the end of file"));
                }
                let mut next_token = tokens.get(self.program_counter).expect("panic");
                if let tokenizer::Token::Label(label) = next_token {
                    let found = self.label_map.get(label);
                    if found.is_none() {
                        return Err(format!("label does not exist: {}", label));
                    }
                    self.program_counter += 1;
                    if self.program_counter == tokens.len() {
                        // TODO: use the method (why does this not compile?)
                        return Err(String::from("expected an integer at the end of file"));
                    }
                    next_token = tokens.get(self.program_counter).expect("panic");
                    if let tokenizer::Token::ValueInt(int_token) = next_token {
                        let pc = *found.clone().unwrap();
                        let value = self.stack.pop()?;
                        if value == *int_token {
                            self.program_counter = pc;
                        }
                        self.stack.push(value)?;
                    } else {
                        return Err(String::from("expected next token to be an integer"));
                    }
                } else {
                    return Err(String::from("expected next token to be a label"));
                }
            }
            tokenizer::Keyword::Jump => {
                if !self.increment_pc_intermediary(tokens) {
                    return Err(String::from("expected a label at the end of file"));
                }
                let next_token = tokens.get(self.program_counter).expect("panic");
                if let tokenizer::Token::Label(label) = next_token {
                    let found = self.label_map.get(label);
                    if found.is_none() {
                        return Err(format!("label does not exist: {}", label));
                    }
                    let pc = *found.clone().unwrap();
                    self.program_counter = pc;
                }
            }
        }
        Ok(())
    }

    // Increment during the loop when expecting an operation in the next step. Returns an error if we are at the end.
    fn increment_pc_intermediary(&mut self, tokens: &tokenizer::VecTokens) -> bool {
        self.program_counter += 1;
        return self.program_counter < tokens.len();
    }
}
