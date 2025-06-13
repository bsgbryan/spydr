use std::fmt::Display;

use url::Url;

pub struct HomePage {
  pub sections:    Vec<Url>,
  pub subsections: Vec<Url>,
  pub articles:    Vec<Url>,
}

impl Display for HomePage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let _ = writeln!(f, "SECTIONS");
    for s in &self.sections {
      if let Some(domain) = s.domain() {
        let url = format!("{}://{}{}",
          s.scheme(),
          domain,
          s.path(),
        );

        let _ = writeln!(f, "  {}", url);
      }
    }

    let _ = writeln!(f, "SUB-SECTIONS");
    for s in &self.subsections {
      if let Some(domain) = s.domain() {
        let url = format!("{}://{}{}",
          s.scheme(),
          domain,
          s.path(),
        );

        let _ = writeln!(f, "  {}", url);
      }
    }

    let _ = writeln!(f, "ARTICLES");
    for a in &self.articles {
      if let Some(domain) = a.domain() {
        let url = format!("{}://{}{}",
          a.scheme(),
          domain,
          a.path(),
        );

        let _ = writeln!(f, "  {}", url);
      }
    }

    Ok(())
  }
}