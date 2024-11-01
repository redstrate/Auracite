use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Currencies {
    pub gil: u32,
}

#[derive(Default, Serialize)]
pub struct Appearance {
    pub race: String,
    pub gender: String,
    pub model_type: i32,
    pub height: i32,
    pub tribe: String,
    pub face_type: i32,
    pub hair_style: i32,
    pub has_highlights: bool,
    pub skin_color: i32,
    pub eye_color: i32,
    pub hair_color: i32,
    pub hair_color2: i32,
    pub face_features: i32,
    pub face_features_color: i32,
    pub eyebrows: i32,
    pub eye_color2: i32,
    pub eye_shape: i32,
    pub nose_shape: i32,
    pub jaw_shape: i32,
    pub lip_style: i32,
    pub lip_color: i32,
    pub race_feature_size: i32,
    pub race_feature_type: i32,
    pub bust_size: i32,
    pub facepaint: i32,
    pub facepaint_color: i32,
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
