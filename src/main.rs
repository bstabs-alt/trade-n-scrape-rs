mod scraper;




#[tokio::main]
async fn main() {
    println!("Welcome to Trade N Scrape!");
    let url = "https://www.trademe.co.nz/a/marketplace";

    let html_content = scraper::request::fetch_category_page(url).await.expect("failed to fetch html content!");
    
    if let Err(e) = scraper::parse::parse_categories(html_content).await {
        eprintln!("Error getting categories: {}", e);
    }
}
