use crate::{
    data::CharacterData,
    value::{
        CityStateValue, ClassJobValue, GenderValue, GuardianValue, ItemValue, NamedayValue,
        RaceValue, TribeValue, WorldValue,
    },
};
use regex::Regex;
use scraper::{Html, Selector};

const ENTRY_SELECTOR: &str = ".entry";
const ENTRY_NAME_SELECTOR: &str = ".entry__name";

/// Parses the HTML from `data` and returns the relative Lodestone URL for the first search entry.
pub fn parse_search(data: &str) -> String {
    let document = Html::parse_document(data);
    let mut href = String::new();

    for element in document.select(&Selector::parse(ENTRY_SELECTOR).unwrap()) {
        if let Some(_) = element
            .select(&Selector::parse(ENTRY_NAME_SELECTOR).unwrap())
            .next()
            && let Some(block_name) = element
                .select(&Selector::parse("a.entry__link").unwrap())
                .next()
        {
            href = block_name.attr("href").unwrap().parse().unwrap();
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
const FREE_COMPANY_SELECTOR: &str = ".character__freecompany__name > h4 > a";
const TITLE_SELECTOR: &str = ".frame__chara__title";

/// Parses the HTML from `data` and returns `CharacterData`. The data may be incomplete.
pub fn parse_profile(data: &str, char_data: &mut CharacterData) {
    let document = Html::parse_document(data);

    for element in document.select(&Selector::parse(CHARACTER_NAME_SELECTOR).unwrap()) {
        char_data.name = element.inner_html();
    }

    if let Some(title) = document
        .select(&Selector::parse(TITLE_SELECTOR).unwrap())
        .next()
    {
        char_data.title = Some(title.inner_html().as_str().to_string());
    }

    let world_re = Regex::new(r"(\w+)\s\[(\w+)\]").unwrap();
    for element in document.select(&Selector::parse(WORLD_DATA_CENTER_SELECTOR).unwrap()) {
        let inner_html = element.inner_html();
        let captures = world_re.captures(&inner_html).unwrap();
        // TODO: use error
        char_data.world = WorldValue::try_from(captures.get(1).unwrap().as_str()).unwrap();
        char_data.data_center = captures.get(2).unwrap().as_str().to_owned();
    }

    let block_re = Regex::new(r"([^<]+)<br>([^\/]+)\s\/\s(\W)").unwrap();
    let grand_re = Regex::new(r"([^\/]+)\s\/\s([^\/]+)").unwrap();
    for element in document.select(&Selector::parse(CHARACTER_BLOCK_SELECTOR).unwrap()) {
        if let Some(block_title) = element
            .select(&Selector::parse(CHARACTER_BLOCK_TITLE_SELECTOR).unwrap())
            .next()
        {
            let name = block_title.inner_html();
            if name == "Race/Clan/Gender" {
                if let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .next()
                {
                    let inner_html = block_name.inner_html();
                    let captures = block_re.captures(&inner_html).unwrap();

                    char_data.race =
                        RaceValue::try_from(captures.get(1).unwrap().as_str()).unwrap();
                    char_data.tribe =
                        TribeValue::try_from(captures.get(2).unwrap().as_str()).unwrap();
                    char_data.gender =
                        GenderValue::try_from(captures.get(3).unwrap().as_str()).unwrap();
                }
            } else if name == "City-state" {
                if let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .next()
                {
                    char_data.city_state =
                        CityStateValue::try_from(block_name.inner_html().as_str()).unwrap();
                }
            } else if name == "Nameday" {
                for element in element.select(&Selector::parse(NAMEDAY_SELECTOR).unwrap()) {
                    char_data.nameday =
                        NamedayValue::try_from(element.inner_html().as_str()).unwrap();
                }

                if let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .next()
                {
                    char_data.guardian =
                        GuardianValue::try_from(block_name.inner_html().as_str()).unwrap();
                }
            } else if name == "Grand Company"
                && let Some(block_name) = element
                    .select(&Selector::parse(CHARACTER_BLOCK_NAME_SELECTOR).unwrap())
                    .next()
            {
                let inner_html = block_name.inner_html();
                let captures = grand_re.captures(&inner_html).unwrap();

                char_data.grand_company.name = captures.get(1).unwrap().as_str().to_string();
                char_data.grand_company.rank = captures.get(2).unwrap().as_str().to_string();
            }
        }

        if let Some(free_company) = element
            .select(&Selector::parse(FREE_COMPANY_SELECTOR).unwrap())
            .next()
        {
            char_data.free_company = Some(free_company.inner_html().as_str().to_string());
        }
    }

    for element in document.select(&Selector::parse(FACE_IMG_SELECTOR).unwrap()) {
        char_data.face_url = element.attr("src").unwrap().parse().unwrap();
    }

    if let Some(element) = document
        .select(&Selector::parse(PORTRAIT_IMG_SELECTOR).unwrap())
        .next()
    {
        char_data.portrait_url = element.attr("src").unwrap().parse().unwrap();
    }

    // TODO: support facewear
    let item_slot_selectors = [
        ".icon-c--0",  // Main Hand
        ".icon-c--1",  // Off hand
        ".icon-c--2",  // Head
        ".icon-c--3",  // Body
        ".icon-c--4",  // Hands
        ".icon-c--6",  // Legs
        ".icon-c--7",  // Feet
        ".icon-c--8",  // Earrings
        ".icon-c--9",  // Necklace
        ".icon-c--10", // Bracelets
        ".icon-c--11", // Left Ring
        ".icon-c--12", // Right Ring
        ".icon-c--13", // Soul Crystal
    ];

    for (i, selector) in item_slot_selectors.iter().enumerate() {
        if let Some(slot) = document.select(&Selector::parse(selector).unwrap()).next()
            && let Some(item) = slot.select(&Selector::parse(".db-tooltip").unwrap()).next()
        {
            let parsed_item = parse_item_tooltip(&item);
            let slot = match i {
                0 => &mut char_data.equipped.main_hand,
                1 => &mut char_data.equipped.off_hand,
                2 => &mut char_data.equipped.head,
                3 => &mut char_data.equipped.body,
                4 => &mut char_data.equipped.hands,
                5 => &mut char_data.equipped.legs,
                6 => &mut char_data.equipped.feet,
                7 => &mut char_data.equipped.earrings,
                8 => &mut char_data.equipped.necklace,
                9 => &mut char_data.equipped.bracelets,
                10 => &mut char_data.equipped.left_ring,
                11 => &mut char_data.equipped.right_ring,
                12 => &mut char_data.equipped.soul_crystal,
                _ => panic!("Unexpected slot!"),
            };

            *slot = parsed_item;
        }
    }
}

const CLASSJOB_SELECTOR: &str = ".character__job > li";
const CLASSJOB_LEVEL_SELECTOR: &str = ".character__job__level";
const CLASSJOB_NAME_SELECTOR: &str = ".character__job__name";
const CLASSJOB_EXP_SELECTOR: &str = ".character__job__exp";

/// Parses the HTML from `data` and returns `CharacterData`. The data may be incomplete.
pub fn parse_classjob(data: &str, char_data: &mut CharacterData) {
    let document = Html::parse_document(data);

    for element in document.select(&Selector::parse(CLASSJOB_SELECTOR).unwrap()) {
        let level = element
            .select(&Selector::parse(CLASSJOB_LEVEL_SELECTOR).unwrap())
            .next()
            .unwrap();
        let name = element
            .select(&Selector::parse(CLASSJOB_NAME_SELECTOR).unwrap())
            .next()
            .unwrap();
        let exp_element = element
            .select(&Selector::parse(CLASSJOB_EXP_SELECTOR).unwrap())
            .next()
            .unwrap();

        let mut exp = None;
        let mut max_exp = None;
        if let Some((exp_text, max_exp_text)) = exp_element.inner_html().split_once(" / ") {
            exp = exp_text.replace(",", "").parse().ok();
            max_exp = max_exp_text.replace(",", "").parse().ok();
        }

        // skip levels that are -, which means they don't even have the classjob
        if let Ok(level) = level.inner_html().parse() {
            let mut class_job_value =
                ClassJobValue::try_from(name.inner_html().as_str()).unwrap_or_default();
            class_job_value.level = level;
            class_job_value.exp = exp;
            class_job_value.max_exp = max_exp;
            char_data.classjob_levels.push(class_job_value);
        }
    }
}

fn parse_item_tooltip(element: &scraper::ElementRef<'_>) -> Option<ItemValue> {
    if let Some(slot) = element
        .select(&Selector::parse(".db-tooltip__item__name").unwrap())
        .next()
    {
        let mut text: String = slot.text().collect();
        if text.contains("\u{e03c}") {
            text = text.strip_suffix("\u{e03c}").unwrap().to_string();
        }
        return Some(ItemValue { name: text });
    }

    None
}
