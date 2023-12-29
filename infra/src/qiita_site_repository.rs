use std::{env, thread::sleep};
use chrono::NaiveDateTime;
use std::time::Duration;
use reqwest;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::BlogDto;

static QIITA_ACCESS_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("QIITA_ACCESS_TOKEN").expect("env variable is not found")
});

#[derive(Deserialize, Debug)]
struct QiitaBlogDto {
    pub url: String,
    pub title: String,
    pub created_at: String,
}

pub async fn get_blog_dto_list(page: i32) -> Result<Vec<BlogDto>,reqwest::Error> {
    let url = format!("https://qiita.com/api/v2/authenticated_user/items?per_page=100&page={}", page);
    let authorization = format!("Bearer {}", *QIITA_ACCESS_TOKEN);
    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .header(reqwest::header::AUTHORIZATION, authorization)
    .send()
    .await?
    .text()
    .await?;

    let qiita_blog_dto_list  = serde_json::from_str::<Vec<QiitaBlogDto>>(&response).unwrap();
    
    let list = qiita_blog_dto_list
    .iter()
    .map(|qiita_blog_dto| {
        return BlogDto{
            blog_url: qiita_blog_dto.url.to_owned(),
            posted_at: NaiveDateTime::parse_from_str(&qiita_blog_dto.created_at, "%Y-%m-%dT%H:%M:%S%z").unwrap(),
            blog_type: "QIITA".to_string(),
            blog_title: qiita_blog_dto.title.to_owned(),
        };
    })
    .collect();

    sleep(Duration::from_millis(500));
    return Ok(list);
}
