use soup::prelude::*;
use soup::Soup;

const BASE_LINK: &str = "https://smitesource.com";

// TODO: This needs to use a headless webdriver to load html from page javascript...
pub fn fetch_html(link: &str) -> String {
    String::from("placeholder")
}

pub fn get_god_build_list(link: &str) -> Result<Vec<String>, reqwest::Error> {
    let page = fetch_html(link);
    let soup = Soup::new(&page);
    let build_cards = soup
        .tag("a")
        .find_all()
        .map(|a| a.display())
        .collect::<Vec<String>>();

    Ok(build_cards)
}
