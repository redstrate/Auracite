use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Currencies {
    pub gil: u32,
}

#[derive(Default, Serialize)]
pub struct Appearance {
    pub race: String,
    pub subrace: String,
    pub gender: String,
    pub height: i32,
    pub bust_size: i32
}

#[derive(Default, Serialize)]
pub struct CharacterData {
    pub name: String,
    pub world: String,
    pub data_center: String,
    pub city_state: String,
    pub nameday: String,
    pub guardian: String,
    pub currencies: Currencies,
    pub playtime: String,
    pub appearance: Appearance,
    pub is_battle_mentor: bool,
    pub is_trade_mentor: bool,
    pub is_novice: bool,
    pub is_returner: bool,
    pub player_commendations: i32,

    #[serde(skip)]
    pub face_url: String,
    #[serde(skip)]
    pub portrait_url: String,
}
