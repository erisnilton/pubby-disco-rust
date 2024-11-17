use crate::shared::vo::{Slug, UUID4};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Changes {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<Slug>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<Option<UUID4>>,
}
