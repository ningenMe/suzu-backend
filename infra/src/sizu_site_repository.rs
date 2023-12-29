use std::thread::sleep;
use std::time::Duration;

use chrono::NaiveDateTime;
use reqwest;
use scraper::Html;
use scraper::Selector;

use crate::BlogDto;


pub async fn get_blog_dto_list() -> Result<Vec<BlogDto>,reqwest::Error> {
    todo!("記事をもうちょっと書いたらスポンサー登録する https://sizu.me/sponsors/purchase");
    let mut list = Vec::new();
    return Ok(list);
}
