use std::str::FromStr;

// CRATES
use crate::core::likes_entity::Likes;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

// COMMONS
use crate::commons::constants::CONTENT_TYPE_APPLICATION_JSON;

// ENTITY SCHEMA CRATE
use crate::schema::likes;
use crate::schema::likes::dsl::*;

pub fn test_likes_service() {
    println!("Likes service");
}

fn set_connection(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    let conn: diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect("Could not get database connection");
    conn
}

fn filter_likes(uuid: &Uuid, pool: &Data<Pool<ConnectionManager<PgConnection>>>) -> Vec<Likes> {
    let mut conn = set_connection(&pool);

    let filtered_likes: Result<Vec<Likes>, _> = likes
        .filter(likes::tweet_id.eq(&uuid))
        .load::<Likes>(&mut conn);

    match filtered_likes {
        Ok(rows) => rows,
        Err(_) => vec![],
    }
}

#[post("/tweets/{id}/likes")]
pub async fn post_likes(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let mut conn = set_connection(&pool);

    let tw_id: &String = &path.into_inner().0;
    let new_like: Likes = Likes::new(Uuid::from_str(tw_id).unwrap());

    diesel::insert_into(likes)
        .values(&new_like)
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_APPLICATION_JSON)
        .json(&new_like)
}

#[get("/tweets/{id}/likes")]
pub async fn get_like(
    path: Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let tw_id: &String = &path.into_inner();
    let uuid: Uuid = Uuid::from_str(&tw_id).expect("Invalid UUID format");

    let results: Vec<Likes> = filter_likes(&uuid, &pool);

    HttpResponse::Created()
        .content_type(CONTENT_TYPE_APPLICATION_JSON)
        .json(results.len())
}

#[delete("/tweets/{id}/likes")]
pub async fn delete_like(
    path: Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let tw_id: &String = &path.into_inner();
    let uuid: Uuid = Uuid::from_str(&tw_id).expect("Invalid UUID format");

    let control: Vec<Likes> = filter_likes(&uuid, &pool);
    if control.is_empty() {
        return HttpResponse::NoContent()
            .content_type(CONTENT_TYPE_APPLICATION_JSON)
            .await
            .unwrap();
    }

    let like_to_delete = control.first().unwrap();
    println!("{:?}", like_to_delete.id);
    let response = diesel::delete(likes.filter(id.eq(like_to_delete.id))).execute(&mut set_connection(&pool));

    match response {
        Ok(deleted_rows) => {
            if deleted_rows > 0 {
                println!("Like deleted successfully");
            } else {
                println!("No like deleted");
            }
        }
        Err(err) => {
            println!("Error deleting like: {}", err);
        }
    }

    HttpResponse::NoContent()
        .content_type(CONTENT_TYPE_APPLICATION_JSON)
        .await
        .unwrap()
}
