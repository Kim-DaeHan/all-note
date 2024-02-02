# Actix REST API Template

This is a template project for building a REST API using the Actix web framework in Rust.

## Features

- Actix web framework for building asynchronous web applications
- RESTful API structure
- CRUD operations example
- JSON serialization and deserialization with serde
- Error handling with custom error types

## Getting Started

1. Clone this repository:

   ```bash
   git clone https://github.com/Kim-DaeHan/actix-rest-template.git
   cd actix-rest-template
   ```

2. Build and run the project:

   ```bash
   cargo build
   cargo run
   ```

   The server will start at http://localhost:8080.

3. Explore the API using your preferred API client or tools like cURL, Postman, or curl:

- Get all post:

  ```bash
  curl http://localhost:8080/api/posts
  ```

- Get a specific post:

  ```bash
  curl http://localhost:8080/api/posts/{id}
  ```

- Create a new post:

  ```bash
  curl -X POST -H "Content-Type: application/json" -d '{"title": "New Post", "body": "Post Body", "published": true}' http://localhost:8080/api/posts
  ```

- Update an post:

  ```bash
  curl -X PUT  -H "Content-Type: application/json" -d '{"id": "uuid", "title": "Update Post", "body": "Update Body", "published": true}' http://localhost:8080/api/posts
  ```

- Delete an item:

  ```bash
  curl -X DELETE http://localhost:8080/api/posts/{id}
  ```

## Docker start

1. docker build:

   ```bash
   docker build -t actix_rest_template .
   ```

2. docker run:

   ```bash
   docker run -p 8080:8080 actix_rest_template
   ```
