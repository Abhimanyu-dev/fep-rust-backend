use fred::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct State {
    pub pool: Pool<Postgres>,
    pub redis: Client,
    pub jwt_secret_key: [u8; 32],
}
impl State {
    pub async fn create() -> State {
        State {
            pool: configure_db().await,
            redis: get_redis_client().await,
            jwt_secret_key: b"abcdefghijklmnopqrstuvwxyzabcdef".to_owned(),
        }
    }
}
async fn get_redis_client() -> Client {
    let url = env::var("REDIS_URL").expect("REDIS_URL not found");
    let config = Config::from_url(&url).unwrap();
    let client = Client::new(config, None, None, None);
    client.init().await.expect("Connot connect to redis");
    client
}

async fn configure_db() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Cannot Connect to DB");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Cannot create db structure");
    pool
}
