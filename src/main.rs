mod scraper;
mod transformer;

use transformer::BuildFinder;

#[tokio::main]
async fn main() {
    let all_gods = transformer::make_god_list();
    let all_cards = transformer::load_god_cards();
    let all_builds = transformer::load_god_builds();

    let buildfinder = BuildFinder::new(all_gods, all_cards, all_builds);
}
