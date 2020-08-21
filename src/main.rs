mod scraper;
mod transformer;

#[tokio::main]
async fn main() {
    let cards = transformer::load_god_cards();
    for c in cards {
        println!("{}", c.cards[0].name);
    }
    println!("done");
}
