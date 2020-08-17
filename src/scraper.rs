use fantoccini::Client;
use serde_json::json;
use soup::prelude::*;
use soup::Soup;
use webdriver::capabilities::Capabilities;

const BASE_LINK: &str = "https://smitesource.com";
const HOST_LINK: &str = "http://localhost:4444";

/// Connect to the geckodriver process and enable headless mode.
async fn build_webdriver(link: &str) -> Result<Client, fantoccini::error::NewSessionError> {
    let mut cap = Capabilities::new();
    let args = json!({"args": ["-headless"]});
    cap.insert("moz:firefoxOptions".to_string(), args);
    let client = Client::with_capabilities(link, cap).await?;
    Ok(client)
}

/// Pull the HTML from a webpage using the headless driver.
async fn fetch_html(link: &str) -> Result<String, fantoccini::error::CmdError> {
    let mut c = build_webdriver(HOST_LINK)
        .await
        .expect("Failed to connect to geckodriver: Geckodriver should be running.");
    c.goto(link).await?;
    c.wait_for_find(fantoccini::Locator::Css(".build-card-list"))
        .await?;
    let page_data = c.source().await?;

    c.close().await?;
    Ok(page_data)
}

/// Hold data from build cards.
pub struct BuildCard {
    pub name: String,
    pub description: String,
    pub link: String,
}

/// Get a list of god build cards.
pub async fn get_god_build_list(link: &str) -> Result<Vec<BuildCard>, fantoccini::error::CmdError> {
    let page = fetch_html(link).await?;
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

    Ok(build_cards)
}
