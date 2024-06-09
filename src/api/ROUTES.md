# Routes

## Health Check and Testing

| route          | method |
| -------------- | ------ |
| /health-check  | GET    |
| /public/hello  | GET    |
| /private/hello | GET    |

/private/hello requires Authorization token in the header.

## Auth

| route          | method |
| -------------- | ------ |
| /registration  | POST   |
| /login         | POST   |
| /refresh-token | POST   |

### /registration

In this route the user can create an account.

**Request:**

```json
{
    "username": "test",
    "password": "12345678",
    "mail": "test@gmail.com",
    "first_name": "test",
    "last_name": ""
}
```

- `username` must be unique and length login must be from 3 to 200 characters.
- `password` length password must be from 8 to 200 characters and password is hashed before being saved to the database.
- `mail` must be unique and be mail.
- `first_name` length must be from 2 to 150 characters and this field is optional.
- `last_name` length must be from 2 to 200 characters and this field is optional.

**Response:**

- Status Ok (200) -> `token` and `refresh_token` in json.
- Status BadRequest (400) -> “Weak login” / “Weak password” / “Already registered by login” / “Bad mail”/ “Already registered by mail” / “Wrong first name”/ “Wrong last name”.
- Status Internal Server Error (500) -> “Internal Server Error”.

### /login

In this route the user can log into an account.

**Request:**

```json
{
    "username": "test",
    "password": "12345678"
}
```

- `username`
- `password`

**Response:**

- Status Ok (200) -> `token` and `refresh_token` in json.
- Status BadRequest (400) -> “Bad request”.
- Status Internal Server Error (500) -> “Internal Server Error”.

### /refresh-token

In this route the program will update the lifetime of the `access_token`.

**Request:**

header:

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNjY2NTQxMjlmZTY2OGEzYTI2NjdjNDg1IiwiZXhwIjoxNzE3OTE2MzU2fQ.umJRApPYz2Wbjc1yM65dfMxjBFGqZdgvcM_x_e_QnPM
```

body:

```json
{
    "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNjY2NTQxMjlmZTY2OGEzYTI2NjdjNDg1IiwiZXhwIjoxNzIwNTA0NzU2fQ.SWsvNQHMvcHZ92fHxGFyrxxA-VBrKMoBRUvw5ZrsWAA"
}
```

- `username`

**Response:**

- Status Ok (200) -> `token` and `refresh_token` in json.
- Status Unauthorized (401) -> “Unauthorized”.
- Status Internal Server Error (500) -> “Internal Server Error”.
