use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Paged<T> {
  pub items: Vec<T>,
  pub total_items: usize,
  pub total_pages: usize,
  pub page: usize,
}

impl<T> Default for Paged<T> {
  fn default() -> Self {
    Paged {
      items: Vec::new(),
      total_items: 0,
      total_pages: 1,
      page: 1,
    }
  }
}

pub type PageTuple<T> = (usize, Vec<T>);

/**
 * Normalized pagination parameters
 */
#[derive(Debug, Clone)]
pub struct PageQueryParams {
  pub skip: usize,
  pub take: usize,
}

impl Default for PageQueryParams {
  fn default() -> Self {
    Self { skip: 0, take: 20 }
  }
}

/**
 * Request pagination parameters
 */
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RequestPageParams {
  pub page: usize,
  pub per_page: usize,
}

impl From<RequestPageParams> for PageQueryParams {
  fn from(params: RequestPageParams) -> Self {
    Self {
      skip: params.page.checked_sub(1).unwrap_or_default() * params.per_page,
      take: params.per_page,
    }
  }
}

impl<T> Paged<T> {
  pub fn from_tuple((total_items, items): PageTuple<T>, params: RequestPageParams) -> Self {
    Self {
      items,
      total_items,
      total_pages: (total_items as f64 / params.per_page as f64).ceil() as usize,
      page: params.page,
    }
  }
}
