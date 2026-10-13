# HTTP Fundamentals

HTTP (HyperText Transfer Protocol) is the protocol used for communication between a client and a server on the web.

When you open a website, your browser sends an HTTP request to a server, and the server sends an HTTP response back.

The basic communication looks like this:

```text
Client (Browser)
      |
      | HTTP Request
      ↓
    Server
      |
      | HTTP Response
      ↓
Client (Browser)
```

For example, when your browser requests:

```http
GET /users/123 HTTP/1.1
Host: example.com
```

The server might respond with:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 123,
  "name": "Raj"
}
```

HTTP defines how this request and response should be structured and how the client and server should communicate.


## 1. HTTP Methods

HTTP methods describe **what the client wants to do with a resource**.

The most important methods for full-stack development are:

```text
GET
POST
PUT
PATCH
DELETE
```

Think of an API resource such as:

```text
/users
```

and an individual user:

```text
/users/123
```

### GET

`GET` is used to **retrieve data** from a server.

Example:

```http
GET /users
```

The server might respond:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": 1,
    "name": "Raj"
  },
  {
    "id": 2,
    "name": "John"
  }
]
```

Getting a specific user:

```http
GET /users/123
```

Response:

```json
{
  "id": 123,
  "name": "Raj"
}
```

A `GET` request normally does not contain a request body.

In a web application, loading a list of products might result in:

```http
GET /api/products
```

### POST

`POST` is generally used to **create a new resource** or submit data to the server.

Example:

```http
POST /users
Content-Type: application/json

{
  "name": "Raj",
  "email": "raj@example.com"
}
```

The server creates the user and might respond:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": 123,
  "name": "Raj",
  "email": "raj@example.com"
}
```

A common full-stack example is user registration:

```text
Browser
   |
   | POST /api/register
   | { name, email, password }
   ↓
Backend
   |
   | Create user
   ↓
Database
```

`POST` is commonly used for operations where submitting the same request again may create another resource.

For example:

```http
POST /orders
```

could create a new order each time it is submitted.


### PUT

`PUT` is generally used to **replace an existing resource** with a new representation.

Example:

```http
PUT /users/123
Content-Type: application/json

{
  "name": "Rajendra",
  "email": "raj@example.com"
}
```

The server treats the provided representation as the new state of the resource.

Before:

```json
{
  "id": 123,
  "name": "Raj",
  "email": "old@example.com"
}
```

After:

```json
{
  "id": 123,
  "name": "Rajendra",
  "email": "raj@example.com"
}
```

The important idea is that `PUT` generally represents **replacement**, rather than changing only one field.


### PATCH

`PATCH` is used to **partially update an existing resource**.

For example, suppose the user currently has:

```json
{
  "id": 123,
  "name": "Raj",
  "email": "raj@example.com",
  "age": 30
}
```

You only want to change the name.

You can send:

```http
PATCH /users/123
Content-Type: application/json

{
  "name": "Rajendra"
}
```

The server can update only that field:

```json
{
  "id": 123,
  "name": "Rajendra",
  "email": "raj@example.com",
  "age": 30
}
```

So the common distinction is:

```text
PUT
→ Replace the resource

PATCH
→ Partially modify the resource
```

### DELETE

`DELETE` is used to **remove a resource**.

Example:

```http
DELETE /users/123
```

The server might respond:

```http
HTTP/1.1 204 No Content
```

The user with ID `123` has been deleted.

A typical frontend operation might look like:

```javascript
fetch("/api/users/123", {
  method: "DELETE"
});
```


## 2. Status Codes

HTTP status codes tell the client **what happened when the server processed the request**.

They are grouped into categories:

```text
1xx → Informational
2xx → Success
3xx → Redirection
4xx → Client-side/request problem
5xx → Server-side problem
```

The most important status codes for application development are:

```text
200 OK
201 Created
204 No Content
400 Bad Request
401 Unauthorized
403 Forbidden
404 Not Found
409 Conflict
422 Unprocessable Entity
500 Internal Server Error
```

### 200 OK

`200 OK` means the request was successfully processed.

Example:

```http
GET /users/123
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 123,
  "name": "Raj"
}
```

Typical uses:

```text
GET → 200 OK
Successful update → 200 OK
Successful operation returning data → 200 OK
```

### 201 Created

`201 Created` means the request successfully created a new resource.

Example:

```http
POST /users
Content-Type: application/json

{
  "name": "Raj"
}
```

Response:

```http
HTTP/1.1 201 Created

{
  "id": 123,
  "name": "Raj"
}
```

The distinction is:

```text
200 → Request succeeded

201 → Request succeeded AND a resource was created
```

### 204 No Content

`204 No Content` means the request succeeded but there is **no response body to return**.

For example:

```http
DELETE /users/123
```

Response:

```http
HTTP/1.1 204 No Content
```

This is useful when the operation succeeded but the client does not need additional data.

### 400 Bad Request

`400 Bad Request` means the server could not process the request because the request itself is invalid.

For example, an API expects:

```json
{
  "name": "Raj",
  "email": "raj@example.com"
}
```

but receives malformed JSON:

```json
{
  "name": "Raj",
```

The server might return:

```http
HTTP/1.1 400 Bad Request
```

It can also be used when the request syntax or parameters are invalid.

### 401 Unauthorized

`401 Unauthorized` means the request does not have valid authentication credentials.

For example:

```http
GET /api/profile
Authorization: Bearer invalid-token
```

The server might respond:

```http
HTTP/1.1 401 Unauthorized
```

Typical situations:

```text
No authentication credentials
Invalid authentication credentials
Expired authentication credentials
```

A useful way to remember it:

```text
401
→ "I don't know who you are."
```

### 403 Forbidden

`403 Forbidden` means the server understands the request and knows who the client is, but the client **does not have permission** to perform the operation.

For example:

```text
User → Regular employee
Resource → Admin dashboard
```

The user may be authenticated, but does not have the required permission.

Response:

```http
HTTP/1.1 403 Forbidden
```

A useful distinction:

```text
401
→ Authentication problem

403
→ Permission problem
```

### 404 Not Found

`404 Not Found` means the requested resource could not be found.

Example:

```http
GET /users/999999
```

If that user doesn't exist:

```http
HTTP/1.1 404 Not Found
```

It can also happen when a requested URL does not exist:

```http
GET /api/something-that-does-not-exist
```

### 409 Conflict

`409 Conflict` means the request conflicts with the current state of the resource.

A common example is attempting to register an email address that already exists.

Request:

```http
POST /users
Content-Type: application/json

{
  "email": "raj@example.com"
}
```

If that email is already registered:

```http
HTTP/1.1 409 Conflict
```

The idea is:

```text
The request may be valid,
but it conflicts with existing state.
```

### 422 Unprocessable Entity

`422 Unprocessable Entity` is commonly used when the server understands the request, but the submitted data fails validation.

For example:

```http
POST /users
Content-Type: application/json

{
  "name": "Raj",
  "email": "not-an-email"
}
```

The JSON itself is valid, but the email value is invalid.

The server might respond:

```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/json

{
  "errors": {
    "email": "Invalid email address"
  }
}
```

So:

```text
400
→ The request itself is bad/invalid.

422
→ The request can be understood,
   but the submitted data fails validation.
```

The exact choice between `400` and `422` depends on the API's conventions.

### 500 Internal Server Error

`500 Internal Server Error` means the server encountered an unexpected problem while processing the request.

For example:

```text
Browser
   |
   | GET /users
   ↓
Backend
   |
   | Database operation fails unexpectedly
   ↓
500 Internal Server Error
```

Response:

```http
HTTP/1.1 500 Internal Server Error
```

A `500` generally indicates a problem on the server side rather than something the client should fix by changing normal request data.


## 3. Headers

HTTP headers provide **metadata and additional information about a request or response**.

A request can contain headers such as:

```http
Content-Type: application/json
Authorization: Bearer ...
Cookie: ...
Cache-Control: ...
```

A response can also contain headers.

For example:

```http
HTTP/1.1 200 OK
Content-Type: application/json
Cache-Control: max-age=3600
```

Headers are essentially key-value pairs:

```text
Header-Name: value
```

### Content-Type

`Content-Type` tells the other side **what type of data is being sent**.

For JSON:

```http
Content-Type: application/json
```

Example:

```http
POST /users
Content-Type: application/json

{
  "name": "Raj"
}
```

The server can use the `Content-Type` to understand how to interpret the request body.

Another example is HTML:

```http
Content-Type: text/html
```

An image might use:

```http
Content-Type: image/png
```

So:

```text
Content-Type
→ "What format is this data?"
```


### Authorization

The `Authorization` header is commonly used to send authentication credentials.

For example:

```http
Authorization: Bearer eyJhbGciOi...
```

A common API flow is:

```text
Login
  ↓
Server provides access token
  ↓
Client stores token
  ↓
Client sends token with API requests
```

Example:

```http
GET /api/profile
Authorization: Bearer <token>
```

The server verifies the token before returning protected data.

### Cookie

The `Cookie` header sends cookies from the browser to the server.

Example:

```http
Cookie: sessionId=abc123
```

Cookies are commonly used for things such as maintaining a user's session.

For example:

```text
Browser
   |
   | Cookie: sessionId=abc123
   ↓
Server
   |
   | Finds session associated with abc123
   ↓
User is recognized
```

Cookies can also be used for other browser-related state.

### Cache-Control

`Cache-Control` provides instructions about caching HTTP responses.

Example:

```http
Cache-Control: max-age=3600
```

This can tell a cache that the response can be considered fresh for 3600 seconds.

Caching can reduce unnecessary requests:

```text
Without caching:

Browser → Server
Browser → Server
Browser → Server
Browser → Server


With caching:

Browser → Server
          ↓
        Cache
          ↓
Browser ← Cached response
```

Caching can improve performance and reduce server/network traffic.


## 4. Request / Response Body

The **body** contains the actual data being sent in an HTTP request or response.

For example:

```http
POST /users
Content-Type: application/json

{
  "name": "Raj"
}
```

Here:

```text
POST /users
→ Request line

Content-Type: application/json
→ Header

{
  "name": "Raj"
}
→ Request body
```

The server might return:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": 123,
  "name": "Raj"
}
```

Here:

```text
HTTP/1.1 201 Created
→ Status

Content-Type: application/json
→ Header

{
  "id": 123,
  "name": "Raj"
}
→ Response body
```

A JSON body is extremely common in modern APIs.

For example:

```json
{
  "id": 123,
  "name": "Raj",
  "email": "raj@example.com"
}
```

The frontend can parse this response and use the data to update the UI.


### Idempotency

An operation is **idempotent** when making the same request multiple times has the same intended effect on the server's state as making it once.

For example, suppose:

```http
PUT /users/123

{
  "name": "Rajendra"
}
```

After the first request:

```text
User name = Rajendra
```

Send the same request again:

```http
PUT /users/123

{
  "name": "Rajendra"
}
```

The intended final state is still:

```text
User name = Rajendra
```

Sending it multiple times does not keep changing the result.

A common comparison is:

```text
PUT
→ Generally idempotent

DELETE
→ Generally idempotent

GET
→ Generally idempotent

POST
→ Generally NOT idempotent
```

For example:

```http
POST /orders
```

might create:

```text
Order #101
```

Sending the same request again could create:

```text
Order #102
```

So the operation can have a different effect each time.

Important: **idempotent does not mean the response must be identical every time**. It means the intended effect on the resource state is the same.


### Caching

Caching means **temporarily storing a response so it can be reused instead of requesting the same data again**.

Suppose a browser requests:

```http
GET /products
```

The server responds with product data.

A cache may store that response.

A later request can potentially be served from the cache:

```text
First request:

Browser → Server
          ↓
        Response
          ↓
        Browser
          ↓
         Cache


Later request:

Browser → Cache
          ↓
      Cached response
```

Caching can make applications faster because the client may not need to communicate with the server for every request.

HTTP provides caching-related headers such as:

```http
Cache-Control: max-age=3600
```

Caching is particularly useful for resources that do not change frequently.


### Redirects

A redirect tells the client that it should make a request to another location.

HTTP uses `3xx` status codes for redirects.

For example:

```http
HTTP/1.1 301 Moved Permanently
Location: https://example.com/new-page
```

The browser can then request:

```http
GET /new-page
```

Redirects are commonly used when:

```text
An URL has permanently changed
A page has temporarily moved
HTTP traffic should move to HTTPS
A resource has another canonical location
```

The `Location` header tells the client where to go:

```http
Location: https://example.com/new-page
```

A common example is:

```text
http://example.com
        ↓
      redirect
        ↓
https://example.com
```


### Content types

A content type describes **the format of the data being transmitted**.

It is communicated using the `Content-Type` header.

For example, JSON:

```http
Content-Type: application/json
```

with:

```json
{
  "name": "Raj"
}
```

HTML:

```http
Content-Type: text/html
```

with:

```html
<h1>Hello</h1>
```

Plain text:

```http
Content-Type: text/plain
```

with:

```text
Hello Raj
```

An image:

```http
Content-Type: image/png
```

The content type allows the receiver to understand how the body should be interpreted.

A typical API request therefore looks like:

```http
POST /users
Content-Type: application/json

{
  "name": "Raj"
}
```

### HTTP cookies

Cookies are small pieces of data that a server can ask a browser to store.

A server can send a cookie using the `Set-Cookie` response header:

```http
HTTP/1.1 200 OK
Set-Cookie: sessionId=abc123
```

The browser stores the cookie.

For a later request, the browser can send it back:

```http
GET /profile
Cookie: sessionId=abc123
```

This allows the server to associate the request with a particular session.

A simplified login flow looks like:

```text
User logs in
     ↓
POST /login
     ↓
Server verifies credentials
     ↓
Server sends Set-Cookie
     ↓
Browser stores cookie
     ↓
Browser requests /profile
     ↓
Browser sends Cookie
     ↓
Server identifies session
     ↓
Server returns profile
```

Cookies are therefore an important mechanism for maintaining state between HTTP requests.


### CORS

CORS stands for **Cross-Origin Resource Sharing**.

It controls whether a browser is allowed to make requests from one origin to a different origin.

For example, suppose your frontend runs at:

```text
https://app.example.com
```

and your API runs at:

```text
https://api.example.com
```

The browser sees these as different origins.

Your frontend might make:

```javascript
fetch("https://api.example.com/users");
```

The API can explicitly allow the frontend's origin using a response header such as:

```http
Access-Control-Allow-Origin: https://app.example.com
```

Then the browser can allow the frontend to access the response.

A simplified flow:

```text
Frontend
https://app.example.com
        |
        | Request
        ↓
API
https://api.example.com
        |
        | Access-Control-Allow-Origin
        ↓
Browser decides whether
frontend may access response
```

An important point is:

```text
CORS is enforced by browsers.
```

It is not primarily a mechanism that prevents one server from communicating with another server.

For example, server-to-server communication does not follow the browser's CORS enforcement in the same way.

A common error in frontend development is:

```text
Access to fetch at 'https://api.example.com'
from origin 'https://app.example.com'
has been blocked by CORS policy
```

This usually means the API's CORS configuration does not allow the browser's origin for that request.


### HTTP vs HTTPS

HTTP and HTTPS are both used for web communication, but HTTPS adds encryption and authentication through TLS.

HTTP:

```text
Browser
   |
   | HTTP
   ↓
Server
```

HTTPS:

```text
Browser
   |
   | HTTPS
   | encrypted connection
   ↓
Server
```

With normal HTTP, data is transmitted without the protections provided by TLS.

With HTTPS, TLS helps provide:

```text
Encryption
Authentication
Integrity
```

For example:

```text
HTTP
http://example.com

HTTPS
https://example.com
```

HTTPS protects data while it travels between the client and server.

This is especially important when transmitting sensitive information such as:

```text
Login credentials
Authentication information
Personal information
Payment-related information
```

HTTPS does not change the basic idea of HTTP requests and responses.

You still have:

```text
Request
   ↓
Server
   ↓
Response
```

The difference is that HTTPS carries HTTP communication over a TLS-protected connection.

So conceptually:

```text
HTTP
= Web communication protocol

HTTPS
= HTTP + TLS protection
```