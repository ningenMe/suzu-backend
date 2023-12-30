use std::thread::sleep;
use chrono::NaiveDateTime;
use std::time::Duration;
use reqwest;
use serde::Deserialize;

use crate::BlogDto;

#[derive(Deserialize, Debug)]
struct ZennScrapDto {
    pub slug: String,
    pub title: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
struct ZennScrapApiDto {
    pub scraps: Vec<ZennScrapDto>,
}

static SCRAP_URL_PREFIX: &str = "https://zenn.dev/ningenme/scraps";

pub async fn get_blog_dto_list() -> Result<Vec<BlogDto>,reqwest::Error> {
    let url = "https://zenn.dev/api/scraps?username=ningenme&count=100".to_string();
    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .send()
    .await?
    .text()
    .await?;

    let zenn_scrap_api_dto  = serde_json::from_str::<ZennScrapApiDto>(&response).unwrap();
    
    let list = zenn_scrap_api_dto.scraps
    .iter()
    .map(|zenn_scrap_dto| {
        return BlogDto{
            blog_url: format!("{}/{}", SCRAP_URL_PREFIX, zenn_scrap_dto.slug),
            posted_at: NaiveDateTime::parse_from_str(&zenn_scrap_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%z").unwrap(),
            blog_type: "ZENN".to_string(),
            blog_title: zenn_scrap_dto.title.to_owned(),
        };
    })
    .collect();

    sleep(Duration::from_millis(500));
    return Ok(list);
}
