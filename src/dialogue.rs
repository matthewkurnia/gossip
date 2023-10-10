use std::collections::HashMap;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

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
        Rule::bold_text => format!("[b]{text}[/b]"),
        Rule::italic_text => format!("[i]{text}[/i]"),
        Rule::wave_text => format!("[wave amp=50.0 freq=5.0 connected=1]{{{text}}}[/wave]"),
        Rule::shake_text => format!("[shake rate=20.0 level=5 connected=1]{{{text}}}[/shake]"),
        _ => text,
    };
}

fn get_bbcode_text(styled_text: Pair<'_, Rule>) -> String {
    let mut bbcode_text = "".to_owned();
    for text in styled_text.into_inner() {
        bbcode_text.push_str(&decorate(text.as_str().to_owned(), text.as_rule()));
    }
    return bbcode_text;
}

pub fn get_dialogue(contents: &String, dialogue: &mut HashMap<String, Vec<Line>>) {
    let fragments = DialogueParser::parse(Rule::fragments, &contents)
        .expect("Unsuccessful parse of dialogue file.")
        .next()
        .unwrap();

    for fragment in fragments.into_inner() {
        match fragment.as_rule() {
            Rule::fragment => {
                let mut fragment = fragment.into_inner();
                assert!(fragment.len() > 1);

                let fragment_title = fragment.next().unwrap();

                let mut lines = Vec::new();
                for line in fragment {
                    match line.as_rule() {
                        Rule::choice_line => {
                            let mut choice_line = line.into_inner();
                            assert!(choice_line.len() >= 2);

                            let identifier = choice_line.next().unwrap();

                            let mut choices = Vec::new();
                            for choice in choice_line {
                                let mut choice = choice.into_inner();
                                assert!(choice.len() == 2);

                                let styled_text = choice.next().unwrap();
                                let fragment_identifier = choice.next().unwrap();

                                // TODO: This should point to a line identifier instead of using the text outright.
                                choices.push((
                                    get_bbcode_text(styled_text),
                                    fragment_identifier.as_str().to_owned(),
                                ));
                            }
                            lines.push(Line::Choice(identifier.as_str().to_owned(), choices));
                        }
                        Rule::regular_line => {
                            let mut regular_line = line.into_inner();
                            assert!(regular_line.len() == 2);

                            let identifier = regular_line.next().unwrap();
                            let styled_text = regular_line.next().unwrap();

                            // TODO: This should point to a line identifier instead of using the text outright.
                            lines.push(Line::Regular(
                                identifier.as_str().to_owned(),
                                get_bbcode_text(styled_text),
                            ));
                        }
                        Rule::set_line => {
                            let set_line = line.into_inner();
                            assert!(set_line.len() == 1);

                            let set_line = set_line.last().unwrap();
                            let operator = match set_line.as_rule() {
                                Rule::inc_int => "+=",
                                Rule::dec_int => "-=",
                                Rule::set_int | Rule::set_enum | _ => "=",
                            };

                            let mut set_line = set_line.into_inner();
                            assert!(set_line.len() == 2);
                            let variable_identifier = set_line.next().unwrap().as_str();
                            let value = set_line.next().unwrap().as_str();

                            lines.push(Line::Func(format!(
                                "func(): GossipVariables.{variable_identifier} {operator} {value}"
                            )));
                        }
                        Rule::event_line => {
                            let event_line = line.into_inner();
                            assert!(event_line.len() == 1);

                            // TODO: This identifier should consider file name to be unique.
                            let event_identifier = event_line.last().unwrap();

                            lines.push(Line::Func(format!(
                                "func(): GossipEvents.event.emit(\"{event_identifier}\")"
                            )));
                        }
                        _ => {}
                    }
                }

                dialogue.insert(fragment_title.as_str().to_owned(), lines);
            }
            _ => {}
        }
    }
}
