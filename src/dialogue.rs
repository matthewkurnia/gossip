use crate::variables::Variable;
use heck::{ToPascalCase, ToShoutySnakeCase};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

#[derive(Parser)]
#[grammar = "grammars/dialogue.pest"]
struct DialogueParser;

pub enum Line {
    Regular(String, String),
    Choice(String, Vec<(String, String)>),
    Func(String),
}

fn decorate(text: String, rule: Rule) -> String {
    return match rule {
        Rule::bold_text => format!("[b]{}[/b]", text),
        Rule::italic_text => format!("[i]{}[/i]", text),
        Rule::wave_text => format!("[wave amp=50.0 freq=5.0 connected=1]{{{}}}[/wave]", text),
        Rule::shake_text => format!("[shake rate=20.0 level=5 connected=1]{{{}}}[/shake]", text),
        _ => text,
    };
}

fn get_bbcode_text(styled_text: Pair<'_, Rule>) -> String {
    let mut bbcode_text = "".to_owned();
    for text in styled_text.into_inner() {
        bbcode_text += &decorate(text.as_str().to_owned(), text.as_rule());
    }
    return bbcode_text;
}

fn get_localisation_key(file_path: &Path, fragment_title: &str, line_identifier: String) -> String {
    let mut localisation_key = "".to_owned();

    localisation_key += &file_path.to_string_lossy();
    localisation_key += " ";
    localisation_key += fragment_title;
    localisation_key += " ";
    localisation_key += &line_identifier;

    return localisation_key;
}

fn check_is_character_valid(character_identifier: &Pair<'_, Rule>, characters: &HashSet<String>) {
    if characters.contains(character_identifier.as_str()) {
        return;
    }
    panic!(
        "Error: \"{}\" is not a registered character! (line {} col {})",
        character_identifier.as_str(),
        character_identifier.line_col().0.to_string(),
        character_identifier.line_col().1.to_string(),
    );
}
pub fn get_dialogue(
    contents: &String,
    dialogue: &mut HashMap<String, Vec<Line>>,
    characters: &HashSet<String>,
    variables: &HashMap<String, Variable>,
    localisation_map: &mut HashMap<String, String>,
    file_path: &Path,
) {
    let fragments = DialogueParser::parse(Rule::fragments, &contents)
        .expect("Unsuccessful parse of dialogue file.")
        .next()
        .unwrap();

    let mut fragment_titles = HashSet::new();
    for fragment in fragments.clone().into_inner() {
        match fragment.as_rule() {
            Rule::fragment => {
                let mut fragment = fragment.into_inner();
                assert!(fragment.len() > 1);

                let fragment_title = fragment.next().unwrap();
                if fragment_titles.contains(fragment_title.as_str()) {
                    panic!(
                        "Error: {} contains duplicate dialogue fragment titles! (\"{}\" at line {} col {})",
                        file_path.to_string_lossy(),
                        fragment_title.as_str(),
                        fragment_title.line_col().0.to_string(),
                        fragment_title.line_col().1.to_string(),
                    )
                }

                fragment_titles.insert(fragment_title.as_str());
            }
            _ => {}
        }
    }

    for fragment in fragments.into_inner() {
        match fragment.as_rule() {
            Rule::fragment => {
                let mut fragment = fragment.into_inner();
                assert!(fragment.len() > 1);

                let fragment_title = fragment.next().unwrap();

                let mut line_counter = 0;
                let mut lines = Vec::new();
                for line in fragment {
                    let line = line.into_inner().last().unwrap();
                    match line.as_rule() {
                        Rule::choice_line => {
                            let mut choice_line = line.into_inner();
                            assert!(choice_line.len() >= 2);

                            let character_identifier = choice_line.next().unwrap();
                            check_is_character_valid(&character_identifier, characters);

                            let mut choice_counter = 0;
                            let mut choices = Vec::new();
                            for choice in choice_line {
                                let mut choice = choice.into_inner();
                                assert!(choice.len() == 2);

                                let styled_text = choice.next().unwrap();
                                let fragment_title = choice.next().unwrap();

                                if !fragment_titles.contains(fragment_title.as_str()) {
                                    panic!(
                                        "Error: {} contains a transition to a non-existent fragment! (\"{}\" at line {} col {})",
                                        file_path.to_string_lossy(),
                                        fragment_title.as_str(),
                                        fragment_title.line_col().0.to_string(),
                                        fragment_title.line_col().1.to_string(),
                                    );
                                }

                                let localisation_key = get_localisation_key(
                                    file_path,
                                    fragment_title.as_str(),
                                    line_counter.to_string() + " " + &choice_counter.to_string(),
                                );
                                let entry = localisation_key.clone();

                                localisation_map
                                    .insert(localisation_key, get_bbcode_text(styled_text));
                                choices.push((entry, fragment_title.as_str().to_owned()));

                                choice_counter += 1;
                            }
                            lines.push(Line::Choice(
                                character_identifier.as_str().to_owned(),
                                choices,
                            ));
                        }
                        Rule::regular_line => {
                            let mut regular_line = line.into_inner();
                            assert!(regular_line.len() == 2);

                            let character_identifier = regular_line.next().unwrap();
                            check_is_character_valid(&character_identifier, characters);

                            let styled_text = regular_line.next().unwrap();

                            let localisation_key = get_localisation_key(
                                file_path,
                                fragment_title.as_str(),
                                line_counter.to_string(),
                            );
                            let entry = localisation_key.clone();

                            localisation_map.insert(localisation_key, get_bbcode_text(styled_text));
                            lines.push(Line::Regular(
                                character_identifier.as_str().to_owned(),
                                entry,
                            ));
                        }
                        Rule::set_line => {
                            let set_line = line.into_inner();
                            assert!(set_line.len() == 1);

                            let set_line = set_line.last().unwrap();
                            let line_type = set_line.as_rule();
                            let operator = match line_type {
                                Rule::inc_int => "+=",
                                Rule::dec_int => "-=",
                                Rule::set_enum => "=",
                                Rule::set_int | _ => "=",
                            };

                            let mut set_line = set_line.into_inner();
                            assert!(set_line.len() == 2);
                            let variable_identifier = set_line.next().unwrap();
                            let value = set_line.next().unwrap();

                            let mut invalid_type = false;
                            match variables.get(variable_identifier.as_str()) {
                                Some(Variable::Int(_)) => match line_type {
                                    Rule::set_enum => invalid_type = true,
                                    _ => {}
                                },
                                Some(Variable::Enum(possible_values, _)) => match line_type {
                                    Rule::set_int | Rule::inc_int | Rule::dec_int => {
                                        invalid_type = true
                                    }
                                    _ => {
                                        invalid_type = invalid_type
                                            || possible_values
                                                .contains(&value.as_str().to_shouty_snake_case());
                                    }
                                },
                                None => {
                                    panic!(
                                        "Error: {} refers to an undefined variable! (\"{}\" at line {} col {})",
                                        file_path.to_string_lossy(),
                                        variable_identifier.as_str(),
                                        variable_identifier.line_col().0.to_string(),
                                        variable_identifier.line_col().1.to_string(),
                                    );
                                }
                            }
                            if invalid_type {
                                panic!(
                                    "Error: {} contains a type mismatch! (\"{}\" at line {} col {})",
                                    file_path.to_string_lossy(),
                                    value.as_str(),
                                    value.line_col().0.to_string(),
                                    value.line_col().1.to_string(),
                                );
                            }

                            let variable_identifier = variable_identifier.as_str();
                            let value = match line_type {
                                Rule::set_enum => {
                                    format!(
                                        "GossipVariables.{}.",
                                        variable_identifier.to_pascal_case()
                                    ) + &value.as_str().to_shouty_snake_case()
                                }
                                _ => value.as_str().to_shouty_snake_case(),
                            };
                            lines.push(Line::Func(format!(
                                "func(): GossipVariables.{} {} {}",
                                variable_identifier, operator, value
                            )));
                        }
                        Rule::event_line => {
                            let event_line = line.into_inner();
                            assert!(event_line.len() == 1);

                            // TODO: This identifier should consider file name to be unique.
                            let event_identifier = event_line.last().unwrap();

                            lines.push(Line::Func(format!(
                                "func(): Gossip.event.emit(\"{}\")",
                                event_identifier.as_str()
                            )));
                        }
                        _ => {}
                    }
                    line_counter += 1;
                }

                dialogue.insert(fragment_title.as_str().to_owned(), lines);
            }
            _ => {}
        }
    }
}
