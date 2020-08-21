mod scraper;
mod transformer;

use transformer::FullBuild;

#[tokio::main]
async fn main() {
    let all_cards = transformer::load_god_cards();
    let mut all_builds: Vec<FullBuild> = vec![];
    for c in all_cards {
        for inner in c.cards {
            let build = transformer::get_full_build(&inner).await.unwrap();
            all_builds.push(build);
        }
    }
    transformer::store_god_builds(all_builds);
    println!("done");
}
