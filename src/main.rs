use smite_api_library::json;

mod scraper;
mod transformer;

#[tokio::main]
async fn main() {
    let page = scraper::get_god_build_list("https://smitesource.com/gods/1699").await;
    println!(
        "{}\n{}\n{}",
        page[0].name, page[0].description, page[0].link
    );
}
