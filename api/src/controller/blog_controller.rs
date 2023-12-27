use infra::select;
use tonic::{Request, Response, Status};

use suzu::blog_service_server::BlogService;
use suzu::{Blog, GetBlogResponse, GetHealthResponse};

pub mod suzu {
    tonic::include_proto!("suzu.v1");
}

#[derive(Debug, Default)]
pub struct MyBlogService {}

#[tonic::async_trait]
impl BlogService for MyBlogService {
    async fn get_blog(&self, _request: Request<()>) -> Result<Response<GetBlogResponse>, Status> {
        println!("get_blog request");
        let blog_dtos = select().await.expect("sql failed");
        let blogs = blog_dtos
            .iter()
            .map(|it| {
                return Blog {
                    url: it.blog_url.clone(),
                    date: it.posted_at.date().to_string(),
                    blog_type: it.blog_type.clone(),
                    blog_title: it.blog_title.clone(),
                };
            })
            .collect();
        Ok(Response::new(GetBlogResponse { blog_list: blogs }))
    }

    async fn get_health(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetHealthResponse>, Status> {
        println!("get_health request");
        Ok(Response::new(GetHealthResponse {
            text: "ok".to_string(),
        }))
    }
}
