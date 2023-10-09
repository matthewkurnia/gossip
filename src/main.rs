use std::collections::HashSet;

mod characters_parser;
mod reader;
mod variables_parser;

fn main() {
    let mut files = reader::read_from_directory(".".to_owned());
    files.sort_by(|a, b| reader::compare_file_types(a.file_type, b.file_type));
    for file in &files {
        println!("{}", file.file_path.to_string_lossy());
    }

    let unparsed_characters = files.pop().expect("No .character file found.").contents;
    let mut characters = HashSet::new();
    characters_parser::get_characters(&unparsed_characters, &mut characters);

    for character in &characters {
        println!("{}", character);
    }
    println!("{}", (characters.contains("Alicea")).to_string());
}
