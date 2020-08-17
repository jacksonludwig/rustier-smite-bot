use fantoccini::Client;
use serde_json::json;
use soup::prelude::*;
use soup::Soup;
use webdriver::capabilities::Capabilities;

const BASE_LINK: &str = "https://smitesource.com";
const HOST_LINK: &str = "http://localhost:4444";

/// Connect to the geckodriver process and enable headless mode.
pub async fn build_webdriver(link: &str) -> Result<Client, fantoccini::error::NewSessionError> {
    let mut cap = Capabilities::new();
    let args = json!({"args": ["-headless"]});
    cap.insert("moz:firefoxOptions".to_string(), args);
    let client = Client::with_capabilities(link, cap).await?;
    Ok(client)
}

/// Pull the HTML from a webpage using the headless driver.
pub async fn fetch_html(link: &str) -> Result<String, fantoccini::error::CmdError> {
    let mut c = build_webdriver(HOST_LINK)
        .await
        .expect("failed to connect to WebDriver");
    c.goto(link).await?;
    c.wait_for_find(fantoccini::Locator::Css(".build-card-list"))
        .await?;
    let page_data = c.source().await?;

    c.close().await?;
    Ok(page_data)
}

pub struct BuildCard {
    pub name: String,
    pub description: String,
    pub link: String,
}

pub async fn get_god_build_list(link: &str) -> Vec<BuildCard> {
    let page = fetch_html(link)
        .await
        .expect("Unable to fetch data from smite source");
    let soup = Soup::new(&page);

    let mut build_cards: Vec<BuildCard> = vec![];
    for (_, link) in soup.class("build-card").find_all().enumerate() {
        let name = link
            .tag("h4")
            .find()
            .expect("Build is missing name (h4) tag")
            .text();
        let description = link
            .tag("h5")
            .find()
            .expect("Build is missing description (h5) tag")
            .text();
        let link = format!(
            "{}{}",
            BASE_LINK,
            link.get("href").expect("Build is missing link (href) tag.")
        );
        build_cards.push(BuildCard {
            name,
            description,
            link,
        });
    }

    build_cards
}
