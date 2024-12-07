use crate::{artist::entity::Artist, source::entity::Source};

use super::entity::Media;

pub struct MediaAggregate {
  pub media: Media,
  pub interpreters: Vec<Artist>,
  pub composers: Vec<Artist>,
  pub sources: Vec<Source>,
}
