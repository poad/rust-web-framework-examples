# actix-example

The example for actix-web.

## Run

```shell
cargo run
```

### with Docker

```shell
docker stop actix; docker rm actix; docker buildx build --platform linux/amd64,linux/arm64 --rm --tag actix . && docker run -t -d --name actix -p 8080:8080 actix
```
