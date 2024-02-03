extern crate dotenv;

// IMPORT MODS
mod commons;
mod core;
mod modules;
mod schema;

// Crates
use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

// Modules
use modules::{
    likes::{
        self,
        services::{delete_like, post_likes, get_like},
    },
    tweets::{
        self,
        services::{get_tweets, get_tweets_by_id, post_tweets},
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("Database connection failed.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    //let pool = Pool::builder().build(manager).expect("Could not create conections pool.");

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create conections pool.");

    tweets::services::test_tweet_service();
    likes::services::test_likes_service();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_tweets)
            .service(post_tweets)
            .service(get_tweets_by_id)
            .service(get_like)
            .service(post_likes)
            .service(delete_like)
    })
    .bind(format!(
        "{}:{}",
        env::var("SERVER_HOST").unwrap(),
        env::var("SERVER_PORT").unwrap()
    ))?
    .run()
    .await
}
