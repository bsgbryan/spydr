use crate::home_page::HomePage;

pub async fn crawl() -> Vec<Option<HomePage>> {
  vec![
    crate::sources::cnn::crawl().await,
    crate::sources::huffpost::crawl().await,
    crate::sources::pcmag::crawl().await,
  ]
}