start:
	cargo run

test:
	cargo test

build:
	cargo build

watch:
	cargo watch -s 'cargo run'

docker@run:
	docker run --name rustagram_local --env-file .env -p 8080:8080 rustagram

docker@compose:
	docker compose up -d --env-file .env

sqlx@migrate:
	sqlx migrate run