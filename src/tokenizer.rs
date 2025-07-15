use std::{io::Read};

pub enum Operation {
    Add,
    Push,
    Print,
}

pub enum Token {
    Operation(Operation),
    ValueInt(u32),
    // ValueString(String),
}

const OP_STR_ADD: &'static str = "add";
const OP_STR_PUSH: &'static str = "push";
const OP_STR_PRINT: &'static str = "print";

pub type VecTokens = Vec<Token>;
pub type ResultTokens = Result<VecTokens, &'static str>;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn tokenize_file(&self, path: &'static str) -> ResultTokens {
        let res = std::fs::File::open(path);
        let mut file = match res {
            Ok(f) => f,
            Err(_) => return Err("failed to open file"),
        };
        let mut content = String::new();
        let _ = match file.read_to_string(&mut content) {
            Ok(n) => n,
            Err(_) => return Err("failed to read file to string"),
        };
        self.tokenize_string(&content)
    }

    pub fn tokenize_string(&self, content: &String) -> ResultTokens {
        let mut total_tokens = VecTokens::new();
        for line in content.lines() {
            let line_str = String::from(line);
            let mut tokens = match self.tokenize_line(&line_str) {
                Ok(t) => t,
                Err(_) => return Err("failed to tokenize line"),
            };
            total_tokens.append(&mut tokens);
        }
        Ok(total_tokens)
    }

    pub fn tokenize_line(&self, content: &String) -> ResultTokens {
        let mut tokens = VecTokens::new();
        let split = content.split(" ");
        for s in split {
            let checked_token = String::from(s);
            // if checked_token.contains("\"") {
            //     // TODO: validate that the string is closed by a quote as well
            //     tokens.push(Token::ValueString(checked_token));
            //     continue;
            // }
            let _ = match checked_token.parse::<u32>() {
                Ok(i) => {
                    tokens.push(Token::ValueInt(i));
                    continue;
                }
                Err(_) => {}
            };
            // this checks which operation is the token
            if checked_token.eq(OP_STR_ADD) {
                tokens.push(Token::Operation(Operation::Add));
            } else if checked_token.eq(OP_STR_PRINT) {
                tokens.push(Token::Operation(Operation::Print));
            } else if checked_token.eq(OP_STR_PUSH) {
                tokens.push(Token::Operation(Operation::Push));
            } else {
                return Err("unrecognized operation");
            }
        }
        Ok(tokens)
    }
}
