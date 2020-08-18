mod scraper;
mod transformer;

#[tokio::main]
async fn main() {
    let gods = transformer::make_god_list();
    let cards = transformer::make_god_cards(gods).await.unwrap();
    transformer::store_god_cards(cards);
    println!("done");
}
