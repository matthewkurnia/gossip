use std::{cmp::Ordering, fs, path::Path};
use walkdir::WalkDir;

#[derive(Clone, Copy)]
pub enum FileType {
    Characters = 3,
    Variables = 2,
    Dialogue = 1,
    Unidentified = 0,
}

pub struct GossipFile {
    pub file_path: Box<Path>,
    pub file_type: FileType,
    pub contents: String,
}

fn get_file_type(file_name: &Path) -> FileType {
    match file_name.extension() {
        Some(extension) => {
            if extension == "characters" {
                return FileType::Characters;
            }
            if extension == "variables" {
                return FileType::Variables;
            }
            if extension == "dialogue" {
                return FileType::Dialogue;
            }
            return FileType::Unidentified;
        }
        None => return FileType::Unidentified,
    }
}

pub fn read_from_directory(path: String) -> Vec<GossipFile> {
    return WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            if e.metadata().unwrap().is_file() {
                let file_type = get_file_type(e.path());
                match file_type {
                    FileType::Unidentified => None,
                    _ => Some(GossipFile {
                        file_path: Box::from(e.path()),
                        file_type,
                        contents: fs::read_to_string(e.path()).expect(&format!(
                            "WARNING! File {} cannot be read. Ignoring.",
                            e.path().display()
                        )),
                    }),
                }
            } else {
                None
            }
        })
        .collect();
}

pub fn compare_file_types(a: FileType, b: FileType) -> Ordering {
    return (a as u8).cmp(&(b as u8));
}
