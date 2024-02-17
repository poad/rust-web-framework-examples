# rocket-example

The example for Rocket.

## Run

```shell
cargo run
```

### with Docker

```shell
docker stop rocket; docker rm rocket; docker build --rm --tag rocket . && docker run -t -d --name rocket -p 8000:8000 rocket
```
