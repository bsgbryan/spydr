#[tokio::main]
async fn main() {
  if let Some(home_page) = spydr::sources::cnn::crawl().await {
    println!("{home_page}")
  }
}
