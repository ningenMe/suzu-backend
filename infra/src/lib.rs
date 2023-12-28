use chrono::NaiveDateTime;

pub mod mysql_repository;
pub mod hatena_site_repository;

#[derive(Debug)]
pub struct BlogDto {
    pub blog_url: String,
    pub posted_at: NaiveDateTime,
    pub blog_type: String,
    pub blog_title: String,
}