use std::{thread::sleep, time::Duration};
use clap::Parser;

use infra::{hatena_site_repository, BlogDto, mysql_repository::insert, qiita_site_repository};

extern crate infra;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// you want to fetch hatena ?
    #[arg(long, default_value_t = false)]
    hatena: bool,
    /// you want to fetch qiita ?
    #[arg(long, default_value_t = false)]
    qiita: bool,
    /// you want to fetch ameba ?
    #[arg(long, default_value_t = false)]
    ameba: bool,
    /// yow want to dry-run ?
    #[arg(long, default_value_t = false)]
    dryrun: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let args = Args::parse();

    let mut blog_dto_list = Vec::<BlogDto>::new();

    if args.hatena {
        for page in 1..20 {
            let mut tmp_list = hatena_site_repository::get_blog_dto_list(page).await.expect("hatena error");
            blog_dto_list.append(&mut tmp_list);
        }    
    }
    if args.qiita {
        for page in 1..5 {
            let mut tmp_list = qiita_site_repository::get_blog_dto_list(page).await.expect("qiita error");
            blog_dto_list.append(&mut tmp_list);
        }    
    }

    if !args.dryrun {
        for blog_dto in blog_dto_list {
            println!("{:?}", blog_dto);
            let _ = insert(blog_dto).await;
            sleep(Duration::from_millis(500));
        }    
    }
    Ok(())
}
