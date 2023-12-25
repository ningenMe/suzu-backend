use sqlx::Pool;
use sqlx::MySql;
use chrono::naive::NaiveDateTime;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

use once_cell::sync::Lazy;

static NINGENME_MYSQL_MASTER_USER_USERNAME: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_MASTER_USER_USERNAME").expect("env variable is not found"));
static NINGENME_MYSQL_MASTER_USER_PASSWORD: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_MASTER_USER_PASSWORD").expect("env variable is not found"));
static NINGENME_MYSQL_HOST: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_HOST").expect("env variable is not found"));
static NINGENME_MYSQL_PORT: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_PORT").expect("env variable is not found"));
static DATABASE_URL: Lazy<String> = Lazy::new(|| 
    format!("mysql://{}:{}@{}:{}/blog", 
    *NINGENME_MYSQL_MASTER_USER_USERNAME, 
    *NINGENME_MYSQL_MASTER_USER_PASSWORD,
    *NINGENME_MYSQL_HOST,
    *NINGENME_MYSQL_PORT
));

static POOL: Lazy<Pool<MySql>> = Lazy::new(|| {
    return futures::executor::block_on(async {
        MySqlPoolOptions::new().max_connections(5).connect(
            &DATABASE_URL
        ).await.expect("database is not connected")
    });
});

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
