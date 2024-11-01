use crate::data::CharacterData;
use minijinja::{context, Environment};

/// Writes a visual HTML for `char_data` to `file_path`.
/// This vaguely represents Lodestone and designed to visually check your character data.
pub fn create_html(char_data: &CharacterData) -> String {
    let mut env = Environment::new();
    env.add_template(
        "character.html",
        include_str!("../templates/character.html"),
    )
    .unwrap();
    let template = env.get_template("character.html").unwrap();
    template
        .render(context! {
            name => char_data.name,
            world => char_data.world,
            data_center => char_data.data_center,
            race => char_data.appearance.race,
            subrace => char_data.appearance.tribe,
            gender => char_data.appearance.gender,
            nameday => char_data.nameday,
            city_state => char_data.city_state
        })
        .unwrap()
}
