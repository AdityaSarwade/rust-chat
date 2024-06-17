# Routes

- [Routes](#routes)
  - [Health Check and Testing](#health-check-and-testing)
  - [Chat](#chat)
    - [/events](#events)
    - [/message](#message)
  - [Auth](#auth)
    - [/registration](#registration)
    - [/login](#login)
    - [/refresh-token](#refresh-token)
  - [User](#user)
    - [(GET) /user](#get-user)
    - [(PATCH) /user](#patch-user)
    - [(DELETE) /user](#delete-user)

## Health Check and Testing

| route          | method |
| -------------- | ------ |
| /health-check  | GET    |
| /public/hello  | GET    |
| /private/hello | GET    |

/private/hello requires Authorization token in the header.

## Chat

| route    | method |
| -------- | ------ |
| /events  | GET    |
| /message | GET    |

### /events

In this route the user can connect to the Server using [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events) to listen for Server Sent Events.

### /message

In this route the user can send a message.

**Request:**

```json
{
    "username": "guest",
    "room": "ChatRoom 1",
    "message": "test message"
}
```

- `username` must be a String of upto 19 characters.
- `room` must be a String of upto 29 characters.
- `message` must be String.

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

- `username` must be unique and length username must be from 3 to 200 characters.
- `password` length password must be from 8 to 200 characters and password is hashed before being saved to the database.
- `mail` must be unique and be valid.
- `first_name` length must be from 2 to 150 characters and this field is optional.
- `last_name` length must be from 2 to 200 characters and this field is optional.

**Response:**

- Status Ok (200) -> `token` and `refresh_token` in json.
- Status BadRequest (400) -> “Weak username” / “Weak password” / “Already registered by username” / “Bad mail”/ “Already registered by mail” / “Wrong first name”/ “Wrong last name”.
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

**Response:**

- Status Ok (200) -> `token` and `refresh_token` in json.
- Status Unauthorized (401) -> “Unauthorized”.
- Status Internal Server Error (500) -> “Internal Server Error”.

## User

| route | method |
| ----- | ------ |
| /user | GET    |
| /user | PATCH  |
| /user | DELETE |

### (GET) /user

In this route, the user can retrieve account information (except for the password).

**Request:**

header:

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNjY2NTQxMjlmZTY2OGEzYTI2NjdjNDg1IiwiZXhwIjoxNzE3OTE2MzU2fQ.umJRApPYz2Wbjc1yM65dfMxjBFGqZdgvcM_x_e_QnPM
```

**Response:**

- Status Ok (200) -> `username`, `mail`, `id` ,`first_name` and `last_name`.
- Status Unauthorized (401) -> “Unauthorized”.
- Status Internal Server Error (500) -> “Internal Server Error”.

### (PATCH) /user

In this route, the user can modify/update account information.

**Request:**

header:

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNjY2NTQxMjlmZTY2OGEzYTI2NjdjNDg1IiwiZXhwIjoxNzE3OTE2MzU2fQ.umJRApPYz2Wbjc1yM65dfMxjBFGqZdgvcM_x_e_QnPM
```

body:

```json
{
    "username": "test",
    "mail": "test@gmail.com",
    "first_name": "test",
    "last_name": ""
}
```

- `username` must be unique and length username must be from 3 to 200 characters.
- `mail` must be unique and valid.
- `first_name` length must be from 2 to 150 characters and this field is optional.
- `last_name` length must be from 2 to 200 characters and this field is optional.

**Response:**

- Status Ok (200).
- Status BadRequest (400) -> “Weak username” / “Already registered by username” / “Bad mail”/ “Already registered by mail” / “Wrong first name”/ “Wrong last name”.
- Status Unauthorized (401) -> “Unauthorized”.
- Status Internal Server Error (500) -> “Internal Server Error”.

### (DELETE) /user

In this route, the user can delete the account.

**Request:**

header:

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiNjY2NTQxMjlmZTY2OGEzYTI2NjdjNDg1IiwiZXhwIjoxNzE3OTE2MzU2fQ.umJRApPYz2Wbjc1yM65dfMxjBFGqZdgvcM_x_e_QnPM
```

**Response:**

- Status No content (204).
- Status Unauthorized (401) -> “Unauthorized”
- Status Internal Server Error (500) -> “Internal Server Error”.
