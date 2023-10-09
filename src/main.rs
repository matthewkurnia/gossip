use pest::Parser;
use walkdir::WalkDir;

mod characters_parser;
mod reader;

fn main() {
    let mut files = reader::read_from_directory(".".to_owned());
    files.sort_by(|a, b| reader::compare_file_types(a.file_type, b.file_type));
    for file in &files {
        println!("{}", file.file_path.to_string_lossy());
    }
    let test = &files.pop();
    characters_parser::get_tokens(&test.as_ref().unwrap().contents);
}
