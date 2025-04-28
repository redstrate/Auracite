use serde::Serialize;

use crate::value::{
    CityStateValue, ClassJobValue, GenderValue, GrandCompanyValue, GuardianValue, ItemValue,
    NamedayValue, RaceValue, TribeValue, WorldValue,
};

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
pub struct EquippedItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_hand: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_hand: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hands: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feet: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earrings: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub necklace: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracelets: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_ring: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_ring: Option<ItemValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soul_crystal: Option<ItemValue>,
}

#[derive(Default, Serialize)]
pub struct CharacterData {
    pub name: String,
    pub world: WorldValue,
    pub data_center: String,
    pub city_state: CityStateValue,
    pub nameday: NamedayValue,
    pub guardian: GuardianValue,
    pub race: RaceValue,
    pub gender: GenderValue,
    pub tribe: TribeValue,
    pub classjob_levels: Vec<ClassJobValue>,
    pub grand_company: GrandCompanyValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub equipped: EquippedItems,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Currencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<Appearance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_battle_mentor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trade_mentor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_novice: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_returner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_commendations: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate_classjob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate_classjob_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<i32>,

    #[serde(skip)]
    pub face_url: String,
    #[serde(skip)]
    pub portrait_url: String,
}
