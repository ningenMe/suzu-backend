# suzu-api

```shell
# build
cargo build
# run
cargo run -p api
cargo run -p batch
```

```shell
grpcurl -plaintext -import-path ./proto -proto suzu.proto -d '{}' '[::]:50051' suzu.BlogService/GetBlog
```