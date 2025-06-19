use url::Url;

pub async fn crawl(href: Url) -> Vec<Url> {
  if let Some(domain) = href.domain() {
    match domain {
      "www.cnn.com" => crate::sources::cnn::articles(href).await,
      "www.huffpost.com" =>
        if let Some(hp) = crate::sources::huffpost::crawl().await { hp.articles }
        else { Vec::with_capacity(0) }
      "www.pcmag.com" =>
        if let Some(hp) = crate::sources::pcmag::crawl().await { hp.articles }
        else { Vec::with_capacity(0) }
      _ => { Vec::with_capacity(0) }
    }
  }
  else { Vec::with_capacity(0) }
}