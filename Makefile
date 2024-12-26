# Application Targets
run:  # Start the application
	cargo run

test:  # Run tests
	cargo test

build:  # Build the application
	cargo build --release

watch:  # Watch for file changes and re-run the application
	cargo watch -s 'cargo run'

lint:	# Run the linter
	cargo clippy

lint_fix:	# Fixing the linter error
	cargo clippy --fix

# Docker Targets
docker_run:  # Run the Docker container
	docker run --name rustagram_local --env-file .env -p 8080:8080 rustagram

docker_build:  # Build the Docker image
	docker build -t rustagram .

docker_compose_up:  # Start Docker Compose services in detached mode
	docker compose up -d

docker_compose_down:  # Stop Docker Compose services
	docker compose down

# Database Migration Target
sqlx_migrate:  # Run database migrations
	sqlx migrate run
