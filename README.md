# suzu-backend

```shell
# build
cargo build
# run
DATABASE_URL=mysql://root:password@127.0.0.1:3306/blog cargo run -p api
DATABASE_URL=mysql://root:password@127.0.0.1:3306/blog cargo run -p batch
```

## client generate
```shell
TS_OUT_PATH="./api/client"
protoc \
--js_out="import_style=commonjs,binary:${TS_OUT_PATH}" \
--grpc-web_out="import_style=typescript,mode=grpcwebtext:${TS_OUT_PATH}" \
./api/proto/*.proto
```

## api call
```shell
grpcurl -plaintext -import-path ./api/proto -proto suzu.proto -d '{}' '[::]:50051' suzu.BlogService/GetBlog
```