# Jiaxin-Week2-Mini-Demo
This mini demo is aimed at teaching you to build an API for autofilling letters and returning possible completec words based on Rust. Users only need to type at least one initial letter, it will automatically fill in the rest and list 7 related potential words, so that they can choose the one they most want to express. This API can be applied in the field of NLP (Natural language processing).

## Run it on Terminal
1. Type: `cargo build --release`
2. Check API: `./target/release/autocomplete-api-poc`

## Run it in Docker
1. Type: `docker build -f Dockerfile . -t rust-autocomplete:latest `
2. See API: `docker run -d -p 0.0.0.0:3030:3030  --name rust-autocomplete-api rust-autocomplete`

## Show some Curl examples: 
Curl, short for "Client for URLs", is a command line tool for transferring data using various protocols.
1. eg for "sug"

```bash
curl  --location --request POST 'localhost:8080/autocomplete/v1' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "term": "pie"
}'
```
```bash
>>["pieces", "pies", "piece", "pierogies", "pierogi", "pierre", "pierce", "piecrust", "pierogie", "pierogis"]
```

2. eg for "pi"

```bash
curl  --location --request POST 'localhost:8080/autocomplete/v1' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "term": "pi"
}'
```
```bash
>>["pie", "pineapple", "pizza", "pieces", "pine", "pita", "pickle", "pinto", "pink", "pickled"]
```