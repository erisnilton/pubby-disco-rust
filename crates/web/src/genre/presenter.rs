use domain::genre::{aggregate::GenreAggregate, entity::Genre};
use shared::vo::{Slug, UUID4};

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

impl From<Genre> for GenrePresenter {
  fn from(value: Genre) -> Self {
    Self {
      id: value.id().clone(),
      name: value.name().clone(),
      slug: value.slug().clone(),
    }
  }
}

impl From<GenreAggregate> for GenreAggregatePresenter {
  fn from(value: GenreAggregate) -> Self {
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
