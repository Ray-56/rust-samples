# Axum test

Video from https://www.bilibili.com/video/BV1kq4y1h7w6?spm_id_from=333.999.0.0

Doc from 


## Curl fetch

```bash
# get Token string
curl -v -H "Content-Type: application/json" -X POST -d '{"email": "a@b.com", "password": "secret"}' http://127.0.0.1:8080/login
# Response: {"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IlJheSBGYW4ifQ.N0ndumtsBNNReWCybTEDhu452uo7tgI6J_7lApoONvQ"}
curl -v -H "Content-Type: application/json" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IlJheSBGYW4iLCJleHAiOjE2NTM3NDg1OTR9.SSLH3ogae7jQaaj42tn-8_dmn62ISWARhZmX5xJsI9E" -X POST -d '{"title": "tttt"}' http://127.0.0.1:8080/todos

curl -v -H "Content-Type: application/json" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IlJheSBGYW4iLCJleHAiOjE2NTM3NDg1OTR9.SSLH3ogae7jQaaj42tn-8_dmn62ISWARhZmX5xJsI9E" http://127.0.0.1:8080/todos
```