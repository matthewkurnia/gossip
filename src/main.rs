use std::collections::{HashMap, HashSet};

mod characters;
mod dialogue;
mod reader;
mod variables;

fn main() {
    let mut files = reader::read_from_directory(".".to_owned());
    files.sort_by(|a, b| reader::compare_file_types(a.file_type, b.file_type));
    // for file in &files {
    //     println!("{}", file.file_path.to_string_lossy());
    // }

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

    // for character in &characters {
    //     println!("{}", character);
    // }
    // println!("{}", (characters.contains("Alicea")).to_string());

    let variables_file = files
        .pop()
        .expect("Are there any files in the project directory?");
    match variables_file.file_type {
        reader::FileType::Variables => {}
        _ => println!("{}", cv_warning_message),
    }
    let unparsed_variables = variables_file.contents;
    let mut variables = HashMap::new();
    variables::get_variables(&unparsed_variables, &mut variables);

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
            &mut localisation_map,
            &dialogue_file.file_path,
        );
        dialogues.push(dialogue);
    }
}
