use scraper::{
  Html,
  Selector,
};
use url::Url;

use crate::{
  collect::execute as collect,
  fetch::execute   as fetch,
  home_page::HomePage,
};

pub async fn crawl() -> Option<HomePage> {
  if let Some(url)  = Url::parse("https://www.pcmag.com").ok() &&
     let Some(page) = fetch(&url).await
  {
    let document = Html::parse_document(&page);

    return Some(HomePage {
      articles: match Selector::parse("section.container > div > div > div > div > a") {
      Ok (s) => collect(&document, &s, "www.pcmag.com"),
      Err(_) => Vec::with_capacity(0),
    },
      sections: match Selector::parse("header > nav > div.container > section > aside > nav > ul > li > a") {
      Ok (s) => collect(&document, &s, "www.pcmag.com"),
      Err(_) => Vec::with_capacity(0),
    },
      subsections: match Selector::parse("header > nav > div.container > section > aside > nav > ul > li > ul > li > a") {
      Ok (s) => collect(&document, &s, "www.pcmag.com"),
      Err(_) => Vec::with_capacity(0),
    },
    })
  }
  None
}
