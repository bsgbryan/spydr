use scraper::{
  Html,
  Selector,
};
use url::Url;

pub fn execute<'a>(
  document: &Html,
  selector: &Selector,
  domain:   &'a str,
) -> Vec<Url> {
  let mut out = vec![];
  for item in document.select(&selector) {
    match item.value().attr("href") {
      Some(href) => {
        if href.starts_with(&format!("https://{domain}")) {
          match Url::parse(href) {
            Ok(href) => out.push(href),
            Err(_)   => (),
          }
        }
        else {
          match Url::parse(&format!("https://{domain}{href}")) {
            Ok(href) => out.push(href),
            Err(_)   => (),
          }
        }
      }
      None => {}
    }
  }
  out
}