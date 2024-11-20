#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Changes {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub media_type: Option<crate::domain::media::MediaType>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<crate::shared::vo::Slug>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub release_date: Option<chrono::NaiveDate>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub cover: Option<Option<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub genre_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub parental_rating: Option<u8>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub composer_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub interpreter_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub album_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_single: Option<bool>,
}
