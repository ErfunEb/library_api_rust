use crate::database::schema::genres;
use crate::database::schema::genres::dsl::genres as genre_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset)]
#[table_name = "genres"]
pub struct Genre {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Genre {
    pub fn list(connection: &SqliteConnection) -> Vec<Self> {
        genre_dsl
            .load::<Genre>(connection)
            .expect("Error loading genres")
    }

    pub fn create(connection: &SqliteConnection, title: String) -> Option<Genre> {
        let new_id = Uuid::new_v4().to_string();
        let new_genre = Genre {
            id: new_id.clone(),
            title,
            parent_id: None,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        diesel::insert_into(genre_dsl)
            .values(&new_genre)
            .execute(connection)
            .expect("Error saving new user");
        Self::find(&new_id, connection)
    }

    pub fn update(
        connection: &SqliteConnection,
        id: String,
        credentials: (String, Option<String>),
    ) -> Option<Genre> {
        let (title, parent_id) = credentials;

        diesel::update(genre_dsl.find(&id))
            .set((genres::title.eq(title), genres::parent_id.eq(parent_id)))
            .execute(connection)
            .expect("Error updating genre");
        Self::find(&id, connection)
    }

    pub fn delete(connection: &SqliteConnection, id: String) {
        diesel::delete(genre_dsl.find(&id))
            .execute(connection)
            .expect("Error deleting genre");
    }

    pub fn find(id: &str, connection: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = genre_dsl.find(id).get_result::<Genre>(connection) {
            Some(record)
        } else {
            None
        }
    }
}
