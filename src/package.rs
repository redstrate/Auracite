use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct InventoryItem {
    pub slot: i32,
    pub quantity: u32,
    pub condition: i32,
    pub id: u32,
    pub glamour_id: u32,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct InventoryContainer {
    pub items: Vec<InventoryItem>,
}

#[derive(Default, Deserialize, Clone)]
pub struct Package {
    pub playtime: String,
    pub is_battle_mentor: bool,
    pub is_trade_mentor: bool,
    pub is_novice: bool,
    pub is_returner: bool,
    pub player_commendations: i32,
    pub portrait: String,
    pub plate_title: String,
    pub plate_title_is_prefix: bool,
    pub plate_class_job: String,
    pub plate_class_job_level: i32,
    pub search_comment: String,
    pub base_plate: Option<String>,
    pub pattern_overlay: Option<String>,
    pub backing: Option<String>,
    pub top_border: Option<String>,
    pub bottom_border: Option<String>,
    pub portrait_frame: Option<String>,
    pub plate_frame: Option<String>,
    pub accent: Option<String>,
    pub voice: i32,

    // Appearance
    pub race: i32,
    pub gender: i32,
    pub model_type: i32,
    pub height: i32,
    pub tribe: i32,
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

    // inventory
    pub inventory1: InventoryContainer,
    pub inventory2: InventoryContainer,
    pub inventory3: InventoryContainer,
    pub inventory4: InventoryContainer,

    pub equipped_items: InventoryContainer,

    pub currency: InventoryContainer,

    pub armory_off_hand: InventoryContainer,
    pub armory_head: InventoryContainer,
    pub armory_body: InventoryContainer,
    pub armory_hands: InventoryContainer,
    pub armory_waist: InventoryContainer,
    pub armory_legs: InventoryContainer,
    pub armory_ear: InventoryContainer,
    pub armory_neck: InventoryContainer,
    pub armory_wrist: InventoryContainer,
    pub armory_rings: InventoryContainer,
    pub armory_soul_crystal: InventoryContainer,
    pub armory_main_hand: InventoryContainer,

    pub unlock_flags: Vec<u8>,
    pub unlock_aetherytes: Vec<u8>,
}
