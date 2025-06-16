#[tokio::main]
async fn main() {
  if let Some(home_page) = spydr::sources::cnn::crawl().await {
    println!("CNN\n{home_page}")
  }

  if let Some(home_page) = spydr::sources::huffpost::crawl().await {
    println!("\nHuffPost\n{home_page}")
  }

  if let Some(home_page) = spydr::sources::pcmag::crawl().await {
    println!("\nPCMAG\n{home_page}")
  }
}
