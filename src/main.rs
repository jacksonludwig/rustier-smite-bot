mod scraper;
mod transformer;

#[tokio::main]
async fn main() {
    let gods = transformer::make_god_list();
    let links = transformer::make_god_links(gods);
    println!("{:?}", links);
}
