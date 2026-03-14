### Commands for testing server 

-  `curl -I http://localhost:3000` -> Only return headers
    ```rust
      HTTP/1.1 200 OK
      content-length: 0
      date: Sat, 14 Mar 2026 14:29:11 GMT
    ```

- `curl -s http://localhost:3000` -> Only return body
- `curl http://localhost:3000` -> Return headers and body
- `curl -v http://localhost:3000` -> Return verbose output
- `curl -X POST http://localhost:3000` -> Send a POST request


### Important commands for setup project
- `cargo new project_name` -> Create a new Rust project
- `cargo add axum --features="ws"` -> Add axum as a dependency with features
