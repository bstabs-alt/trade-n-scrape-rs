use reqwest;

pub async fn fetch_category_page(url: &str) -> Result<String, reqwest::Error> {
    println!("Fetching content from: {:?}", url);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}