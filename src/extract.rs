use std::fs;
const CREATURES_DIR: &str = "/Maine/Content/Blueprints/Creatures";

pub fn extract_creature_damage_modifiers(path_to_maine: &str) {
    println!(
        "Extracting creature damage modifiers from {}",
        format!("{}{}", path_to_maine, CREATURES_DIR)
    );

    let mut files: Vec<String> = Vec::new();
    resolve_path(path_to_maine, "", &mut files);
    for entry in files {
        if fs::read_to_string(format!("{}{}{}", path_to_maine, CREATURES_DIR, &entry))
            .unwrap()
            .contains("DamageResist")
        {
            println!("{:?}", entry);
        }
    }
}

fn resolve_path(path_to_maine: &str, relative_path: &str, files: &mut Vec<String>) {
    let path = format!("{}{}{}", path_to_maine, CREATURES_DIR, relative_path);
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let entries = fs::read_dir(&path).unwrap();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            resolve_path(
                                path_to_maine,
                                &format!(
                                    "{}/{}",
                                    relative_path,
                                    entry.file_name().to_str().unwrap()
                                ),
                                files,
                            );
                        }
                        Err(_) => {}
                    }
                }
            } else {
                files.push(String::from(relative_path));
            }
        }
        Err(_) => {}
    }
}
