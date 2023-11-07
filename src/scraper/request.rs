use reqwest;

pub async fn fetch_category_page(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::
    let body = response;
    Ok(body)
}