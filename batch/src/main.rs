use std::{thread::sleep, time::Duration};
use clap::Parser;

use infra::{hatena_site_repository, BlogDto, mysql_repository::insert, qiita_site_repository, ameba_site_repository};

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
        //20 x 35 = 700 blog
        for page in 1..10 {
            let mut tmp_list = hatena_site_repository::get_blog_dto_list(page).await.expect("hatena error");
            println!("hatena blog: page={}, size={}", page, tmp_list.len());
            blog_dto_list.append(&mut tmp_list);
        }    
    }
    if args.qiita {
        //5 x 100 = 500 blog
        for page in 1..6 {
            let mut tmp_list = qiita_site_repository::get_blog_dto_list(page).await.expect("qiita error");
            println!("qiita blog: page={}, size={}", page, tmp_list.len());
            blog_dto_list.append(&mut tmp_list);
        }    
    }
    if args.ameba {
        //5 x 20 = 100 blog
        for page in 1..6 {
            let mut tmp_list = ameba_site_repository::get_blog_dto_list(page).await.expect("ameba error");
            println!("ameba blog: page={}, size={}", page, tmp_list.len());
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
