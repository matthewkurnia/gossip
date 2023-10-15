use std::{
    collections::{HashMap, HashSet},
    fs,
};

use heck::ToPascalCase;

mod characters;
mod code_generator;
mod dialogue;
mod reader;
mod variables;
mod writer;

fn main() {
    let mut files = reader::read_from_directory(".".to_owned());
    files.sort_by(|a, b| reader::compare_file_types(a.file_type, b.file_type));

    let cv_warning_message =
        "Warning: Make sure there are only one characters and one variables file.";

    let characters_file = files
        .pop()
        .expect("Are there any files in the project directory?");
    match characters_file.file_type {
        reader::FileType::Characters => {}
        _ => println!("{}", cv_warning_message),
    }
    let unparsed_characters = characters_file.contents;
    let mut characters = HashSet::new();
    characters::get_characters(&unparsed_characters, &mut characters);

    let variables_file = files
        .pop()
        .expect("Are there any files in the project directory?");
    match variables_file.file_type {
        reader::FileType::Variables => {}
        _ => println!("{}", cv_warning_message),
    }
    let unparsed_variables = variables_file.contents;
    let mut variables = HashMap::new();
    variables::get_variables(
        &unparsed_variables,
        &mut variables,
        &variables_file.file_path,
    );

    if files.is_empty() {
        println!("Warning: No .dialogue files found.");
        return;
    }

    match files.last().unwrap().file_type {
        reader::FileType::Dialogue => {}
        _ => println!("{}", cv_warning_message),
    }

    let mut dialogues = Vec::new();
    let mut localisation_map = HashMap::new();
    for dialogue_file in files {
        let unparsed_dialogue = dialogue_file.contents;
        let mut dialogue = HashMap::new();
        dialogue::get_dialogue(
            &unparsed_dialogue,
            &mut dialogue,
            &characters,
            &variables,
            &mut localisation_map,
            &dialogue_file.file_path,
        );
        let class_name = dialogue_file
            .file_path
            .with_extension("")
            .to_string_lossy()
            .to_pascal_case();
        dialogues.push((class_name, dialogue));
    }

    println!("{}", code_generator::generate_gossip_code());
    println!("{}", code_generator::generate_variables_code(&variables));
    for (class_name, dialogue) in dialogues {
        println!(
            "{}",
            code_generator::generate_dialogue_code(class_name, &dialogue)
        );
    }

    // fs::remove_dir_all("./gossip_generated").expect("Failed flushing gossip_generated directory.");
    // fs::create_dir("./gossip_generated").expect("Failed creating gossip_generated directory.");
}
