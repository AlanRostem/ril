use std::{fmt::Write, fs, io::Read};

pub enum TokenTag {
    Operation, // such as add, push, print
    StaticValueInt, 
    StaticValueString,
}

pub enum Operation {
    Add,
    Push,
    Print,
}

pub enum Token {
    Operation(Operation),
    ValueInt(i32),
    ValueString(String),
}

pub type VecTokens = Vec<Token>;
pub type ResultTokens = Result<VecTokens, &'static str>;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        return Self{};
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
            let mut line_str = String::new();
            let _ = match line_str.write_str(line) {
                Ok(n) => n,
                Err(_) => return Err("failed to write string to line")
            };
            let mut tokens = match self.tokenize_line(&line_str) {
                Ok(t) => t,
                Err(_) => return Err("failed to tokenize line"),
            };
            total_tokens.append(&mut tokens);
        }
        Ok(total_tokens)
    }

    pub fn tokenize_line(&self, content: &String) -> ResultTokens {
        Ok(VecTokens::new()) // TODO: implement tokenization
    }
}