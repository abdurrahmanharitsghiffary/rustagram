
# Rustagram

Rustagram is an Instagram Clone API built with Rust, designed to replicate core functionalities of Instagram, including user authentication, photo uploads, and social interactions.

<!-- ## Features

- User Authentication: Secure user registration and login mechanisms.
- Photo Uploads: Support for uploading and managing photos.
- Social Interactions: Features such as liking and commenting on posts. -->

## Getting Started

Follow these instructions to set up the project locally for development and testing purposes.

## Prerequisites

Ensure you have the following installed:

- Rust: Install Rust from the official website.
- Docker: Install Docker from the official website.

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/abdurrahmanharitsghiffary/rustagram.git
    cd rustagram
    ```

2. **Set up environment variables:**

    Copy the sample environment file and configure it as needed:

    ```bash
    cp .env.sample .env
    ```

3. **Build and Run the Docker Containers:**

   To start the application using Docker, build the containers and run them in detached mode:

   ```bash
   docker-compose up -d --build
    ```

## Usage

After completing the installation steps, the API should be accessible at <http://localhost:8000>.

## Running the Tests

To execute the automated tests:

```bash
cargo test
```

## Deployment

For deployment in a production environment, consider using orchestration tools like Kubernetes and setting up a reverse proxy with Nginx.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
