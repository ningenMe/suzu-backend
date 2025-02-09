use std::{env, thread::sleep};
use chrono::NaiveDateTime;
use std::time::Duration;
use once_cell::sync::Lazy;
use reqwest;
use serde::Deserialize;

use crate::BlogDto;

static SIZU_ACCESS_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("SIZU_ACCESS_TOKEN").expect("env variable is not found")
});

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SizuPostsDto {
    pub slug: String,
    pub title: String,
    pub created_at: String,
}


#[derive(Deserialize, Debug)]
struct SizuPostsResponseDto {
    pub posts: Vec<SizuPostsDto>,
}

pub async fn get_blog_dto_list(page: i32) -> Result<Vec<BlogDto>,reqwest::Error> {

    let url = format!("https://sizu.me/api/v1/posts?page={}", page);
    let authorization = format!("Bearer {}", *SIZU_ACCESS_TOKEN);
    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .header(reqwest::header::AUTHORIZATION, authorization)
    .send()
    .await?
    .text()
    .await?;

    let sizu_blog_dto_list  = serde_json::from_str::<SizuPostsResponseDto>(&response).unwrap();
    
    let list = sizu_blog_dto_list.posts
    .iter()
    .map(|sizu_blog_dto| {
        return BlogDto{
            blog_url: format!("https://sizu.me/ningenme/posts/{}", sizu_blog_dto.slug),
            posted_at: NaiveDateTime::parse_from_str(&sizu_blog_dto.created_at, "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap(),
            blog_type: "SIZU".to_string(),
            blog_title: sizu_blog_dto.title.to_owned(),
        };
    })
    .collect();

    sleep(Duration::from_millis(500));
    return Ok(list);
}
