# Rust SPA server
A web server serve a Vue Single Page Application written in rust language

## Unit testing and integration testing

By default, output from print statements (e.g. println!, print!) will be eaten (not printed to stdout) by the Rust test harness.
To see the output from print statements, run the tests with the nocapture flag.
```
$ cargo test -- --nocapture

```

https://medium.com/intelliconnect-engineering/using-axum-framework-to-create-rest-api-part-1-7d434d2c5de4

## Docker
Build tag and run docker image.
```
$ docker build -t rust-spa-server .
$ docker tag rust-spa-server:latest rust-spa-server:0.0.1
$ docker run -dp 3000:3000 --rm --name rust-spa-server rust-spa-server:0.0.1

```

## Cross compile
```
docker build --platform linux/x86_64 -t rust-spa-server .
```

## Useful docker commands
```
// list docker images
$ docker images

// list all containers
$ docker ps -a

// stop container, also container name can be used here.
$ docker stop containerid

// delete container
$ docker rm containerid

// delete image
$ docker rmi imagename
```
