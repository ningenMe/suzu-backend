use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use suzu::blog_service_server::{BlogService, BlogServiceServer};
use suzu::{Blog, GetBlogResponse};

pub mod suzu {
    tonic::include_proto!("suzu");
}

#[derive(Debug, Default)]
pub struct MyBlogService {}

#[tonic::async_trait]
impl BlogService for MyBlogService {
    type GetStream = ReceiverStream<Result<GetBlogResponse, Status>>;

    async fn get(&self, _request: Request<()>) -> Result<Response<Self::GetStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let response_vec:Vec<GetBlogResponse> = vec![
            GetBlogResponse{
                blog: None
            },
            GetBlogResponse{
                blog: None
            },
        ];

        println!("hoge1");

        tokio::spawn(async move {
            for response in &response_vec[..]{
                println!("hoge2");
                tx.send(Ok(response.clone())).await.unwrap();
            }
        });
        println!("hoge3");
        Ok(Response::new( ReceiverStream::new(rx)))
    }
}


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