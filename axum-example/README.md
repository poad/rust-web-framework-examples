# axum-example

The example for axum-web.

## Run

```shell
cargo run
```

### with Docker

```shell
docker stop axum; docker rm axum; docker buildx build --platform linux/amd64,linux/arm64 --rm --tag axum . && docker run -t -d --name axum -p 3000:3000 axum
```
