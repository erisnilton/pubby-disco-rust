use std::{collections::HashSet, iter};

use sqlx::Postgres;

use crate::{
  domain::{
    self,
    album::{AlbumEntity, AlbumRepository},
  },
  shared::vo::UUID4,
};

pub struct SqlxAlbumRepository {
  db: sqlx::Pool<Postgres>,
}

impl SqlxAlbumRepository {
  pub fn new(db: sqlx::Pool<Postgres>) -> Self {
    Self { db }
  }
}

impl AlbumRepository for SqlxAlbumRepository {
  async fn create(
    &mut self,
    album: crate::domain::album::AlbumEntity,
  ) -> Result<crate::domain::album::AlbumEntity, crate::domain::album::AlbumRepositoryError> {
    let mut trx = self
      .db
      .begin()
      .await
      .map_err(|error| domain::album::AlbumRepositoryError::DatabaseError(error.to_string()))?;
    sqlx::query!(
      r#"
      INSERT INTO album (id, name, cover, release_date, parental_rating)
      VALUES ($1, $2, $3, $4, $5)
      "#,
      Into::<uuid::Uuid>::into(album.id.clone()),
      album.name,
      album.cover,
      album.release_date,
      album.parental_rating.map(i16::from)
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    for artist_id in album.artist_ids.iter() {
      sqlx::query!(
        r#"
        INSERT INTO album_artist (album_id, artist_id)
        VALUES ($1, $2)
        "#,
        Into::<uuid::Uuid>::into(album.id.clone()),
        Into::<uuid::Uuid>::into(artist_id.clone())
      )
      .execute(&mut *trx)
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;
    }

    trx
      .commit()
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    Ok(album)
  }

  async fn update(
    &mut self,
    album: crate::domain::album::AlbumEntity,
  ) -> Result<crate::domain::album::AlbumEntity, crate::domain::album::AlbumRepositoryError> {
    let mut trx = self
      .db
      .begin()
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    sqlx::query!(
      r#"
      UPDATE album
      SET name = $2, cover = $3, release_date = $4, parental_rating = $5
      WHERE id = $1
      "#,
      Into::<uuid::Uuid>::into(album.id.clone()),
      album.name,
      album.cover,
      album.release_date,
      album.parental_rating.map(i16::from)
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    let old_artist = {
      let result = sqlx::query!(
        r#"
          SELECT "artist_id" FROM "album_artist" WHERE "album_id" = $1 
        "#,
        Into::<uuid::Uuid>::into(album.id.clone())
      )
      .fetch_all(&mut *trx)
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

      result
        .into_iter()
        .map(|value| UUID4::new(value.artist_id).unwrap())
        .collect::<HashSet<UUID4>>()
    };

    let (to_delete_artist, to_insert_artist) = {
      let mut to_delete: HashSet<uuid::Uuid> = HashSet::new();
      let mut to_insert: HashSet<uuid::Uuid> = HashSet::new();
      let all_artist = album
        .artist_ids
        .clone()
        .into_iter()
        .chain(old_artist.clone())
        .collect::<HashSet<UUID4>>();

      for artist_id in all_artist {
        if !old_artist.contains(&artist_id) {
          to_insert.insert(artist_id.into());
          continue;
        }
        if !album.artist_ids.contains(&artist_id) {
          to_delete.insert(artist_id.into());
          continue;
        }
      }

      (to_delete, to_insert)
    };

    // Exclui os artistas que não estão mais no album
    if !to_delete_artist.is_empty() {
      let bindings = iter::repeat(())
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .take(to_delete_artist.len())
        .collect::<Vec<String>>()
        .join(",");
      let query = format!(
        r#"DELETE FROM "album_artist" WHERE "artist_id" IN ({})"#,
        bindings
      );
      print!("{}", query);

      let query = {
        let mut query = sqlx::query(&query);

        for value in to_delete_artist.into_iter() {
          query = query.bind(value);
        }

        query
      };

      query
        .execute(&mut *trx)
        .await
        .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;
    }

    // Insere os Novos artistas
    if !to_insert_artist.is_empty() {
      let bindings = iter::repeat(())
        .enumerate()
        .map(|(i, _)| format!("($1,${})", i + 2))
        .take(to_insert_artist.len())
        .collect::<Vec<String>>()
        .join(",");
      let sql = format!(
        r#"INSERT INTO "album_artist"("album_id", "artist_id") VALUES {}"#,
        bindings
      );

      let query = {
        let mut query = sqlx::query(&sql).bind(Into::<uuid::Uuid>::into(album.id.clone()));

        for value in to_insert_artist.into_iter() {
          query = query.bind(value);
        }

        query
      };

      query
        .execute(&mut *trx)
        .await
        .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;
    }

    trx
      .commit()
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    Ok(album)
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<crate::domain::album::AlbumEntity>, crate::domain::album::AlbumRepositoryError>
  {
    let result = sqlx::query!(
      r#"
      SELECT "id", "name", "cover", "release_date", "parental_rating", "created_at", "updated_at"
      FROM "album"
      WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    if result.is_none() {
      return Ok(None);
    }

    let result = result.unwrap();

    let artists_id = {
      let result = sqlx::query!(
        r#"
        SELECT "artist_id" FROM "album_artist" WHERE "album_id" = $1 
      "#,
        Into::<uuid::Uuid>::into(id.clone())
      )
      .fetch_all(&self.db)
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

      result
        .into_iter()
        .map(|value| UUID4::new(value.artist_id).unwrap())
        .collect::<HashSet<UUID4>>()
    };

    Ok(Some(AlbumEntity {
      id: UUID4::new(result.id).unwrap(),
      artist_ids: artists_id,
      cover: result.cover,
      name: result.name,
      parental_rating: result.parental_rating.map(|value| value as i8),
      release_date: result.release_date,
      created_at: result.created_at,
      updated_at: result.updated_at,
    }))
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::album::AlbumRepositoryError> {
    let mut trx = self
      .db
      .begin()
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    sqlx::query!(
      r#"
      DELETE FROM album_artist WHERE album_id = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    sqlx::query!(
      r#"
      DELETE FROM album WHERE id = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    trx
      .commit()
      .await
      .map_err(|err| domain::album::AlbumRepositoryError::DatabaseError(err.to_string()))?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use sqlx::Postgres;

  use crate::{
    domain::{
      album::{AlbumEntity, AlbumRepository},
      artists::{repository::ArtistRepository, Artist},
    },
    infra::sqlx::SqlxArtistRepository,
    shared::vo::{Slug, UUID4},
    AppState,
  };

  use super::SqlxAlbumRepository;

  #[tokio::test]
  async fn test_create_album() {
    // Load .env file
    dotenvy::dotenv().ok();
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
        DELETE FROM "album_artist" WHERE album_id = '09c227b6-7498-4f63-b17c-11b7fe4e9c77'
      "#
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
        DELETE FROM "album" WHERE id = '09c227b6-7498-4f63-b17c-11b7fe4e9c77'
      "#,
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
        DELETE FROM "artist" WHERE id = '8115d6e2-e15f-42dc-8858-edd305805a7d' 
      "#,
      )
      .execute(db)
      .await
      .unwrap();
    }
    let app_state = AppState::default().await;
    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());
    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let artist = Artist {
      id: UUID4::new("8115d6e2-e15f-42dc-8858-edd305805a7d").unwrap(),
      name: "test_artist".to_string(),
      slug: Slug::new("test-create-album-artist").unwrap(),
      ..Default::default()
    };

    artist_repository.create(&artist).await.unwrap();

    let album = AlbumEntity {
      id: UUID4::new("09c227b6-7498-4f63-b17c-11b7fe4e9c77").unwrap(),
      name: "test_album".to_string(),
      artist_ids: {
        let mut set = HashSet::new();
        set.insert(artist.id.clone());
        set
      },
      ..Default::default()
    };
    let result = album_repository.create(album.clone()).await.unwrap();
    let album_artis = sqlx::query!(r#"
        SELECT "album_id", "artist_id" FROM "album_artist" WHERE "album_id" = '09c227b6-7498-4f63-b17c-11b7fe4e9c77'
    "#).fetch_one(&app_state.db).await.expect("O Relacionamento entre album e artista não foi criado");

    cleanup(&app_state.db).await;
    assert_eq!(album_artis.artist_id, artist.id.into());
    assert_eq!(result, album);
  }

  #[tokio::test]
  async fn update_album_info() {
    // Load .env file
    dotenvy::dotenv().ok();
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
         DELETE FROM "album" WHERE id = '26e948df-1351-461c-b45d-2eb183e6d6fc'
       "#,
      )
      .execute(db)
      .await
      .unwrap();
    }

    let app_state = AppState::default().await;
    cleanup(&app_state.db).await;

    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let album = AlbumEntity {
      id: UUID4::new("26e948df-1351-461c-b45d-2eb183e6d6fc").unwrap(),
      name: "test_album".to_string(),
      ..Default::default()
    };
    let result = album_repository.create(album.clone()).await.unwrap();
    cleanup(&app_state.db).await;
    assert_eq!(result, album);
  }

  #[tokio::test]
  async fn update_album_set_artist() {
    // Load .env file
    dotenvy::dotenv().ok();
    const ALBUM_ID: &str = "3fb46a9c-26b2-44f4-972a-16f32b0c8b5c";
    const ARTIST_ID: &str = "f6378582-8a71-4928-8edd-87af40b66446";
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());
    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let mut album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_update_album_set_artist".to_string(),
      ..Default::default()
    };
    album_repository.create(album.clone()).await.unwrap();

    let artist = Artist {
      id: UUID4::new(ARTIST_ID).unwrap(),
      name: "test_artist".to_string(),
      slug: Slug::new("test-update-album-set-artist").unwrap(),
      ..Default::default()
    };

    artist_repository.create(&artist).await.unwrap();

    album.artist_ids.insert(artist.id.clone());

    album_repository.update(album.clone()).await.unwrap();

    let result = album_repository
      .find_by_id(&album.id)
      .await
      .unwrap()
      .unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result.artist_ids, album.artist_ids);
  }

  #[tokio::test]
  async fn update_album_remove_artist() {
    // Load .env file
    dotenvy::dotenv().ok();
    const ALBUM_ID: &str = "eb59a92e-5765-47b6-819d-68e1d30b996b";
    const ARTIST_ID: &str = "7d100457-6e92-498c-878c-bea6acb16f30";
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());
    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let artist = Artist {
      id: UUID4::new(ARTIST_ID).unwrap(),
      name: "test_artist".to_string(),
      slug: Slug::new("test-update-album-remove-artist").unwrap(),
      ..Default::default()
    };

    artist_repository.create(&artist).await.unwrap();

    let mut album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_album".to_string(),
      artist_ids: {
        let mut set = HashSet::new();
        set.insert(artist.id.clone());
        set
      },
      ..Default::default()
    };
    album_repository.create(album.clone()).await.unwrap();

    album.artist_ids.clear();

    album_repository.update(album.clone()).await.unwrap();

    let result = album_repository
      .find_by_id(&album.id)
      .await
      .unwrap()
      .unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result.artist_ids, album.artist_ids);
  }

  #[tokio::test]
  async fn update_album_replace_artist() {
    // Load .env file
    dotenvy::dotenv().ok();
    const ALBUM_ID: &str = "a4dc9195-e3a1-493e-9482-7734454c4cbd";
    const ARTIST_ID: &str = "7681467f-b7ef-4035-a49d-a70d243799d3";
    const ARTIST_ID_2: &str = "7c99ddf2-6334-49ab-bfc6-ca14e7ea7a2e";
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id in ($1, $2)
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap(),
        uuid::Uuid::parse_str(ARTIST_ID_2).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());
    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let artist = Artist {
      id: UUID4::new(ARTIST_ID).unwrap(),
      name: "test_artist".to_string(),
      slug: Slug::new("test-update-album-replace-artist").unwrap(),
      ..Default::default()
    };

    let artist_2 = Artist {
      id: UUID4::new(ARTIST_ID_2).unwrap(),
      name: "test_artist_2".to_string(),
      slug: Slug::new("test-update-album-replace-artist_2").unwrap(),
      ..Default::default()
    };

    artist_repository.create(&artist).await.unwrap();
    artist_repository.create(&artist_2).await.unwrap();

    let mut album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_album".to_string(),
      artist_ids: {
        let mut set = HashSet::new();
        set.insert(artist.id.clone());
        set
      },
      ..Default::default()
    };
    album_repository.create(album.clone()).await.unwrap();

    album.artist_ids.clear();
    album.artist_ids.insert(artist_2.id.clone());

    album_repository.update(album.clone()).await.unwrap();

    let result = album_repository
      .find_by_id(&album.id)
      .await
      .unwrap()
      .unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result.artist_ids, album.artist_ids);
  }

  #[tokio::test]
  async fn update_album_insert_artist() {
    // Load .env file
    dotenvy::dotenv().ok();
    const ALBUM_ID: &str = "50bde96f-b94b-4c87-9d57-9fd305030634";
    const ARTIST_ID: &str = "d1fd1329-47c6-49f6-8756-527b6980270b";
    const ARTIST_ID_2: &str = "fce8bc1a-397a-4126-ab97-1f8a55875b07";
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id in ($1, $2)
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap(),
        uuid::Uuid::parse_str(ARTIST_ID_2).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());
    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let artist = Artist {
      id: UUID4::new(ARTIST_ID).unwrap(),
      name: "test_artist".to_string(),
      slug: Slug::new("test-update-album-insert-artist").unwrap(),
      ..Default::default()
    };

    let artist_2 = Artist {
      id: UUID4::new(ARTIST_ID_2).unwrap(),
      name: "test_artist_2".to_string(),
      slug: Slug::new("test-update-album-insert-artist_2").unwrap(),
      ..Default::default()
    };

    artist_repository.create(&artist).await.unwrap();
    artist_repository.create(&artist_2).await.unwrap();

    let mut album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_album".to_string(),
      artist_ids: {
        let mut set = HashSet::new();
        set.insert(artist.id.clone());
        set
      },
      ..Default::default()
    };
    album_repository.create(album.clone()).await.unwrap();

    album.artist_ids.insert(artist_2.id.clone());

    album_repository.update(album.clone()).await.unwrap();

    let result = album_repository
      .find_by_id(&album.id)
      .await
      .unwrap()
      .unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result.artist_ids, album.artist_ids);
  }

  #[tokio::test]
  async fn update_album_remove_a_artist() {
    // Load .env file
    dotenvy::dotenv().ok();
    const ALBUM_ID: &str = "123324fd-a42a-441b-b633-5ec7b1b715f7";
    const ARTIST_ID: &str = "46e7054e-dc41-4d72-a833-3524e748dd7e";
    const ARTIST_ID_2: &str = "6393a977-a557-4165-8d3d-abf34918bf98";
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id in ($1, $2)
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap(),
        uuid::Uuid::parse_str(ARTIST_ID_2).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());
    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let artist = Artist {
      id: UUID4::new(ARTIST_ID).unwrap(),
      name: "test_artist".to_string(),
      slug: Slug::new("test-update-album-remove-a-artist").unwrap(),
      ..Default::default()
    };

    let artist_2 = Artist {
      id: UUID4::new(ARTIST_ID_2).unwrap(),
      name: "test_artist_2".to_string(),
      slug: Slug::new("test-update-album-remove-a-artist_2").unwrap(),
      ..Default::default()
    };

    artist_repository.create(&artist).await.unwrap();
    artist_repository.create(&artist_2).await.unwrap();

    let mut album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_album".to_string(),
      artist_ids: {
        let mut set = HashSet::new();
        set.insert(artist.id.clone());
        set.insert(artist_2.id.clone());
        set
      },
      ..Default::default()
    };
    album_repository.create(album.clone()).await.unwrap();

    album.artist_ids.remove(&artist_2.id);

    album_repository.update(album.clone()).await.unwrap();

    let result = album_repository
      .find_by_id(&album.id)
      .await
      .unwrap()
      .unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result.artist_ids, album.artist_ids);
  }

  #[tokio::test]
  async fn delete_album() {
    const ALBUM_ID: &str = "a3f3dbb0-ae1f-40f3-a98d-7efc0c46bf05";

    // Load .env file
    dotenvy::dotenv().ok();

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_album".to_string(),
      ..Default::default()
    };

    album_repository.create(album.clone()).await.unwrap();

    album_repository.delete_by_id(&album.id).await.unwrap();

    let result = album_repository.find_by_id(&album.id).await.unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result, None);
  }

  #[tokio::test]
  async fn find_album_by_id() {
    const ALBUM_ID: &str = "d91f0a0f-ecbd-4673-b18a-ad38076a7992";

    // Load .env file
    dotenvy::dotenv().ok();

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .unwrap();
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut album_repository = SqlxAlbumRepository::new(app_state.db.clone());

    let album = AlbumEntity {
      id: UUID4::new(ALBUM_ID).unwrap(),
      name: "test_album".to_string(),
      ..Default::default()
    };

    album_repository.create(album.clone()).await.unwrap();

    let result = album_repository
      .find_by_id(&album.id)
      .await
      .unwrap()
      .unwrap();

    cleanup(&app_state.db).await;

    assert_eq!(result, album);
  }
}
