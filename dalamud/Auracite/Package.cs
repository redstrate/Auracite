using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;

namespace Auracite;

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class InventoryItem
{
    public int slot;
    public uint quantity;
    public uint id;
    public ulong crafter_content_id;
    public byte item_flags;
    public ushort condition;
    public ushort spiritbond_or_collectability;
    public uint glamour_id;
    public List<ushort> materia;
    public List<byte> materia_grades;
    public List<byte> stains;
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class InventoryContainer
{
    public List<InventoryItem> items;
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class NameValue
{
    public string name;
    public uint value;
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class DayMonthValue
{
    // TODO: add back datetime name once I can figure out how to calculate it
    public int day;
    public int month;
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class ClassJobLevel
{
    public string name;
    public int level;
    public int exp;
    public uint value;
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class Appearance
{
    public int model_type;
    public int height;
    public int face_type;
    public int hair_style;
    public bool has_highlights;
    public int skin_color;
    public int eye_color;
    public int hair_color;
    public int hair_color2;
    public int face_features;
    public int face_features_color;
    public int eyebrows;
    public int eye_color2;
    public int eye_shape;
    public int nose_shape;
    public int jaw_shape;
    public int lip_style;
    public int lip_color;
    public int race_feature_size;
    public int race_feature_type;
    public int bust_size;
    public int facepaint;
    public int facepaint_color;
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class CharacterJson
{
    public string name;
    public NameValue world;
    public NameValue data_center;
    public NameValue city_state;
    public DayMonthValue nameday;
    public NameValue guardian;
    public NameValue gender;
    public NameValue tribe;
    public NameValue race;
    public List<ClassJobLevel> classjob_levels = new List<ClassJobLevel>();
    public NameValue grand_company;
    public int grand_company_rank; // TODO: introduce as a NameValue
    public NameValue title;
    public string playtime;
    public int voice;

    // adventurer plate
    public string? plate_title;
    public bool? plate_title_is_prefix;
    public string? plate_class_job;
    public int plate_class_job_level;
    public string? search_comment;

    public bool is_battle_mentor;
    public bool is_trade_mentor;
    public bool is_novice;
    public bool is_returner;
    public short player_commendations;

    // Appearance
    public Appearance appearance = new Appearance();

    // inventory
    public InventoryContainer inventory1;
    public InventoryContainer inventory2;
    public InventoryContainer inventory3;
    public InventoryContainer inventory4;

    public InventoryContainer equipped;

    public InventoryContainer currency;

    public InventoryContainer armory_off_hand;
    public InventoryContainer armory_head;
    public InventoryContainer armory_body;
    public InventoryContainer armory_hands;
    public InventoryContainer armory_waist;
    public InventoryContainer armory_legs;
    public InventoryContainer armory_ear;
    public InventoryContainer armory_neck;
    public InventoryContainer armory_wrist;
    public InventoryContainer armory_rings;
    public InventoryContainer armory_soul_crystal;
    public InventoryContainer armory_main_hand;

    // Other stuff useful to Kawari:

    // unlocks
    public List<byte> unlocks;
    public List<byte> seen_active_help;
    public List<byte> minions;
    public List<byte> mounts;
    public List<byte> orchestrion_rolls;
    public List<byte> cutscene_seen;
    public List<byte> ornaments;
    public List<byte> caught_fish;
    public List<byte> caught_spearfish;
    public List<byte> adventures;
    public List<byte> triple_triad_cards;
    public List<byte> glasses_styles;
    public List<byte> chocobo_taxi_stands;
    public List<byte> titles;
    public List<byte> unlocked_companion_equip;

    // aether currents
    public List<byte> comp_flg_set;
    public List<byte> unlocked_aether_currents;

    // aetheryte
    public List<byte> unlocked_aetherytes;
    public int hoempoint;
    public List<ushort> favorite_aetherytes;
    public int free_aetheryte;

    // classjob
    public int current_class;
    public int first_class;
    public uint rested_exp;

    // content
    public List<byte> unlocked_special_content;
    public List<byte> unlocked_raids;
    public List<byte> unlocked_dungeons;
    public List<byte> unlocked_guildhests;
    public List<byte> unlocked_trials;
    public List<byte> unlocked_crystalline_conflicts;
    public List<byte> unlocked_frontlines;
    public List<byte> cleared_raids;
    public List<byte> cleared_dungeons;
    public List<byte> cleared_guildhests;
    public List<byte> cleared_trials;
    public List<byte> cleared_crystalline_conflicts;
    public List<byte> cleared_frontlines;
    public List<byte> cleared_masked_carnivale;
    public List<byte> unlocked_misc_content;
    public List<byte> cleared_misc_content;

    // quest
    public List<byte> completed_quests;

    // volatile
    public float position_x;
    public float position_y;
    public float position_z;
    public float rotation;
    public ushort zone_id;
}
