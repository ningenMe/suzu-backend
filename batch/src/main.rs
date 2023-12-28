use std::{thread::sleep, time::Duration};

use infra::{hatena_site_repository, BlogDto, mysql_repository::insert};

extern crate infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("This is batch project");

    let mut blog_dto_list = Vec::<BlogDto>::new();
    for page in 1..20 {
        let mut tmp_list = hatena_site_repository::get_blog_dto_list(page).await.expect("hatena error");
        blog_dto_list.append(&mut tmp_list);
    }

    for blog_dto in blog_dto_list {
        println!("{:?}", blog_dto);
        let _ = insert(blog_dto).await;
        sleep(Duration::from_millis(500));
    }
    Ok(())
}
