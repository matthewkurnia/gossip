use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/variables.pest"]
struct VariablesParser;

pub fn get_tokens(contents: &String) {}
