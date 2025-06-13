use crate::home_page::HomePage;

pub async fn crawl() -> Option<HomePage> {
  crate::crawl::execute(
    "www.cnn.com",
    "container__link--type-article",
    "subnav__section-link",
    "subnav__subsection-link",
  ).await
}
