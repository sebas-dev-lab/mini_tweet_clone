// CRATES
use actix_web::web::{self, Data, Path};
use actix_web::{get, post, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

// ENTITY SCHEMA CRATE
use crate::schema::tweets;
use crate::schema::tweets::dsl::*;

// COMMONS
use crate::commons::constants::CONTENT_TYPE_APPLICATION_JSON;
use crate::core::tweets_entity::Tweets;

// DTO
use crate::modules::tweets::dto::PostTweetDTO;

// SERVICES
pub fn test_tweet_service() {
    println!("Tweet service");
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    let mut conn = pool.get().expect("Could not get database connection");
    //let filtered_tweets: Vec<Tweets> = tweets.get_results(&mut conn).expect("Could not get tweets");

    let results = tweets.order(created_at.asc()).load::<Tweets>(&mut conn);

    let filtered_tweets: Vec<Tweets> = match results {
        Ok(t) => t,
        Err(_) => vec![],
    };

    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_APPLICATION_JSON)
        .json(filtered_tweets)
}

#[get("/tweets/{id}")]
pub async fn get_tweets_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Could not get database connection");

    let tweet_id: String = path.into_inner().0;
    let uuid: Uuid = Uuid::parse_str(&tweet_id).expect("Invalid UUID format");

    let tweet = tweets
        .find(uuid)
        .first::<Tweets>(&mut conn)
        .expect("Tweet not found");

    HttpResponse::Ok()
        .content_type(CONTENT_TYPE_APPLICATION_JSON)
        .json(tweet)
}

#[post("/tweets")]
pub async fn post_tweets(
    req_body: web::Json<PostTweetDTO>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    println!("{:?}", req_body);
    let tweet: Tweets = Tweets::new(req_body.message.clone());
    let mut conn = pool.get().expect("Could not get database connection");

    diesel::insert_into(tweets::table)
        .values(&tweet)
        .execute(&mut conn)
        .expect("Error inserting tweet");

    HttpResponse::Created()
        .content_type(CONTENT_TYPE_APPLICATION_JSON)
        .json(tweet.message)
}
