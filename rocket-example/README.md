# rocket-example

The example for Rocket.

## Run

```shell
cargo run
```

### with Docker

```shell
docker stop rocket; docker rm rocket; docker buildx build --platform linux/amd64,linux/arm64 --rm --tag rocket . && docker run -t -d --name rocket -p 8000:8000 rocket
```
