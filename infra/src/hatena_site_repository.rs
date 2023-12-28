use std::thread::sleep;
use std::time::Duration;

use chrono::NaiveDateTime;
use reqwest;
use scraper::Html;
use scraper::Selector;

use crate::BlogDto;


pub async fn get_list(page: i32) -> Result<Vec<BlogDto>,reqwest::Error> {
    let url = format!("https://ningenme.hatenablog.com/archive?page={}", page);
    println!("{}", url);
    let raw_html = reqwest::get(url).await?.text().await?;
    let html = Html::parse_document(&raw_html);

    let archive_selector = Selector::parse("div.archive-entry-header").unwrap();

    let mut list = Vec::new();
    for element in html.select(&archive_selector) {
        let time_selector = Selector::parse("time").unwrap();
        let posted_date = element.select(&time_selector).into_iter().find(|_| true).unwrap().attr("datetime").unwrap();

        let title_selector = Selector::parse("a.entry-title-link").unwrap();
        let title_element = element.select(&title_selector).into_iter().find(|_| true).unwrap();
        let blog_url = title_element.attr("href").unwrap();
        let blog_title = title_element.text().next().unwrap();

        list.push(BlogDto{
            blog_url: blog_url.to_owned(),
            posted_at: NaiveDateTime::parse_from_str(&format!("{} 00:00:00", &posted_date), "%Y-%m-%d %H:%M:%S").unwrap(),
            blog_type: "HATENA".to_string(),
            blog_title: blog_title.to_owned(),
        });
    }
    sleep(Duration::from_millis(500));
    return Ok(list);
}
