using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;

namespace Auracite;

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class AdventurerPlate
{
    public string? title;
    public bool? title_is_prefix;
    public string? class_job;
    public int class_job_level;
    public string? search_comment;
    public bool invert_portrait_placement;
}

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
    public List<ushort> materia = new List<ushort>();
    public List<byte> materia_grades = new List<byte>();
    public List<byte> stains = new List<byte>();
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class InventoryContainer
{
    public List<InventoryItem> items = new List<InventoryItem>();
}

[SuppressMessage("ReSharper", "InconsistentNaming")]
public class NameValue
{
    public string? name;
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
    public string? name;
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
    public string? name;
    public NameValue? world;
    public NameValue? data_center;
    public NameValue? city_state;
    public DayMonthValue? nameday;
    public NameValue? guardian;
    public NameValue? gender;
    public NameValue? tribe;
    public NameValue? race;
    public List<ClassJobLevel> classjob_levels = new List<ClassJobLevel>();
    public NameValue? grand_company;
    public List<byte> grand_company_ranks = new List<byte>(); // TODO: introduce as a NameValue
    public NameValue? title;
    public uint? playtime;
    public int voice;

    // adventurer plate
    public AdventurerPlate plate = new AdventurerPlate();

    public bool is_battle_mentor;
    public bool is_trade_mentor;
    public bool is_novice;
    public bool is_returner;
    public short player_commendations;

    // Appearance
    public Appearance appearance = new Appearance();

    // inventory
    public InventoryContainer? inventory1;
    public InventoryContainer? inventory2;
    public InventoryContainer? inventory3;
    public InventoryContainer? inventory4;

    public InventoryContainer? equipped;

    public InventoryContainer? currency;

    public InventoryContainer? armory_off_hand;
    public InventoryContainer? armory_head;
    public InventoryContainer? armory_body;
    public InventoryContainer? armory_hands;
    public InventoryContainer? armory_waist;
    public InventoryContainer? armory_legs;
    public InventoryContainer? armory_feets;
    public InventoryContainer? armory_ear;
    public InventoryContainer? armory_neck;
    public InventoryContainer? armory_wrist;
    public InventoryContainer? armory_rings;
    public InventoryContainer? armory_soul_crystal;
    public InventoryContainer? armory_main_hand;

    // unlocks
    public List<byte> unlocks = new List<byte>();
    public List<byte> seen_active_help = new List<byte>();
    public List<byte> minions = new List<byte>();
    public List<byte> mounts = new List<byte>();
    public List<byte> orchestrion_rolls = new List<byte>();
    public List<byte> cutscene_seen = new List<byte>();
    public List<byte> ornaments = new List<byte>();
    public List<byte> caught_fish = new List<byte>();
    public List<byte> caught_spearfish = new List<byte>();
    public List<byte> adventures = new List<byte>();
    public List<byte> triple_triad_cards = new List<byte>();
    public List<byte> glasses_styles = new List<byte>();
    public List<byte> chocobo_taxi_stands = new List<byte>();
    public List<byte> titles = new List<byte>();
    public List<byte> unlocked_companion_equip = new List<byte>();

    // aether currents
    public List<byte> comp_flg_set = new List<byte>();
    public List<byte> unlocked_aether_currents = new List<byte>();

    // aetheryte
    public List<byte> unlocked_aetherytes = new List<byte>();
    public int homepoint;
    public List<ushort> favorite_aetherytes = new List<ushort>();
    public int free_aetheryte;

    // classjob
    public int current_class;
    public int first_class;
    public int rested_exp;

    // content
    public List<byte> unlocked_special_content = new List<byte>();
    public List<byte> unlocked_raids = new List<byte>();
    public List<byte> unlocked_dungeons = new List<byte>();
    public List<byte> unlocked_guildhests = new List<byte>();
    public List<byte> unlocked_trials = new List<byte>();
    public List<byte> unlocked_crystalline_conflicts = new List<byte>();
    public List<byte> unlocked_frontlines = new List<byte>();
    public List<byte> cleared_raids = new List<byte>();
    public List<byte> cleared_dungeons = new List<byte>();
    public List<byte> cleared_guildhests = new List<byte>();
    public List<byte> cleared_trials = new List<byte>();
    public List<byte> cleared_crystalline_conflicts = new List<byte>();
    public List<byte> cleared_frontlines = new List<byte>();
    public List<byte> cleared_masked_carnivale = new List<byte>();
    public List<byte> unlocked_misc_content = new List<byte>();
    public List<byte> cleared_misc_content = new List<byte>();

    // quest
    public List<byte> completed_quests = new List<byte>();

    // volatile
    public float position_x;
    public float position_y;
    public float position_z;
    public float rotation;
    public ushort zone_id;
}
