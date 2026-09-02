use serde_json::Value;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

const CREATURES_DIR: &str = "/Maine/Content/Blueprints/Creatures";

pub enum DamageAugment {
    Fresh,
    Salty,
    Spicy,
    Sour,
}

pub enum DamageType {
    Generic,
    Stabbing,
    Slashing,
    Chopping,
    Busting,
}

pub enum ModifierDirection {
    Resistance(ModifierStrength),
    Weakness(ModifierStrength),
    Undirected,
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, EnumIter)]
pub enum ModifierStrength {
    Tiny,
    Small,
    Medium,
    Large,
    VeryLarge,
    Base,
    Undisclosed,
    IBM,
}

pub enum DamageModifier {
    DamageAugment(DamageAugment, ModifierDirection),
    DamageType(DamageType, ModifierDirection),
    All(ModifierDirection),
    Other(String),
}

pub struct Creature {
    name: String,
    receiving_damage_modifiers: Vec<DamageModifier>,
}

pub fn extract_creature_damage_modifiers(path_to_maine: &str) -> Vec<Creature> {
    println!(
        "Extracting creature damage modifiers from {}",
        format!("{}{}", path_to_maine, CREATURES_DIR)
    );

    let mut creatures: Vec<Creature> = Vec::new();

    let mut files: Vec<String> = Vec::new();
    resolve_path(path_to_maine, "", &mut files);
    for entry in files {
        if let Ok(file_content) =
            fs::read_to_string(format!("{}{}{}", path_to_maine, CREATURES_DIR, &entry))
        {
            if file_content.contains("DamageResist") {
                //println!("{:?}", entry);
                let mut creature = Creature {
                    name: entry,
                    receiving_damage_modifiers: Vec::new(),
                };
                for modifier in read_creature_damage_modifiers(&file_content).unwrap() {
                    //println!("{}", modifier);
                    if modifier.starts_with("DamageResist") {
                        resolve_damage_modifier(&modifier);
                    }
                }
            }
        }
    }

    creatures
}

fn read_creature_damage_modifiers(file_content: &str) -> serde_json::Result<Vec<String>> {
    let creature: Value = serde_json::from_str(file_content)?;

    let mut modifiers: Vec<String> = Vec::new();

    if let Value::Array(array) = creature {
        for object in array {
            if object["Type"] == "StatusEffectComponent" {
                if let Value::Array(modifier_rows) = &object["Properties"]["DefaultStatusEffects"] {
                    for modifier_row in modifier_rows {
                        if let Value::String(modifier) = &modifier_row["RowName"] {
                            modifiers.push(String::from(modifier));
                        }
                    }
                }
            }
        }
    }

    Ok(modifiers)
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

pub fn resolve_damage_modifier(modifier_string: &str) {
    if !modifier_string.starts_with("DamageResist") {
        panic!("modifier_string doesn't start with DamageResist")
    }

    let mut modifier_string = String::from(&modifier_string[12..]);
    let mut weakness = false;
    let mut strength = ModifierStrength::Undisclosed;
    if modifier_string.contains("Down") {
        weakness = true;
        let weakness_identifier_position = modifier_string.find("Down").unwrap();
        modifier_string = format!(
            "{}{}",
            &modifier_string[..weakness_identifier_position],
            &modifier_string[weakness_identifier_position + "Down".len()..],
        );
    }

    for entry in ModifierStrength::iter() {
        if modifier_string.ends_with((&entry).to_string().as_str()) {
            strength = entry;
            modifier_string = String::from(
                &modifier_string[..modifier_string.len() - (&strength).to_string().len()],
            );
            break;
        }
    }

    println!(
        "{} {} {}",
        strength.to_string().as_str(),
        modifier_string,
        if weakness { "Weakness" } else { "Resistance" },
    )
}

pub fn damage_types(all_modifiers: &str) -> Vec<String> {
    let mut damage_types: Vec<String> = Vec::new();
    for line in all_modifiers.split('\n') {
        if !damage_types.iter().any(|e| e == line) {
            damage_types.push(String::from(line));
        }
    }
    damage_types
}
