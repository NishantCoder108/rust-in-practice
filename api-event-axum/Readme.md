### What we are building
```text
API:
----
POST /v1/register -> User or Admin register themselves
POST /v1/login -> User or Admin login


POST /v1/event -> Create an event, Only by Admin
GET /v1/events -> Get all events, Only by Admin
GET /v1/event/:id -> Get a specific event, Only autheticated admin
PUT /v1/event/:id -> Update a specific event, Only by Admin
DELETE /v1/event/:id -> Delete a specific event, Only by Admin


Input:
------

For register request:
{
  "id": "nishant@solana.com",
  "name": "Nishant",
  "username": "nishant",
  "role": "ADMIN",
  "password": "12345"
}
-> Response:
{ 
  "username": "nishant",
  "message": "User registered successfully"
}


For login request:
{
  "username": "nishant",
  "password": "12345"
}
-> Response:
{
  "id": "hti-34o",
  "token": "ekffjfsjfj...skdfj",
  "message": "User logged in successfully"
}


For event creation:
{ 
  "id": "fjsf-34f",
  "title": "Rust Session",
  "description": "A session on Rust programming language",
  "date": "2024-07-01T10:00:00Z",
  "location": "online",
  "meet_url": "https://meet.google.com/",
  "organizer": "nishant"
}
-> Response:
{
  "id": "fjsf-34f",
  "message": "Event created successfully"
}


For Get all events:
GET /v1/events
-> Response:
{
  "events": [
    {
      "id": "fjsf-34f",
      "title": "Rust Session",
      "description": "A session on Rust programming language",
      "date": "2024-07-01T10:00:00Z",
      "location": "online",
      "meet_url": "https://meet.google.com/",
      "organizer": "nishant"
    }
  ]
}


For Get a specific event:
GET /v1/event/fjsf-34f
-> Response:
{
  "id": "fjsf-34f",
  "title": "Rust Session",
  "description": "A session on Rust programming language",
  "date": "2024-07-01T10:00:00Z",
  "location": "online",
  "meet_url": "https://meet.google.com/",
  "organizer": "nishant"
}


For Update a specific event:
PUT /v1/event/fjsf-34f
{
  "title": "Rust Session",
  "description": "A session on Rust programming language",
  "date": "2024-07-01T10:00:00Z",
  "location": "online",
  "meet_url": "https://meet.google.com/",
  "organizer": "nishant"
}
-> Response:
{
  "id": "fjsf-34f",
  "message": "Event updated successfully"
}


For Delete a specific event:
DELETE /v1/event/fjsf-34f
-> Response:
{
  "id": "fjsf-34f",
  "message": "Event deleted successfully"
}
```
---
### Curl command for testing API
#### Login test
```bash
curl -X POST http://localhost:3000/v1/login  \
   -H "Content-Type: application/json" \
   -d '{"name":"Nishant","username":"nishant","role":"ADMIN","id":"nishant@solaa.com", "password":"13444"}' | jq
```
- jq : a JSON processor that formats JSON output for readability.

#### Event test creation
```bash

curl -X POST http://localhost:3000/v1/event  \
   -H "Content-Type: application/json" \
   -d '{"title":"Rust Session","description":"A session on Rust programming language","date":"2024-07-01T10:00:00Z","location":"online","meet_url":"https://meet.google.com/","organizer":"nishant"}' | jq

```

#### Event test get all events
```bash
curl http://localhost:3000/v1/events | jq
```

#### Delete event test
```bash
curl -X DELETE http://localhost:3000/v1/event/f1234b13-59b8-4813-9983-95b7b8815246 | jq
```

#### Event test get by id
```bash
curl http://localhost:3000/v1/event/f1234b13-59b8-4813-9983-95b7b8815246 | jq
```
