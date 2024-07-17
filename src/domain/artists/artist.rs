use chrono::Utc;

use crate::domain::shared::uuid_vo::UUID4;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: UUID4,
    pub slug: String,
    pub name: String,
    pub country: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Artist {
    pub fn new(name: String, slug: String, country: String) -> Self {
        Self {
            id: UUID4::generate(),
            slug,
            name,
            country,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Default for Artist {
    fn default() -> Self {
        Self {
            id: UUID4::generate(),
            slug: "".to_string(),
            name: "".to_string(),
            country: "".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
