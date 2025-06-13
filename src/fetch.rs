use url::Url;

pub async fn execute(
  url: &Url
) -> Option<String> {
  match reqwest::get(url.as_str()).await {
    Ok(response) => {
      match response.text().await {
        Ok(html) => Some(html),
        Err(_)   => None,
      }
    }
    Err(_) => None
  }
}