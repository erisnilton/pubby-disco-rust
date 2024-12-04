use crate::domain::media::Media;

pub struct MediaAggregate {
  pub media: Media,
  pub interpreters: Vec<crate::domain::artist::Artist>,
  pub composers: Vec<crate::domain::artist::Artist>,
  pub sources: Vec<crate::domain::source::source_entity::Source>,
}
