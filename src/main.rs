const PATH_TO_MAINE: &str = "/Applications/Output/Exports";

mod extract;

fn main() {
    println!("Hello, world!");
    let creatures = extract::extract_creature_damage_modifiers(&format!(
        "{}{}",
        directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap(),
        PATH_TO_MAINE,
    ));
}
