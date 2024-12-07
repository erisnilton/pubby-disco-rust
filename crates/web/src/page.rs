use shared::paged::RequestPageParams;

#[derive(Debug, serde::Deserialize)]
pub struct PageParams {
  pub page: Option<usize>,
  pub per_page: Option<usize>,
}

impl From<PageParams> for RequestPageParams {
  fn from(value: PageParams) -> Self {
    Self {
      page: value.page.unwrap_or(1),
      per_page: value.per_page.unwrap_or(20),
    }
  }
}
