
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use crate::controller::blog_controller::MyBlogService;
use crate::controller::blog_controller::suzu::blog_service_server::BlogServiceServer;

mod controller;

extern crate infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, infra::add_one(num));
    println!("This is api project");

    let addr = "[::1]:50051".parse()?;
    let blog_service = MyBlogService::default();

    Server::builder()
        .add_service(BlogServiceServer::new(blog_service))
        .serve(addr)
        .await?;

    Ok(())
}