use crate::home_page::HomePage;

pub async fn crawl() -> Option<HomePage> {
  crate::crawl::execute(
    "www.huffpost.com",
    "card__image__link",
    "site-footer__section",
    None,
  ).await
}
