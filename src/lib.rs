mod parser;
mod tokenizer;
use tokenizer::Tokenizer;

pub fn run_file() {
    let code = "let a = 10;";
    Tokenizer::new(&code).tokenizer();
}
