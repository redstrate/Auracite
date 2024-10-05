use crate::data::CharacterData;
use minijinja::{context, Environment};
use std::fs::write;
use std::io;

/// Writes a visual HTML for `char_data` to `file_path`.
/// This vaguely represents Lodestone and designed to visually check your character data.
pub fn write_html(char_data: &CharacterData, file_path: &str) -> io::Result<()> {
    let mut env = Environment::new();
    env.add_template(
        "character.html",
        include_str!("../templates/character.html"),
    )
    .unwrap();
    let template = env.get_template("character.html").unwrap();
    let character_html = template
        .render(context! {
            name => char_data.name,
            world => char_data.world,
            data_center => char_data.data_center,
            race => char_data.appearance.race,
            subrace => char_data.appearance.subrace,
            gender => char_data.appearance.gender,
            nameday => char_data.nameday,
            city_state => char_data.city_state
        })
        .unwrap();

    write(file_path, &character_html)
}
