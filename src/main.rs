//TODO: integrate serenity
mod scraper;
mod transformer;

#[tokio::main]
async fn main() {
    let mut builds_for_god = scraper::get_god_build_cards("https://smitesource.com/gods/3585")
        .await
        .unwrap();

    let build = scraper::get_god_explanation(builds_for_god.pop().unwrap())
        .await
        .unwrap();
    println!("{:?}", build);
}
