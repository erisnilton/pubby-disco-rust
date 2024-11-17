#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Changes {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub country: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<crate::shared::vo::Slug>,
}
