use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

use super::scraper;
use super::scraper::BuildCard;

const GOD_JSON_DIR: &str = "resources/gods.json";
const CARDS_JSON_DIR: &str = "resources/cards.json";
const BASE_LINK: &str = "https://smitesource.com/gods/";

/// Download the god data into a .json file for use with other functions.
pub fn store_god_json(builder: &QueryBuilder) -> Result<(), reqwest::Error> {
    let data = builder.get_gods()?;

    let _ = json::write_string_to_file(GOD_JSON_DIR, data).unwrap();

    Ok(())
}

pub struct God {
    pub name: String,
    pub id: String,
}

impl God {
    fn new(name: String, id: String) -> Self {
        God { name, id }
    }
}

/// Read the god data and make it into a Vec of Gods, which only store the name
/// and id.
pub fn make_god_list() -> Vec<God> {
    let data = json::read_file_to_string(GOD_JSON_DIR).unwrap();
    let values: Vec<Value> = serde_json::from_str(&data).unwrap();
    let mut god_tuples: Vec<God> = vec![];
    for god in values {
        let tuple = God::new(
            god["Name"].as_str().unwrap().to_string(),
            god["id"].to_string(),
        );
        god_tuples.push(tuple);
    }
    god_tuples
}

/// Get all of the links from a Vec of Gods.
fn make_god_links(gods: &Vec<God>) -> Vec<String> {
    gods.iter()
        .map(|g| format!("{}{}", BASE_LINK, g.id))
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct SingleGodCardHolder {
    pub id: String,
    pub cards: Vec<BuildCard>,
}

impl SingleGodCardHolder {
    pub fn new(id: String, cards: Vec<BuildCard>) -> Self {
        SingleGodCardHolder { id, cards }
    }
}

/// Use the Vec of all gods to create a struct of AllGodCardHolder. This contains a list of all god cards
/// associated with a specific god id in a concise fashion to use with serde.
/// God and Link vector will always be the same size.
pub async fn make_god_cards(
    gods: &Vec<God>,
) -> Result<Vec<SingleGodCardHolder>, fantoccini::error::CmdError> {
    let links = make_god_links(gods);
    let mut all_holder: Vec<SingleGodCardHolder> = vec![];

    for i in 0..links.len() {
        let id = &gods[i].id;
        let cards = scraper::get_god_build_cards(&links[i]).await?;
        let holder = SingleGodCardHolder::new(id.to_string(), cards);
        all_holder.push(holder);
    }

    Ok(all_holder)
}

/// Take the vector of all god cards and store them in a json file.
pub fn store_god_cards(all_holder: Vec<SingleGodCardHolder>) {
    let data = serde_json::to_string(&all_holder).unwrap();
    json::write_string_to_file(CARDS_JSON_DIR, data).unwrap();
}

/// Load cards from a json file.
pub fn load_god_cards() -> Vec<SingleGodCardHolder> {
    let data = json::read_file_to_string(CARDS_JSON_DIR).unwrap();
    let cards: Vec<SingleGodCardHolder> = serde_json::from_str(&data).unwrap();
    cards
}

pub struct FullBuild {
    pub starter: Vec<String>,
    pub relics: Vec<String>,
    pub ending: Vec<String>,
    pub explanation: String,
}

/// Get the full god build given a build card.
pub async fn get_full_build(card: &BuildCard) -> Result<FullBuild, fantoccini::error::CmdError> {
    let starter = scraper::get_starter_god_build(card).await?;
    let relics = scraper::get_god_relics(card).await?;
    let ending = scraper::get_final_god_build(card).await?;
    let explanation = scraper::get_god_explanation(card).await?;
    Ok(FullBuild {starter, relics, ending, explanation})
}
