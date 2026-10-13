# Full-Stack Web Development Roadmap

## 1. How the Internet Works

Understand:

* Client vs server
* IP addresses and ports
* DNS — domain → IP
* HTTP/HTTPS
* Request → response
* TCP/IP at a high level
* TLS/SSL at a high level
* What happens when you type `https://example.com` into a browser

You should be able to explain:

```text
Browser
   ↓
DNS
   ↓
Server IP
   ↓
HTTPS connection
   ↓
HTTP request
   ↓
Web server / application
   ↓
HTTP response
   ↓
Browser renders page
```

---

## 2. HTTP Fundamentals

This is probably the **most important web fundamental**.

### HTTP Methods

```text
GET
POST
PUT
PATCH
DELETE
```

### Status Codes

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

### Headers

```http
Content-Type: application/json
Authorization: Bearer ...
Cookie: ...
Cache-Control: ...
```

### Request / Response Body

```json
{
  "name": "Raj"
}
```

Also understand:

* Idempotency
* Caching
* Redirects
* Content types
* HTTP cookies
* CORS
* HTTP vs HTTPS

---

## 3. HTML

You don't need to become a designer, but you should understand semantic HTML.

Know:

```html
<html>
<head>
<body>

<header>
<nav>
<main>
<section>
<article>
<footer>

<form>
<input>
<button>
<select>
<textarea>

<a>
<img>
```

More importantly, understand:

* DOM
* Forms
* Form submission
* Links
* Accessibility basics
* Semantic elements
* Attributes
* Browser rendering

---

## 4. CSS

Understand the fundamentals before relying heavily on frameworks.

Learn:

* Selectors
* Cascade
* Specificity
* Inheritance
* Box model
* `display`
* Positioning
* Flexbox
* Grid
* Responsive design
* Media queries
* Units (`px`, `%`, `rem`, `em`, `vh`, `vw`)
* CSS variables

You should be able to build something like:

```text
┌─────────────────────────────┐
│           Navbar            │
├──────────────┬──────────────┤
│   Sidebar    │    Content   │
│              │              │
│              │              │
└──────────────┴──────────────┘
```

without needing a CSS framework.

---

## 5. JavaScript in the Browser

Before React/Next/Vue/etc., understand JavaScript itself.

Important topics:

* Variables
* Functions
* Objects
* Arrays
* Destructuring
* Modules
* Scope
* Closures
* Promises
* `async/await`
* Error handling
* Events
* DOM manipulation
* JSON
* `fetch()`

Especially understand:

```javascript
const response = await fetch("/api/users");

const users = await response.json();
```

and what happens asynchronously.

Also understand:

```text
JavaScript
   ↓
Browser APIs
   ↓
DOM / Fetch / Storage / Events
```

---

## 6. Browser Fundamentals

This is where many developers have gaps.

### DOM

```text
HTML
 ↓
DOM tree
 ↓
JavaScript interacts with DOM
```

### Browser Storage

Know the differences between:

* Cookies
* localStorage
* sessionStorage
* IndexedDB — basic awareness

### Browser Security

Understand:

* Same-origin policy
* CORS
* XSS
* CSRF
* Content Security Policy at a high level

These become **very important** once you implement authentication.

---

## 7. APIs

Before building full-stack apps, become comfortable designing and consuming APIs.

For example:

```text
GET    /api/users
GET    /api/users/123
POST   /api/users
PATCH  /api/users/123
DELETE /api/users/123
```

Understand:

* REST concepts
* Resources
* Routes
* Request parameters
* Query parameters
* Request bodies
* Response bodies
* Validation
* Error responses
* Pagination
* Filtering
* Sorting

For example:

```http
GET /api/products?category=books&page=2&limit=20
```

---

## 8. Backend Fundamentals

Then learn what actually happens on the server.

Understand:

```text
HTTP request
     ↓
Router
     ↓
Controller / Handler
     ↓
Business logic
     ↓
Database
     ↓
Response
```

Learn:

* Routing
* Middleware
* Request validation
* Authentication
* Authorization
* Error handling
* Logging
* Environment variables
* Configuration
* File uploads
* Background jobs

The specific backend language/framework matters less initially.

For example:

```text
Node.js + Express
Node.js + Fastify
Python + FastAPI
Java + Spring
Go
C# + ASP.NET
```

The underlying concepts are similar.

---

## 9. Databases

For most full-stack applications, learn **SQL first**.

Understand:

```text
Database
  ↓
Tables
  ↓
Rows
  ↓
Columns
```

Then learn:

* Primary keys
* Foreign keys
* Relationships
* One-to-one
* One-to-many
* Many-to-many
* Indexes
* Constraints
* Transactions
* Normalization
* Joins
* Aggregation

You should be comfortable writing things like:

```sql
SELECT
    users.name,
    orders.total
FROM users
JOIN orders
    ON orders.user_id = users.id
WHERE users.id = 123;
```

Then learn an ORM such as Prisma, Drizzle, SQLAlchemy, Hibernate, etc.

**Don't let the ORM hide SQL from you.**

---

## 10. Authentication & Authorization

This deserves its own topic.

Understand the difference:

**Authentication**

> Who are you?

**Authorization**

> What are you allowed to do?

Learn:

```text
Registration
   ↓
Password hashing
   ↓
Login
   ↓
Session / token
   ↓
Authenticated requests
   ↓
Authorization
```

Understand:

* Password hashing
* Sessions
* Cookies
* JWTs
* Access tokens
* Refresh tokens
* OAuth/OIDC
* Roles/permissions
* Session expiration
* Logout
* CSRF

You don't need to implement your own cryptography.

---

## 11. Frontend ↔ Backend Communication

Now connect everything.

For example:

```text
React
  │
  │ POST /api/orders
  ↓
Backend
  │
  │ validate user
  ↓
Business logic
  │
  ↓
PostgreSQL
  │
  ↓
Backend
  │
  │ 201 Created
  ↓
React
```

Understand:

* API calls
* Loading states
* Error states
* Optimistic updates
* Form submission
* Validation
* Authentication state
* Caching

---

## 12. Git and GitHub

Absolutely learn this before serious projects.

Know:

```bash
git init
git status
git add
git commit
git log
git branch
git switch
git merge
git pull
git push
```

Also understand:

* Branches
* Pull requests
* Merge conflicts
* `.gitignore`
* Environment secrets

---

## 13. Deployment Fundamentals

Eventually your application has to leave your laptop.

Understand:

```text
User
 ↓
DNS
 ↓
CDN / Reverse proxy
 ↓
Frontend
 ↓
API server
 ↓
Database
```

Learn the concepts behind:

* Domains
* DNS
* HTTPS certificates
* Reverse proxies
* Environment variables
* Containers/Docker
* CI/CD
* Cloud hosting
* Databases in production
* Logs
* Monitoring
* Backups

You don't need Kubernetes to build your first full-stack app.

---

# The Most Important Mental Model

If you're coming from software development, focus particularly on understanding this:

```text
                 INTERNET
                    │
                    ▼
             ┌─────────────┐
             │   Browser   │
             └──────┬──────┘
                    │
                 HTTP/S
                    │
                    ▼
             ┌─────────────┐
             │   Web/API   │
             │   Server    │
             └──────┬──────┘
                    │
             Business Logic
                    │
                    ▼
             ┌─────────────┐
             │  Database   │
             └─────────────┘
```

Then add:

```text
Authentication
     ↓
Sessions / Cookies / Tokens

Caching
     ↓
Browser / CDN / Server / Database

Security
     ↓
HTTPS / CORS / CSRF / XSS / SQL injection

Deployment
     ↓
DNS / Reverse Proxy / Containers / Cloud
```

Once you understand this architecture, frameworks become much easier to learn.

---

# Practical Learning Sequence

Don't spend months studying theory before building.

A good progression is:

```text
1. Internet + HTTP
        ↓
2. HTML
        ↓
3. CSS
        ↓
4. JavaScript
        ↓
5. Browser APIs + DOM
        ↓
6. REST APIs
        ↓
7. Backend framework
        ↓
8. SQL + PostgreSQL
        ↓
9. Authentication
        ↓
10. Frontend framework
        ↓
11. Testing
        ↓
12. Docker + deployment
```

Then build progressively.

---

# Project 1 — Todo App

```text
Frontend
    ↕
REST API
    ↕
Database
```

Goal: understand the complete request → backend → database → response → UI cycle.

---

# Project 2 — Authentication App

```text
Register
Login
Logout
Sessions
Protected routes
Roles
```

Goal: understand authentication, authorization, sessions, cookies, and protected resources.

---

# Project 3 — Real Application

For example:

```text
E-commerce
SaaS dashboard
Expense tracker
Project management system
Booking system
```

This third project is where the fundamentals start connecting together.

---

# Important Recommendation

Don't learn:

```text
React + Node + Express + MongoDB + Docker + AWS
```

simultaneously.

Learn the **web concepts first**, then use a stack to implement those concepts.

For example:

```text
HTML/CSS/JS
      ↓
    React
      ↓
 Node/Express
      ↓
 PostgreSQL
      ↓
    Docker
      ↓
  Deployment
```

This is much easier to understand than memorizing framework-specific APIs without knowing what problem they're solving.
