use chrono::{NaiveDateTime, Utc};
use serde_derive::Serialize;
use uuid::Uuid;
use diesel::{Insertable, Queryable};
use crate::schema::tweets;

#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = tweets)]
pub struct Tweets {
    pub id: Uuid,
    pub created_at: NaiveDateTime, // naive = ingenu => ISO 8601 - sin timezone
    pub message: String,
}

impl Tweets {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}
