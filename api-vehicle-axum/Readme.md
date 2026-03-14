### Commands for testing server 
---
#### Only return headers
```bash
curl -I http://localhost:3000
```
> ```text
> //Respone Looks like
>
> HTTP/1.1 200 OK
> content-length: 0
> date: Sat, 14 Mar 2026 14:29:11 GMT
> ```

#### Only return body
```bash
curl -s http://localhost:3000` 
```

#### Return headers and body
```bash
curl http://localhost:3000
```

#### Return verbose output
```bash
curl -v http://localhost:3000
```

#### Send a POST request
 ```bash
curl -X POST http://localhost:3000
 ```
#### Send a POST request with JSON data
 ```bash
curl -v  -X POST http://localhost:3000 \
    -H "Content-Type: application/json" \
    -d '{"manufacturer":"Bullet","model":"Classic500","year":2024}'
 ```

#### Send a POST request with query parameters
```bash
curl -X POST "http://localhost:3000?manufacturer=Bullet&model=Classic500&year=2024"
```

### Send a Post request with multiple query parameters
```bash
curl -X POST "http://localhost:3000?manufacturer=Bullet&model=Classic500&year=2024&last_name=musk&first_name=elon"
```


### Important commands for setup project
- `cargo new project_name` -> Create a new Rust project
- `cargo add axum --features="ws"` -> Add axum as a dependency with features
