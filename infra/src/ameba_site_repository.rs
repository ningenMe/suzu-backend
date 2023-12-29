use std::thread::sleep;
use std::time::Duration;

use chrono::NaiveDateTime;
use reqwest;
use scraper::Html;
use scraper::Selector;
use serde::Deserialize;

use crate::BlogDto;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AmebaBlogDto {
    pub date_published: String
}

pub async fn get_blog_dto_list(page: i32) -> Result<Vec<BlogDto>,reqwest::Error> {
    let url = format!("https://ameblo.jp/ningenme/entrylist-{}.html", page);
    let raw_html = reqwest::get(url).await?.text().await?;
    let html = Html::parse_document(&raw_html);

    let mut list = Vec::new();

    let li_selector = Selector::parse("li.skin-borderQuiet").unwrap();
    for element in html.select(&li_selector) {
        let mut blog_title: String = "".to_string();
        let mut blog_url: String = "https://ameblo.jp".to_string();

        let h2_selector = Selector::parse("h2").unwrap();
        for tmp_element in element.select(&h2_selector) {
            blog_title = tmp_element.text().next().expect("error").to_owned();
            let a_selector = Selector::parse("a").unwrap();
            for tmp2_element in tmp_element.select(&a_selector) {
                blog_url += tmp2_element.attr("href").unwrap();
            }
        }
        match get_posted_at(blog_url.clone()).await? {
            Some(posted_at) => {
                list.push(
                    BlogDto {
                        blog_url,
                        posted_at,
                        blog_type: "AMEBA".to_string(),
                        blog_title,
                    }
                );
            },
            None => {},
        }
    }
    sleep(Duration::from_millis(500));
    return Ok(list);
}


async fn get_posted_at(blog_url: String) -> Result<Option<NaiveDateTime>, reqwest::Error>{
    let raw_html = reqwest::get(blog_url).await?.text().await?;
    let html = Html::parse_document(&raw_html);

    let script_selector = Selector::parse("script").unwrap();
    let mut posted_at_string = "".to_string();
    for element in html.select(&script_selector) {
        if element.attr("data-react-helmet") != Some("true"){
            continue;
        }
        let json = element.text().next().unwrap();
        let ameba_blog_dto = serde_json::from_str::<AmebaBlogDto>(&json).unwrap();
        posted_at_string = ameba_blog_dto.date_published;
    }
    let posted_at = match NaiveDateTime::parse_from_str(&posted_at_string, "%Y-%m-%dT%H:%M:%S%.f%z") {
        Ok(posted_at) => Some(posted_at),
        Err(e) => {
            println!("{}", e);
            None
        }
    };
    Ok(posted_at)
}

