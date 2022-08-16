# Athena
A rust service for storing books

## Development

To format the code, run:

```shell
cargo fmt
```

To start the server, run:

```shell
cargo run
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

```
