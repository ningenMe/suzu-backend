# suzu-backend

```shell
# build
cargo build
# run
DATABASE_URL=mysql://root:password@127.0.0.1:3306/blog cargo run -p api
DATABASE_URL=mysql://root:password@127.0.0.1:3306/blog cargo run -p batch
```

## api call
```shell
grpcurl -plaintext -import-path ./api/proto -proto suzu.proto -d '{}' '[::]:50051' suzu.BlogService/GetBlog
```