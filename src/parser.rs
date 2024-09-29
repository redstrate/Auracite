use crate::data::CharacterData;
use regex::Regex;
use scraper::{Html, Selector};

const ENTRY_SELECTOR: &str = ".entry";
const ENTRY_NAME_SELECTOR: &str = ".entry__name";

/// Parses the HTML from `data` and returns the relative Lodestone URL for the first search entry.
pub fn parse_search(data: &str) -> String {
    let document = Html::parse_document(data);
    let mut href = String::new();

    for element in document.select(&Selector::parse(ENTRY_SELECTOR).unwrap()) {
        if let Some(block_name) = element
            .select(&Selector::parse(ENTRY_NAME_SELECTOR).unwrap())
            .nth(0)
        {
            if let Some(block_name) = element
                .select(&Selector::parse("a.entry__link").unwrap())
                .nth(0)
            {
                href = block_name.attr("href").unwrap().parse().unwrap();
            }
        }
    }

    href
}

const CHARACTER_NAME_SELECTOR: &str = ".frame__chara__name";
const WORLD_DATA_CENTER_SELECTOR: &str = ".frame__chara__world";
const CHARACTER_BLOCK_SELECTOR: &str = ".character-block__box";
const CHARACTER_BLOCK_TITLE_SELECTOR: &str = ".character-block__title";
const CHARACTER_BLOCK_NAME_SELECTOR: &str = ".character-block__name";
const FACE_IMG_SELECTOR: &str = ".frame__chara__face > img";
const PORTRAIT_IMG_SELECTOR: &str = ".character__detail__image > a > img";
const NAMEDAY_SELECTOR: &str = ".character-block__birth";

/// Parses the HTML from `data` and returns `CharacterData`. The data may be incomplete.
pub fn parse_lodestone(data: &str) -> CharacterData {
    let document = Html::parse_document(data);

    let mut char_data = CharacterData::default();

    for element in document.select(&Selector::parse(CHARACTER_NAME_SELECTOR).unwrap()) {
        char_data.name = element.inner_html();
    }

    for element in document.select(&Selector::parse(WORLD_DATA_CENTER_SELECTOR).unwrap()) {
        let re = Regex::new(r"(\w+)\s\[(\w+)\]").unwrap();
        let inner_html = element.inner_html();
        let captures = re.captures(&inner_html).unwrap();
        char_data.world = captures.get(1).unwrap().as_str().to_owned();
        char_data.data_center = captures.get(2).unwrap().as_str().to_owned();
    }

    for element in document.select(&Selector::parse(CHARACTER_BLOCK_SELECTOR).unwrap()) {
        if let Some(block_title) = element
            .select(&Selector::parse(CHARACTER_BLOCK_TITLE_SELECTOR).unwrap())
            .nth(0)
        {
            let name = block_title.inner_html();
            if name == "Race/Clan/Gender" {
                if let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .nth(0)
                {
                    let re = Regex::new(r"(\w+)<br>(\w+)\s\/\s(\W)").unwrap();
                    let inner_html = block_name.inner_html();
                    let captures = re.captures(&inner_html).unwrap();

                    char_data.race = captures.get(1).unwrap().as_str().to_owned();
                    char_data.subrace = captures.get(2).unwrap().as_str().to_owned();
                    if captures.get(3).unwrap().as_str() == "♀" {
                        char_data.gender = "Female".parse().unwrap();
                    } else {
                        char_data.gender = "Male".parse().unwrap();
                    }
                }
            } else if name == "City-state" {
                if let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .nth(0)
                {
                    char_data.city_state = block_name.inner_html();
                }
            } else if name == "Nameday" {
                for element in element.select(&Selector::parse(NAMEDAY_SELECTOR).unwrap()) {
                    char_data.nameday = element.inner_html();
                }

                if let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .nth(0)
                {
                    char_data.guardian = block_name.inner_html();
                }
            }
        }
    }

    for element in document.select(&Selector::parse(FACE_IMG_SELECTOR).unwrap()) {
        char_data.face_url = element.attr("src").unwrap().parse().unwrap();
    }

    for element in document
        .select(&Selector::parse(PORTRAIT_IMG_SELECTOR).unwrap())
        .nth(0)
    {
        char_data.portrait_url = element.attr("src").unwrap().parse().unwrap();
    }

    char_data
}
