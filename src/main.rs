use walkdir::WalkDir;

mod lexer;
mod reader;

fn main() {
    let files = reader::read_from_directory(".".to_owned());
    let tokenised_files = lexer::get_tokenised_files(files);
    for t_file in tokenised_files {
        for token in t_file.tokens {
            println!("{}", token.slice);
        }
    }
}
