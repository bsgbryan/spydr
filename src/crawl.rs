use scraper::{Html, Selector};
use url::Url;

use crate::{
  collect::collect,
  fetch::execute as fetch,
  home_page::HomePage,
};

pub async fn execute(
  domain:      &str,
  articles:    &str,
  sections:    &str,
  subsections: &str,
) -> Option<HomePage> {
  if let Some(url)  = Url::parse(&format!("https://{domain}")).ok() &&
     let Some(page) = fetch(&url).await
  {
    let document = Html::parse_document(&page);

    return Some(HomePage {
      articles: match Selector::parse(&format!("a.{articles}")) {
        Ok (a) => collect(&document, &a, Some(domain)),
        Err(_) => Vec::with_capacity(0),
      },
      sections: match Selector::parse(&format!("a.{sections}")) {
        Ok (s) => collect(&document, &s, None).iter().filter(|&url| {
          match url.domain() {
            Some(d) => d == domain,
            None    => false,
          }
        }).cloned().collect(),
        Err(_) => Vec::with_capacity(0),
      },
      subsections: match Selector::parse(&format!("a.{subsections}")) {
        Ok (s) => collect(&document, &s, None).iter().filter(|&url| {
          match url.domain() {
            Some(d) => d == domain,
            None    => false,
          }
        }).cloned().collect(),
        Err(_) => Vec::with_capacity(0),
      },
    })
  }

  None
}
