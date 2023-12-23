use sqlx::Pool;
use sqlx::MySql;
use chrono::naive::NaiveDateTime;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

use once_cell::sync::Lazy;

static DATABASE_URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").expect("database url is not found"));
static POOL: Lazy<Pool<MySql>> = Lazy::new(|| {
    return futures::executor::block_on(async {
        MySqlPoolOptions::new().max_connections(5).connect(&DATABASE_URL).await.expect("database is not connected")
    });
}
);

pub struct BlogDto {
    pub blog_url: String,
    pub posted_at: NaiveDateTime,
    pub blog_type: String,
    pub blog_title: String,
}

pub async fn select() -> Result<Vec<BlogDto>,sqlx::Error> {
    let blogs = sqlx::query_as!(
        BlogDto,
        "SELECT blog_url, posted_at, blog_type, blog_title FROM blog WHERE blog_type != 'DIARY' ORDER BY posted_at DESC"
    )
    .fetch_all(&*POOL).await?;
    return Ok(blogs);
}
