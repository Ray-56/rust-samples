# Axum test


## Curl fetch

```bash
# get Token string
curl -v -H "Content-Type: application/json" -X POST -d '{"email": "a@b.com", "password": "secret"}' http://127.0.0.1:8080/login
# Response: {"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IlJheSBGYW4ifQ.N0ndumtsBNNReWCybTEDhu452uo7tgI6J_7lApoONvQ"}
curl -v -H "Content-Type: application/json" -H"Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IlJheSBGYW4ifQ.N0ndumtsBNNReWCybTEDhu452uo7tgI6J_7lApoONvQ" -X POST -d '{"title": "tttt"}' http://127.0.0.1:8080/todos
```