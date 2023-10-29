use csv::Writer;
use heck::ToSnakeCase;
use std::{collections::HashMap, fs};

pub fn initialise_directory() {
    match fs::remove_dir_all("./gossip_generated") {
        Ok(_) => {}
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => {
                panic!("Failed flushing gossip_generated directory.")
            }
        },
    }
    fs::create_dir("./gossip_generated").expect("Failed creating gossip_generated directory.");

    println!("Generated directory ./gossip_generated");
}

pub fn write_localisation_csv(localisation_map: HashMap<String, String>) {
    let mut writer = Writer::from_path("./gossip_generated/translation.csv")
        .expect(" Path \'./gossip_generated/translation.csv\' is invalid somehow.");
    writer.serialize(("key", "en")).unwrap();

    // Sort localisation entries to make them nicer to edit.
    let mut entries: Vec<(&String, &String)> = localisation_map.iter().collect();
    entries.sort_by(|(a, _), (b, _)| human_sort::compare(a, b));

    for entry in entries {
        writer.serialize(&entry).expect(&format!(
            "Failed serializing entry ({}, {}).",
            entry.0, entry.1
        ));
    }
    writer.flush().expect("CSV writer flush error.");

    println!("Generated translation.csv");
}

pub fn write_gdscript(class_name: String, contents: String) {
    let file_name = class_name.to_snake_case() + ".gd";
    fs::write("./gossip_generated/".to_owned() + &file_name, contents)
        .expect(&format!("Unable to write {}", file_name));

    println!("Generated {}", file_name);
}
