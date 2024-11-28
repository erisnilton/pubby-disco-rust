use crate::{
  domain::genre::Genre,
  shared::vo::{Slug, UUID4},
};

#[derive(Debug, serde::Serialize)]
pub struct GenrePresenter {
  id: UUID4,
  name: String,
  slug: Slug,
}

#[derive(Debug, serde::Serialize)]
pub struct GenreAggregatePresenter {
  genre: GenrePresenter,
  related: Vec<GenrePresenter>,
}

impl From<crate::domain::genre::Genre> for GenrePresenter {
  fn from(value: crate::domain::genre::Genre) -> Self {
    Self {
      id: value.id().clone(),
      name: value.name().clone(),
      slug: value.slug().clone(),
    }
  }
}

impl From<crate::shared::paged::Paged<Genre>> for crate::shared::paged::Paged<GenrePresenter> {
  fn from(value: crate::shared::paged::Paged<Genre>) -> Self {
    Self {
      items: value.items.into_iter().map(GenrePresenter::from).collect(),
      total_items: value.total_items,
      total_pages: value.total_pages,
      page: value.page,
    }
  }
}

impl From<crate::domain::genre::GenreAggregate> for GenreAggregatePresenter {
  fn from(value: crate::domain::genre::GenreAggregate) -> Self {
    Self {
      genre: value.genre().clone().into(),
      related: value
        .related()
        .iter()
        .map(|genre| GenrePresenter::from(genre.clone()))
        .collect(),
    }
  }
}
