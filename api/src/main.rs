use tonic::transport::Server;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use sqlx::Pool;
use sqlx::MySql;

use crate::controller::blog_controller::MyBlogService;
use crate::controller::blog_controller::suzu::blog_service_server::BlogServiceServer;

mod controller;

extern crate infra;
use once_cell::sync::Lazy;

static DATABASE_URL: Lazy<String> = Lazy::new(|| env::var("DATABASE_URL").expect("database url is not found"));
static POOL: Lazy<Pool<MySql>> = Lazy::new(|| {
    return futures::executor::block_on(async {
        MySqlPoolOptions::new().max_connections(5).connect(&DATABASE_URL).await.expect("database is not connected")
    });
}
);


// MySqlPoolOptions::new()
// .max_connections(5)
// .connect(&DATABASE_URL).await?;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let blog_service = MyBlogService::default();

    Server::builder()
        .add_service(BlogServiceServer::new(blog_service))
        .serve(addr)
        .await?;

    Ok(())
}