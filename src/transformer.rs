use serde_json::{Result, Value};
use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

const GOD_JSON_DIR: &str = "resources/gods.json";

/// Download the god data into a .json file for use with other functions.
pub fn store_god_json(builder: &QueryBuilder) -> Result<()> {
    // TODO: Handle this
    let data = builder.get_gods().unwrap();

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
