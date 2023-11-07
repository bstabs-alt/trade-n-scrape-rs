use scraper::{Html, Selector};
use reqwest::Error;
use crate::scraper::save_html;

pub async fn parse_categories(html_content: String) -> Result<(), Error>{
    println!("Parsing categories..."); // "a.tm-marketplace-cat-splat__link"

    let delimeter  = "li a";
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("li a").expect("Failed to parse selector!");

    let elements = document.select(&selector).collect::<Vec<_>>();
    if elements.is_empty(){
        println!("No elements matched the selector {delimeter}!");
        save_html::save_html_to_file(&html_content.as_str());
        return Ok(());
    }

    for element in elements {
        let category_name = element.text().collect::<Vec<_>>().join("");

        let category_link = element.value().attr("href").unwrap_or("No link found!");

        println!("{}: {}", category_name, category_link);
    }

    Ok(())
}