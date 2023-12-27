use infra::health;
use tonic::transport::Server;
use tonic::codegen::http::{Method, header};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{CorsLayer, AllowOrigin};

use crate::controller::blog_controller::suzu::blog_service_server::BlogServiceServer;
use crate::controller::blog_controller::MyBlogService;

mod controller;

extern crate infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let blog_service = MyBlogService::default();

    health().await;

    let cors_layer = CorsLayer::new()
    .allow_credentials(true)
        .allow_origin(AllowOrigin::list([
            "https://ningenme.net".parse()?,
            "http://localhost:3000".parse()? 
        ]))
        .allow_methods([
            Method::GET, 
            Method::POST,
            Method::OPTIONS
        ])
        .allow_headers([
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
            "x-user-agent".parse()?,
            "x-grpc-web".parse()?,        
        ]);
    Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(GrpcWebLayer::new())
        .add_service(BlogServiceServer::new(blog_service))
        .serve(addr)
        .await?;

    Ok(())
}
