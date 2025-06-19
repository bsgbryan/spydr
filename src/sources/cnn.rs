use scraper::{
  Html,
  Selector,
};
use url::Url;

use crate::{collect, fetch, home_page::HomePage};

pub async fn crawl() -> Option<HomePage> {
  crate::crawl::execute(
    "www.cnn.com",
    "container__link--type-article",
    "subnav__section-link",
    Some("subnav__subsection-link"),
  ).await
}

pub async fn articles(url: Url) -> Vec<Url> {
  if let Some(page) = fetch::execute(&url).await.ok() {
    let document = Html::parse_document(&page);

    match Selector::parse("a.container__link--type-article") {
      Ok (a) => collect::execute(&document, &a, "www.cnn.com"),
      Err(_) => Vec::with_capacity(0),
    }
  }
  else { Vec::with_capacity(0) }
} 
