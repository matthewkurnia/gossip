use std::collections::{HashMap, HashSet};

mod characters;
mod reader;
mod variables;

fn main() {
    let mut files = reader::read_from_directory(".".to_owned());
    files.sort_by(|a, b| reader::compare_file_types(a.file_type, b.file_type));
    // for file in &files {
    //     println!("{}", file.file_path.to_string_lossy());
    // }

    let characters_file = files
        .pop()
        .expect("Are there any files in the project directory?");
    match characters_file.file_type {
        reader::FileType::Characters => {}
        _ => println!("Warning: No .character file found."),
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
        _ => println!("Warning: No .variable file found."),
    }
    let unparsed_variables = variables_file.contents;
    let mut variables = HashMap::new();
    variables::get_variables(&unparsed_variables, &mut variables);
}
