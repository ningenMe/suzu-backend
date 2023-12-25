use tonic::transport::Server;

use crate::controller::blog_controller::MyBlogService;
use crate::controller::blog_controller::suzu::blog_service_server::BlogServiceServer;

mod controller;

extern crate infra;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let blog_service = MyBlogService::default();

    Server::builder()
        .add_service(BlogServiceServer::new(blog_service))
        .serve(addr)
        .await?;

    Ok(())
}