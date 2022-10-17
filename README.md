# Rust SPA server
A web server serve a Vue Single Page Application written in rust language

## Unit testing and integration testing

By default, output from print statements (e.g. println!, print!) will be eaten (not printed to stdout) by the Rust test harness.
To see the output from print statements, run the tests with the nocapture flag.
```
$ cargo test -- --nocapture

```

https://medium.com/intelliconnect-engineering/using-axum-framework-to-create-rest-api-part-1-7d434d2c5de4
