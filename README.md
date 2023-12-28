# suzu-backend

```shell
cd proto
buf lint
npx buf generate suzu
```

```shell
# build
docker-compose -f ./infra/tool/docker-compose.yaml up -d
cargo sqlx prepare --workspace --database-url "mysql://root:password@127.0.0.1:3306/blog"
# cargo sqlx prepare --workspace --database-url "mysql://${NINGENME_MYSQL_MASTER_USER_USERNAME}:${NINGENME_MYSQL_MASTER_USER_PASSWORD}@${NINGENME_MYSQL_HOST}:${NINGENME_MYSQL_PORT}/blog"
cargo build
# run
`aws ssm get-parameters-by-path --path "/" --region ap-northeast-1 --output text | awk '{print "export",$5"="$7}'`
cargo run -p api
cargo run -p batch
```

## client generate
```shell
TS_OUT_PATH="./api/proto/client"
protoc \
--js_out="import_style=commonjs,binary:${TS_OUT_PATH}" \
--grpc-web_out="import_style=typescript,mode=grpcwebtext:${TS_OUT_PATH}" \
./api/proto/suzu/v1/*.proto
```

## api call
```shell
# local
grpcurl -v -plaintext -import-path ./api/proto/suzu/v1 -proto suzu.proto -d '{}' 'localhost:50051' suzu.v1.BlogService/GetHealth
grpcurl -v -plaintext -import-path ./api/proto/suzu/v1 -proto suzu.proto -d '{}' 'localhost:50051' suzu.v1.BlogService/GetBlog
# production
grpcurl -v -plaintext -import-path ./api/proto/suzu/v1 -proto suzu.proto -d '{}' 'k8s-ec2.ningenme.net:30007' suzu.v1.BlogService/GetHealth
grpcurl -v -plaintext -import-path ./api/proto/suzu/v1 -proto suzu.proto -d '{}' 'k8s-ec2.ningenme.net:30007' suzu.v1.BlogService/GetBlog
```

## batch call
```shell
cargo run -p batch -- --hatena --qiita --dryrun
```