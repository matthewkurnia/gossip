use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/variables.pest"]
struct VariablesParser;

pub enum VariableType {
    Int(i32),
    Enum(Vec<String>, String),
}

pub fn get_variables(contents: &String, variables: &mut HashMap<String, VariableType>) {
    let definitions = VariablesParser::parse(Rule::definitions, &contents)
        .expect("Unsuccessful parse of variables file.")
        .next()
        .unwrap(); // According to https://pest.rs/book/examples/csv.html never fails;

    for definition in definitions.into_inner() {
        match definition.as_rule() {
            Rule::definition => {
                let definition_inner = definition.into_inner();
                assert!(definition_inner.len() == 1);
                let def = definition_inner.last().unwrap();

                match def.as_rule() {
                    Rule::int_definition => {
                        let mut def_inner = def.into_inner();
                        assert!(def_inner.len() == 2);

                        let identifier = def_inner.next().unwrap();
                        let value = def_inner.next().unwrap();

                        variables.insert(
                            identifier.as_str().to_string(),
                            VariableType::Int(value.as_str().parse::<i32>().unwrap()),
                        );
                    }
                    Rule::enum_definition => {
                        let mut def_inner = def.into_inner();
                        assert!(def_inner.len() >= 2);

                        let identifier = def_inner.next().unwrap();
                        let mut values = Vec::new();
                        loop {
                            match def_inner.next() {
                                Some(value) => values.push(value.as_str().to_string()),
                                None => break,
                            }
                        }
                        let current_value = values[0].clone();

                        variables.insert(
                            identifier.as_str().to_string(),
                            VariableType::Enum(values, current_value),
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
