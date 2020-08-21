mod scraper;
mod transformer;

#[tokio::main]
async fn main() {
    let cards = transformer::load_god_cards();
    for c in cards {
        let build = transformer::get_full_build(&c.cards[0]).await.unwrap();
        println!("{:?}\n{:?}\n{:?}\n{}", build.starter, build.relics, build.ending, build.explanation);
    }
    println!("done");
}
