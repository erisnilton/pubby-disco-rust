use crate::domain::artists::{
    repository::{ArtistRepository, ArtistRepositoryError},
    Artist,
};

pub struct CreateArtistDto {
    pub name: String,
    pub slug: String,
    pub country: String,
}

pub async fn create_artist(
    repository: &impl ArtistRepository,
    input: CreateArtistDto,
) -> Result<Artist, ArtistRepositoryError> {
    let artist = Artist::new(input.name, input.slug, input.country);

    repository.create(&artist).await?;

    Ok(artist)
}
