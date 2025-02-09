FROM alpine:3.19

COPY ./../target/x86_64-unknown-linux-musl/release/batch /app/batch

CMD ["/app/batch", "--hatena", "--qiita", "--ameba", "--sizu", "--zenn"]
