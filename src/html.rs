use crate::data::CharacterData;
use minijinja::{Environment, context};

/// Writes a visual HTML for `char_data` to `file_path`.
/// This vaguely represents Lodestone and designed to visually check your character data.
pub fn create_character_html(char_data: &CharacterData) -> String {
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
            world => char_data.world.name,
            data_center => char_data.data_center,
            race => char_data.race.name,
            subrace => char_data.tribe.name,
            gender => char_data.gender.name,
            nameday => char_data.nameday.value,
            city_state => char_data.city_state.name
        })
        .unwrap()
}

/// Writes a visual HTML for `char_data` to `file_path`.
/// This vaguely represents Lodestone and designed to visually check your character data.
pub fn create_plate_html(char_data: &CharacterData) -> String {
    let mut env = Environment::new();
    env.add_template("plate.html", include_str!("../templates/plate.html"))
        .unwrap();
    let template = env.get_template("plate.html").unwrap();
    template
        .render(context! {
            name => char_data.name,
            world => char_data.world.name,
            data_center => char_data.data_center,
            title => char_data.plate_title,
            level => char_data.plate_classjob_level,
            class => char_data.plate_classjob,
            search_comment => char_data.search_comment,
        })
        .unwrap()
}
