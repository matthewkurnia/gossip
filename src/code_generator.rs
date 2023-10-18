use crate::{dialogue::Line, variables::Variable};
use heck::{ToPascalCase, ToShoutySnakeCase};
use std::collections::HashMap;

pub fn generate_gossip_code() -> String {
    return "\
        extends Node\n\
        \n\
        signal event(event_identifier: String)\n\
        \n\
        enum LineType {\n\
        \tREGULAR,\n\
        \tCHOICE,\n\
        \tFUNC,\n\
        }\n"
    .to_owned();
}

pub fn generate_variables_code(variables: &HashMap<String, Variable>) -> String {
    let mut variables_code = "extends Node\n\n\n".to_owned();
    for (identifier, variable) in variables {
        match variable {
            Variable::Int(initial_value) => {
                variables_code += &format!("{} := {}\n\n\n", identifier, initial_value.to_string());
            }
            Variable::Enum(possible_values, initial_value) => {
                let enum_name = identifier.to_pascal_case();
                variables_code += &format!("enum {} = {{\n", enum_name);
                for possible_value in possible_values {
                    variables_code += &format!("\t{},\n", possible_value.to_shouty_snake_case());
                }
                variables_code += "}\n\n";
                variables_code += &format!(
                    "{} := {}.{}\n\n\n",
                    identifier,
                    enum_name,
                    initial_value.to_shouty_snake_case()
                );
            }
        }
    }
    return variables_code;
}

pub fn generate_dialogue_code(class_name: String, dialogue: &HashMap<String, Vec<Line>>) -> String {
    let mut dialogue_code = format!("class_name {}\n\n\n", class_name);
    dialogue_code += "var dialogue := {\n";
    for (fragment_title, lines) in dialogue {
        dialogue_code += &format!("\t\"{}\": [\n", fragment_title);
        for line in lines {
            dialogue_code += "\t\t{\n";
            match line {
                Line::Regular(character_identifier, localisation_key) => {
                    dialogue_code += "\t\t\t\"type\": Gossip.LineType.REGULAR,\n";
                    dialogue_code += &format!("\t\t\t\"speaker\": \"{}\",\n", character_identifier);
                    dialogue_code += &format!("\t\t\t\"contents\": \"{}\",\n", localisation_key);
                }
                Line::Choice(character_identifier, choices) => {
                    dialogue_code += "\t\t\t\"type\": Gossip.LineType.CHOICE,\n";
                    dialogue_code += &format!("\t\t\t\"speaker\": \"{}\",\n", character_identifier);
                    dialogue_code += "\t\t\t\"choices\": [\n";
                    for (localisation_key, fragment_title) in choices {
                        dialogue_code += "\t\t\t\t{\n";
                        dialogue_code +=
                            &format!("\t\t\t\t\t\"contents\": \"{}\",\n", localisation_key);
                        dialogue_code +=
                            &format!("\t\t\t\t\t\"target\": \"{}\",\n", fragment_title);
                        dialogue_code += "\t\t\t\t},\n";
                    }
                    dialogue_code += "\t\t\t],\n";
                }
                Line::Func(func) => {
                    dialogue_code += "\t\t\t\"type\": Gossip.LineType.FUNC,\n";
                    dialogue_code += &format!("\t\t\t\"func\": {},\n", func);
                }
            }
            dialogue_code += "\t\t},\n";
        }
        dialogue_code += "\t],\n";
    }
    dialogue_code += "}\n";
    return dialogue_code;
}
