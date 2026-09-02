use serde_json::Value;
use std::{fs, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

const CREATURES_DIR: &str = "/Maine/Content/Blueprints/Creatures";

#[derive(Debug, Display, EnumString, PartialEq, Eq, EnumIter)]
pub enum DamageAugment {
    Fresh,
    Salty,
    Spicy,
    Sour,
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, EnumIter)]
pub enum DamageType {
    General,
    Stabbing,
    Slashing,
    Chopping,
    Smashing,
}

#[derive(Debug, Display)]
pub enum ModifierDirection {
    Resistance(ModifierStrength),
    Weakness(ModifierStrength),
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, EnumIter)]
pub enum ModifierStrength {
    Tiny,
    Small,
    Medium,
    VeryLarge,
    Large,
    Base,
    Undisclosed,
    IBM,
}

#[derive(Debug, Display)]
pub enum DamageModifier {
    DamageAugment(DamageAugment, ModifierDirection),
    DamageType(DamageType, ModifierDirection),
    All(ModifierDirection),
    Other(String, ModifierDirection),
}

pub struct Creature {
    pub name: String,
    pub receiving_damage_modifiers: Vec<DamageModifier>,
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
                    creature
                        .receiving_damage_modifiers
                        .push(resolve_damage_modifier(&modifier));
                }
                creatures.push(creature);
            }
        }
    }

    creatures
}

fn read_creature_damage_modifiers(file_content: &str) -> serde_json::Result<Vec<String>> {
    let creature: Value = serde_json::from_str(file_content)?;

    let mut modifiers: Vec<String> = Vec::new();

    if let Value::Array(array) = creature {
        for mut object in array {
            if object["Type"] == "StatusEffectComponent" {
                if let Value::Array(modifier_rows) =
                    object["Properties"]["DefaultStatusEffects"].take()
                {
                    for mut modifier_row in modifier_rows {
                        if let Value::String(modifier) = modifier_row["RowName"].take() {
                            if modifier.starts_with("DamageResist") {
                                modifiers.push(modifier);
                            }
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

pub fn resolve_damage_modifier(modifier_string: &str) -> DamageModifier {
    if !modifier_string.starts_with("DamageResist") {
        panic!("modifier_string doesn't start with DamageResist")
    }

    let modifier_string = String::from(&modifier_string[12..]);
    let (weakness, modifier_string) = {
        if modifier_string.contains("Down") {
            let weakness_identifier_position = modifier_string.find("Down").unwrap();
            let modifier_string = format!(
                "{}{}",
                &modifier_string[..weakness_identifier_position],
                &modifier_string[weakness_identifier_position + "Down".len()..],
            );
            (true, modifier_string)
        } else {
            (false, modifier_string)
        }
    };

    let (strength, modifier_string) = ModifierStrength::iter()
        .find(|entry| modifier_string.ends_with(entry.to_string().as_str()))
        .map(|entry| {
            let modifier_string =
                String::from(&modifier_string[..modifier_string.len() - entry.to_string().len()]);
            (entry, modifier_string)
        })
        .unwrap_or((ModifierStrength::Undisclosed, modifier_string));

    let direction = {
        if weakness {
            ModifierDirection::Weakness(strength)
        } else {
            ModifierDirection::Resistance(strength)
        }
    };

    if modifier_string == "All" {
        DamageModifier::All(direction)
    } else {
        if let Ok(damage_type) = DamageType::from_str(&modifier_string) {
            DamageModifier::DamageType(damage_type, direction)
        } else if let Ok(damage_augment) = DamageAugment::from_str(&modifier_string) {
            DamageModifier::DamageAugment(damage_augment, direction)
        } else {
            DamageModifier::Other(modifier_string, direction)
        }
    }
}
