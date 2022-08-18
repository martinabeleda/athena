# Athena
A rust service for storing books

## Development

To format the code, run:

```shell
cargo fmt
```

To start the server locally, run:

```shell
cargo run
```

## Docker

To build the image:

```shell
make build
```

To run the server in docker

```shell
make run
```

## Usage

### Create an author

```shell
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"id": 1, "first_name": "George", "last_name": "Orwell"}' \
  http://localhost:8080/athena/v1/author | jq
```

### Get a list of authors

```shell
curl -s localhost:8080/athena/v1/authors | jq
```

### Get an author by ID

```shell
curl -s localhost:8080/athena/v1/author/1 | jq
```
