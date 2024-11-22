use domain_proc_macros::Entity;

use crate::shared::{util::naive_now, vo::UUID4};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SourceType {
  Youtube,
  Soundcloud,
  Vimeo,
}

impl std::fmt::Display for SourceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SourceType::Youtube => write!(f, "Youtube"),
      SourceType::Soundcloud => write!(f, "Soundcloud"),
      SourceType::Vimeo => write!(f, "Video"),
    }
  }
}

impl std::str::FromStr for SourceType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Youtube" => Ok(SourceType::Youtube),
      "Soundcloud" => Ok(SourceType::Soundcloud),
      "Video" => Ok(SourceType::Vimeo),
      _ => Err(format!("Invalid SourceType: {}", s)),
    }
  }
}

#[derive(Debug, Entity, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Source {
  id: UUID4,
  source_type: SourceType,
  src: String,
  media_id: UUID4,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Default for Source {
  fn default() -> Self {
    let now = naive_now();
    Self {
      id: UUID4::default(),
      source_type: SourceType::Youtube,
      src: String::new(),
      media_id: UUID4::default(),
      created_at: now,
      updated_at: now,
    }
  }
}
