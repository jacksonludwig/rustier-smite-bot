use smite_api_library::json;
use smite_api_library::queries::QueryBuilder;

mod scraper;
mod transformer;

fn main() {
    let discord_token = json::read_secret("disc_token");
    println!("{}", discord_token);

    let page = scraper::get_god_build_list("https://smitesource.com/gods/1699");
    println!("{:?}", page.unwrap());
}
