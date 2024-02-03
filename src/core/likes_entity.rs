use crate::schema::likes;
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = likes)]
pub struct Likes {
    pub id: Uuid,
    pub created_at: NaiveDateTime, // naive = ingenu => ISO 8601 - sin timezone
    pub tweet_id: Uuid,
}

impl Likes {
    pub fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }
}
