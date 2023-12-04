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
    async fn get_blog(&self, request: Request<()>) -> Result<Response<GetBlogResponse>, Status> {
        Ok(Response::new(GetBlogResponse{ blog_list: vec![
            Blog {
                url: "hoge".to_string(),
                date: "".to_string(),
                blog_type: "".to_string(),
                blog_title: "".to_string(),
            },
            Blog {
                url: "fuga".to_string(),
                date: "".to_string(),
                blog_type: "".to_string(),
                blog_title: "".to_string(),
            }
        ]} ))
    }
}
