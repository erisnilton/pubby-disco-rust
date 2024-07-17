mod create;
mod find_by_slug;

pub use create::{CreateArtistDto, create_artist};
pub use find_by_slug::find_artist_by_slug;