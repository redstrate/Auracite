use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Currencies {
    pub gil: i64,
}

#[derive(Default, Serialize)]
pub struct CharacterData {
    pub name: String,
    pub world: String,
    pub data_center: String,
    pub race: String,
    pub subrace: String,
    pub gender: String,
    pub city_state: String,
    pub nameday: String,
    pub guardian: String,
    pub currencies: Currencies,
    pub playtime: String,

    #[serde(skip)]
    pub face_url: String,
    #[serde(skip)]
    pub portrait_url: String,
}
