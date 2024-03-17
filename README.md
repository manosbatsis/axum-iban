<h1>axum-iban</h1>

[![CI](https://github.com/manosbatsis/axum-iban/actions/workflows/ci.yml/badge.svg)](https://github.com/manosbatsis/axum-iban/actions/workflows/ci.yml)

A simple REST API to validate [IBANs](https://en.wikipedia.org/wiki/International_Bank_Account_Number) and get information about them. Uses
[axum](https://github.com/tokio-rs/axum) and [iban_validate](https://github.com/ThomasdenH/iban_validate).
Tests use [axum-test](https://github.com/JosephLenton/axum-test).

<h2>Table of Contents</h2>

<!-- TOC -->
  * [Build](#build)
    * [Basics](#basics)
    * [Binary](#binary)
  * [Run](#run)
    * [Start Server with Cargo](#start-server-with-cargo)
    * [Start Server with Docker](#start-server-with-docker)
  * [Test](#test)
    * [OpenAPI Documentation](#openapi-documentation)
<!-- TOC -->

## Build

### Basics
Assuming you have Rust instaalled, checkout the repo, bavigate to the project's root folder and run:

```shell
cargo build
```
Run tests:

```shell
cargo test
```

### Binary

Test the binary:

```shell
target/release/axum-iban -h
```

which should display:

```shell
Axum IBAN REST API

Usage: axum-iban [OPTIONS]

Options:
--host <IP>      Optional host IP to listen to (for example "0.0.0.0")
-l, --log <LEVEL>    Log level to use [possible values: trace, debug, info, warn, error]
-p, --port <NUMBER>  Optional port number to use (default is 3000)
-v, --version        Print version info and exit
-h, --help           Print help (see more with '--help')
```

## Run

### Start Server with Cargo

Run server locally:

```shell
cargo run --release
```

Specify log level:

```shell
cargo run --release -- --log error

```

Log level from env:

```shell
RUST_LOG=debug cargo run --release
```

### Start Server with Docker

Build Docker image and run container:

```shell
./run.sh
```

## Test

Start the server first and then in another terminal (tab):

```shell
./test-routes.sh
```

Or manually:

```shell
curl -s http://127.0.0.1:3000/info/version | jq .

curl -s http://127.0.0.1:3000/info/healthcheck | jq .

curl -s http://127.0.0.1:3000/iban/DE44500105175407324931 | jq .
```

### OpenAPI Documentation

Uses [utoipa](https://github.com/juhaku/utoipa) to generate OpenAPI documentation and UIs.

Swagger UI is available at `/doc`, Redoc at `/redoc`, and RapiDoc at `/rapidoc`.

The raw JSON can be seen from `/api-docs/openapi.json`.
