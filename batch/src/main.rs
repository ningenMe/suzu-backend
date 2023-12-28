use infra::{hatena_site_repository, BlogDto};

extern crate infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("This is batch project");

    let mut list = Vec::<BlogDto>::new();
    for page in 1..20 {
        let mut tmp_list = hatena_site_repository::get_list(page).await.expect("hatena error");
        list.append(&mut tmp_list);
    }
    println!("{:?}", list);
    Ok(())
}
