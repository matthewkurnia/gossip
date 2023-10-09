use std::collections::HashSet;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/characters.pest"]
struct CharactersParser;

pub fn get_characters(contents: &String, characters: &mut HashSet<String>) {
    let names = CharactersParser::parse(Rule::names, &contents)
        .expect("Unsuccessful parse of characters file.")
        .next()
        .unwrap(); // According to https://pest.rs/book/examples/csv.html never fails

    for name in names.into_inner() {
        match name.as_rule() {
            Rule::name => {
                characters.insert(name.as_str().to_string());
            }
            _ => {}
        }
    }
}
