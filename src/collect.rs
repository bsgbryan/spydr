use scraper::{
  Html,
  Selector,
};
use url::Url;

pub fn collect<'a>(
  document: &Html,
  selector: &Selector,
  domain:    Option<&'a str>,
) -> Vec<Url> {
  let mut out = vec![];
  for item in document.select(&selector) {
    match item.value().attr("href") {
      Some(href) => {
        match domain {
          Some(d) => {
            match Url::parse(&format!("https://{d}{href}")) {
              Ok(href) => out.push(href),
              Err(_)   => (),
            }
          }
          None => {
            match Url::parse(href) {
              Ok(href) => out.push(href),
              Err(_)   => (),
            }
          }
        }
        
      }
      None => {}
    }
  }
  out
}