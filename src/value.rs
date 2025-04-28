use regex::Regex;
use serde::Serialize;

use crate::ArchiveError;

// TODO: does it make sense to implement Default?
#[derive(Default, Serialize)]
pub struct WorldValue {
    /// Name of the world.
    pub name: String,
    /// Internal ID of the world.
    pub value: i32,
}

impl TryFrom<&str> for WorldValue {
    type Error = ArchiveError;

    fn try_from(name: &str) -> Result<Self, ArchiveError> {
        let value = match name {
            "Adamantoise" => 73,
            "Cactuar" => 79,
            "Faerie" => 54,
            "Gilgamesh" => 63,
            "Jenova" => 40,
            "Midgardsormr" => 65,
            "Sargatanas" => 99,
            "Siren" => 57,
            "Balmung" => 91,
            "Brynhildr" => 34,
            "Coeurl" => 74,
            "Diabolos" => 62,
            "Goblin" => 81,
            "Malboro" => 75,
            "Mateus" => 37,
            "Zalera" => 41,
            "Behemoth" => 78,
            "Excalibur" => 93,
            "Exodus" => 53,
            "Famfrit" => 35,
            "Hyperion" => 95,
            "Lamia" => 55,
            "Leviathan" => 64,
            "Ultros" => 77,
            "Halicarnassus" => 406,
            "Maduin" => 407,
            "Marilith" => 404,
            "Seraph" => 405,
            "Cuchulainn" => 408,
            "Golem" => 411,
            "Kraken" => 409,
            "Rafflesia" => 410,
            "Cerberus" => 80,
            "Louisoix" => 83,
            "Moogle" => 71,
            "Omega" => 39,
            "Phantom" => 401,
            "Ragnarok" => 97,
            "Sagittarius" => 400,
            "Spriggan" => 85,
            "Alpha" => 402,
            "Lich" => 36,
            "Odin" => 66,
            "Phoenix" => 56,
            "Raiden" => 403,
            "Shiva" => 67,
            "Twintania" => 33,
            "Zodiark" => 42,
            "Aegis" => 90,
            "Atomos" => 68,
            "Carbuncle" => 45,
            "Garuda" => 58,
            "Gungnir" => 94,
            "Kujata" => 49,
            "Tonberry" => 72,
            "Typhon" => 50,
            "Alexander" => 43,
            "Bahamut" => 69,
            "Durandal" => 92,
            "Fenrir" => 46,
            "Ifrit" => 59,
            "Ridill" => 98,
            "Tiamat" => 76,
            "Ultima" => 51,
            "Anima" => 44,
            "Asura" => 23,
            "Chocobo" => 70,
            "Hades" => 47,
            "Ixion" => 48,
            "Masamune" => 96,
            "Pandaemonium" => 28,
            "Titan" => 61,
            "Belias" => 24,
            "Mandragora" => 82,
            "Ramuh" => 60,
            "Shinryu" => 29,
            "Unicorn" => 30,
            "Valefor" => 52,
            "Yojimbo" => 31,
            "Zeromus" => 32,
            "Bismarck" => 22,
            "Ravana" => 21,
            "Sephirot" => 86,
            "Sophia" => 87,
            "Zurvan" => 88,
            _ => return Err(ArchiveError::ParsingError),
        };

        Ok(Self {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Default, Serialize)]
pub struct CityStateValue {
    /// Name of the city-state.
    pub name: String,
    /// Internal ID of the city-state.
    pub value: i32,
}

impl TryFrom<&str> for CityStateValue {
    type Error = ArchiveError;

    fn try_from(name: &str) -> Result<Self, ArchiveError> {
        let value = match name {
            "Limsa Lominsa" => 1,
            "Gridania" => 2,
            "Ul'dah" => 3,
            _ => return Err(ArchiveError::ParsingError),
        };

        Ok(Self {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Default, Serialize)]
pub struct GenderValue {
    /// Name of the gender.
    pub name: String,
    /// Internal ID of the gender.
    pub value: i32,
}

impl TryFrom<&str> for GenderValue {
    type Error = ArchiveError;

    fn try_from(name: &str) -> Result<Self, ArchiveError> {
        let (value, name) = match name {
            "♂" => (0, "Male"),
            "♀" => (1, "Female"),
            _ => return Err(ArchiveError::ParsingError),
        };

        Ok(Self {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Default, Serialize)]
pub struct RaceValue {
    /// Name of the race.
    pub name: String,
    /// Internal ID of the race.
    pub value: i32,
}

impl TryFrom<&str> for RaceValue {
    type Error = ArchiveError;

    fn try_from(name: &str) -> Result<Self, ArchiveError> {
        let value = match name {
            "Hyur" => 1,
            "Elezen" => 2,
            "Lalafell" => 3,
            "Miqo'te" => 4,
            "Roegadyn" => 5,
            "Au Ra" => 6,
            "Hrothgar" => 7,
            "Viera" => 8,
            _ => return Err(ArchiveError::ParsingError),
        };

        Ok(Self {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Default, Serialize)]
pub struct TribeValue {
    /// Name of the tribe.
    pub name: String,
    /// Internal ID of the tribe.
    pub value: i32,
}

impl TryFrom<&str> for TribeValue {
    type Error = ArchiveError;

    fn try_from(name: &str) -> Result<Self, ArchiveError> {
        let value = match name {
            "Midlander" => 1,
            "Highlander" => 2,
            "Wildwood" => 3,
            "Duskwight" => 4,
            "Plainsfolk" => 5,
            "Dunesfolk" => 6,
            "Seeker" => 7,
            "Keeper of the Moon" => 8,
            "SeaWolf" => 9,
            "Hellsguard" => 10,
            "Raen" => 11,
            "Xaela" => 12,
            "Hellion" => 13,
            "Lost" => 14,
            "Rava" => 15,
            "Veena" => 16,
            _ => return Err(ArchiveError::ParsingError),
        };

        Ok(Self {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Default, Serialize)]
pub struct GuardianValue {
    /// Name of the guardian.
    pub name: String,
    /// Internal ID of the guardian.
    pub value: i32,
}

impl TryFrom<&str> for GuardianValue {
    type Error = ArchiveError;

    fn try_from(name: &str) -> Result<Self, ArchiveError> {
        let value = match name {
            "Halone, the Fury" => 1,
            "Menphina, the Lover" => 2,
            "Thaliak, the Scholar" => 3,
            "Nymeia, the Spinner" => 4,
            "Llymlaen, the Navigator" => 5,
            "Oschon, the Wanderer" => 6,
            "Byregot, the Builder" => 7,
            "Rhalgr, the Destroyer" => 8,
            "Azeyma, the Warden" => 9,
            "Nald'thal, the Traders" => 10,
            "Nophica, the Matron" => 11,
            "Althyk, the Keeper" => 12,
            _ => return Err(ArchiveError::ParsingError),
        };

        Ok(Self {
            name: name.to_string(),
            value,
        })
    }
}

#[derive(Default, Serialize)]
pub struct NamedayValue {
    /// String represenation of your nameday.
    pub value: String,
    /// Day part of your nameday.
    pub day: i32,
    /// Month part of your nameday.
    pub month: i32,
}

impl TryFrom<&str> for NamedayValue {
    type Error = ArchiveError;

    fn try_from(value: &str) -> Result<Self, ArchiveError> {
        let re = Regex::new(r"(\d{1,2})[^\d]+(\d{1,2})").unwrap();
        let captures = re.captures(&value).unwrap();

        Ok(Self {
            value: value.to_string(),
            day: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            month: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        })
    }
}

#[derive(Default, Serialize)]
pub struct ClassJobValue {
    /// Name of the class or job.
    pub name: String,
    /// Level of the class or job.
    pub level: i32,
}

#[derive(Default, Serialize)]
pub struct GrandCompanyValue {
    /// Name of the grand company.
    pub name: String,
    /// Name of your rank in the grand company.
    pub rank: String,
}

#[derive(Default, Serialize)]
pub struct ItemValue {
    /// Name of the item.
    pub name: String,
}
