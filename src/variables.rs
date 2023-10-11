use pest::Parser;
use pest_derive::Parser;
use std::{collections::HashMap, path::Path};

#[derive(Parser)]
#[grammar = "grammars/variables.pest"]
struct VariablesParser;

pub enum Variable {
    Int(i32),
    Enum(Vec<String>, String),
}

pub fn get_variables(
    contents: &String,
    variables: &mut HashMap<String, Variable>,
    file_path: &Path,
) {
    let definitions = VariablesParser::parse(Rule::definitions, &contents)
        .expect("Unsuccessful parse of variables file.")
        .next()
        .unwrap(); // According to https://pest.rs/book/examples/csv.html never fails;

    for definition in definitions.into_inner() {
        match definition.as_rule() {
            Rule::definition => {
                let definition = definition.into_inner();
                assert!(definition.len() == 1);

                let definition = definition.last().unwrap();
                let rule = definition.as_rule().clone();

                let mut def_inner = definition.into_inner();
                assert!(def_inner.len() >= 2);

                let identifier = def_inner.next().unwrap();

                if variables.contains_key(identifier.as_str()) {
                    panic!(
                        "Error: {} contains duplicate variable names! ({} at line {} col {})",
                        file_path.to_string_lossy(),
                        identifier.as_str(),
                        identifier.line_col().0.to_string(),
                        identifier.line_col().1.to_string(),
                    );
                }

                match rule {
                    Rule::int_definition => {
                        let value = def_inner.next().unwrap();

                        variables.insert(
                            identifier.as_str().to_owned(),
                            Variable::Int(value.as_str().parse::<i32>().unwrap()),
                        );
                    }
                    Rule::enum_definition => {
                        let mut values = Vec::new();

                        for value in def_inner {
                            values.push(value.as_str().to_owned());
                        }
                        let current_value = values[0].clone();

                        variables.insert(
                            identifier.as_str().to_owned(),
                            Variable::Enum(values, current_value),
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
