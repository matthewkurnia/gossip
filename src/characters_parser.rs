use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/characters.pest"]
pub struct CharactersParser;

pub fn get_tokens(contents: &String) {
    let test = CharactersParser::parse(Rule::names, &contents)
        .expect("Parsing error.")
        .next()
        .unwrap();

    for t in test.into_inner() {
        match t.as_rule() {
            Rule::names => {
                for name in t.into_inner() {
                    println!("{}", name.as_str());
                }
            }
            Rule::name => {
                println!("{}", t.as_str());
            }
            _ => println!("ASDAFSDASDF"),
        }
    }
}
