use sqlx::Pool;
use sqlx::MySql;
use sqlx::Row;
use chrono::naive::NaiveDateTime;


pub struct BlogDto {
    pub blog_url: String,
    pub posted_at: NaiveDateTime,
    pub blog_type: String,
    pub blog_title: String,
}

pub async fn select(pool: &Pool<MySql>) -> Result<Vec<BlogDto>,sqlx::Error> {
    let query_blog_type = "HATENA";

    let blogs = 
    sqlx::query_as!(
        BlogDto,
        "SELECT blog_url, posted_at, blog_type, blog_title FROM blog WHERE blog_type = ?",
        query_blog_type
    )
    .fetch_all(pool).await?;

    // let blogs = sqlx::query_as!(
    //     BlogDto, 
    //     "SELECT blog_url, posted_at, blog_type, blog_title FROM blog WHERE blog_type = ?",
    //     blog_type
    // )
    // .fetch_all(pool) 
    // .await?;

    return Ok(blogs);
}
