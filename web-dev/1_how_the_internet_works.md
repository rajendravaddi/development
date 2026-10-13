# How the Internet Works

Before building full-stack applications, you should understand what happens when a browser communicates with a server.

The goal is not to become a networking engineer.

The goal is to understand this flow deeply:

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

We will build this understanding step by step.


## 1. Client vs Server

The first fundamental concept is:

> A client asks for something. A server provides something.

### Client

A **client** is a program/device that makes a request to another system.

In web development, the most common client is a **web browser**.

Examples:

```text
Chrome
Firefox
Safari
Edge
```

But a client doesn't have to be a browser.

Other examples:

```text
Mobile application
Desktop application
curl
Postman
Another backend server
IoT device
```

For example:

```text
Browser
   │
   │ "Give me example.com"
   ↓
Server
```

The browser is the client because it initiates the communication.


### Server

A **server** is a computer/program that listens for requests and provides some service.

For example, a web server may receive:

```http
GET /products
```

and return:

```json
[
  {
    "id": 1,
    "name": "Laptop"
  },
  {
    "id": 2,
    "name": "Phone"
  }
]
```

A server could provide:

* Web pages
* APIs
* Images
* Videos
* Files
* Authentication
* Database-backed data


### Important: "Server" Can Mean Two Things

The word server is commonly used in two ways.

#### Server as a computer

A physical or virtual machine connected to a network.

For example:

```text
Server computer
IP: 203.0.113.10
```

#### Server as software

A program running on that computer that listens for requests.

For example:

```text
Linux machine
    ↓
Node.js process
    ↓
Express application
    ↓
Listening on port 3000
```

So when someone says:

> "The server received the request."

they may mean the machine, the server software, or the application running on it.

Context determines which one.


## 2. A Simple Client-Server Example

Imagine you open:

```text
https://example.com
```

Your browser acts as the client.

Some server somewhere hosts the website.

Conceptually:

```text
┌──────────────┐
│   Browser    │
│   (Client)   │
└──────┬───────┘
       │
       │ Request
       ↓
┌──────────────┐
│    Server    │
│              │
│   Web App    │
└──────┬───────┘
       │
       │ Response
       ↓
┌──────────────┐
│   Browser    │
└──────────────┘
```

The browser doesn't normally know the server by its human-friendly domain name.

It ultimately needs an **IP address**.

That brings us to the next concept.


## 3. IP Addresses

Every device participating in an IP network needs an address so that network traffic can be delivered to the correct destination.

That address is called an **IP address**.

Think of an IP address somewhat like a postal address.

For example:

```text
203.0.113.10
```

This is an IPv4 address.

Another version is IPv6:

```text
2001:db8::1
```

You don't need to memorize IPv6 syntax right now.

Just understand:

```text
Domain name
     ↓
IP address
     ↓
Network destination
```

## 4. Why Do We Need IP Addresses?

Suppose your computer wants to communicate with a server.

It needs to know:

> "Where should I send these network packets?"

An IP address provides that addressing information.

Conceptually:

```text
Your computer
IP: 192.168.1.20

        ↓

    Internet

        ↓

     Server
IP: 203.0.113.10
```

The network uses IP addresses to move packets toward the destination.


## 5. Public vs Private IP Addresses

You'll encounter two broad categories frequently.

### Private IP

Used inside local networks.

Examples:

```text
192.168.x.x
10.x.x.x
172.16.x.x - 172.31.x.x
```

For example:

```text
Your laptop
192.168.1.20

Your phone
192.168.1.21

Your TV
192.168.1.22
```

These devices might all be connected to the same home router.


### Public IP

Your network can have a public IP address that is reachable across the public Internet.

Conceptually:

```text
Laptop
192.168.1.20
     ↓
Home Router
     ↓
Public IP
     ↓
Internet
```

You don't need to understand NAT deeply yet, but know that your private local address and your public Internet-facing address can be different.


## 6. IP Addresses Are Not Domain Names

Consider:

```text
example.com
```

That's a **domain name**.

Consider:

```text
203.0.113.10
```

That's an **IP address**.

They serve different purposes.

Humans prefer:

```text
google.com
github.com
example.com
```

Computers ultimately communicate using network addresses such as:

```text
IP address
```

We therefore need a system that translates:

```text
example.com
      ↓
IP address
```

That system is DNS.


## 7. Ports

An IP address tells the network **which machine** you're trying to reach.

A port helps identify **which network service/application** on that machine should receive the traffic.

Think of:

```text
IP address = building
Port       = apartment/door
```

For example:

```text
203.0.113.10:443
```

means:

```text
IP address: 203.0.113.10
Port:       443
```

## 8. Why Do We Need Ports?

Imagine one server running several services:

```text
Server
203.0.113.10

Port 80
→ HTTP server

Port 443
→ HTTPS server

Port 22
→ SSH server

Port 5432
→ PostgreSQL
```

The IP address identifies the machine.

The port identifies the service endpoint.

Conceptually:

```text
203.0.113.10:443
        │      │
        │      └── Port
        │
        └───────── IP address
```

## 9. Common Ports

You don't need to memorize every port, but these are worth knowing:

```text
22     SSH
53     DNS
80     HTTP
443    HTTPS
5432   PostgreSQL
3306   MySQL
6379   Redis
```

Development servers often use ports such as:

```text
3000
3001
4000
5000
8000
8080
```

These aren't special Internet standards in the same way that 80 and 443 are.

They're simply commonly used development ports.


## 10. URL

When you type:

```text
https://example.com/products?id=123
```

you're entering a **URL**.

A URL can contain several pieces:

```text
https://example.com/products?id=123
│       │           │        │
│       │           │        └── Query parameter
│       │           └─────────── Path
│       └─────────────────────── Domain
└─────────────────────────────── Scheme
```

More precisely:

```text
https://example.com:443/products?id=123#reviews
│       │           │   │        │
│       │           │   │        └── Fragment
│       │           │   └─────────── Query
│       │           └─────────────── Path
│       └─────────────────────────── Host
└─────────────────────────────────── Scheme
```

The port may be omitted because browsers know the default port for common schemes.

For HTTPS:

```text
https://example.com
```

normally means:

```text
example.com:443
```

For HTTP:

```text
http://example.com
```

normally means:

```text
example.com:80
```

## 11. DNS — Domain Name System

DNS stands for:

> Domain Name System

Its main job is to translate domain names into IP addresses.

Conceptually:

```text
example.com
     ↓
    DNS
     ↓
203.0.113.10
```

This is why DNS is often described as:

> The phonebook of the Internet.

Although the real system is more sophisticated than a simple phonebook.


## 12. Why Don't We Just Use IP Addresses?

You technically can access a server using an IP address.

For example:

```text
https://203.0.113.10
```

But humans don't want to memorize addresses like:

```text
142.250.72.14
```

Instead, we use:

```text
google.com
```

DNS gives us the mapping.

```text
google.com
     ↓
IP address
```


## 13. What Happens During DNS Lookup?

Suppose you enter:

```text
https://example.com
```

The browser needs the IP address.

It may first check whether the answer is already cached.

There can be several layers of caching.

Conceptually:

```text
Browser cache
     ↓
Operating system cache
     ↓
DNS resolver cache
     ↓
DNS infrastructure
```

If the answer isn't available from cache, a DNS resolver can query the DNS hierarchy to find the appropriate record.

You don't need to memorize the complete DNS hierarchy yet.

Just understand the important idea:

```text
Domain
  ↓
DNS lookup
  ↓
IP address
```


## 14. DNS Records

DNS doesn't only store IP addresses.

You'll eventually encounter records such as:

```text
A
AAAA
CNAME
MX
TXT
NS
```

For example:

### A record

Maps a domain to an IPv4 address.

```text
example.com
      ↓
203.0.113.10
```

### AAAA record

Maps a domain to an IPv6 address.

### CNAME

Creates an alias to another domain name.

### MX

Used for mail delivery.

### TXT

Can contain various pieces of text-based configuration information, including domain verification information.

You don't need to master DNS records yet.

For now:

> DNS translates human-friendly names into network destinations.


## 15. TCP/IP

Now we get into networking.

You don't need networking-engineer-level knowledge.

You need the mental model.

The Internet uses a collection of networking protocols commonly referred to as the **TCP/IP protocol suite**.

A simplified model is:

```text
Application
    ↓
Transport
    ↓
Internet
    ↓
Link
```

Different models use slightly different terminology, but this simplified view is enough for web development.


## 16. IP — Internet Protocol

IP is responsible for addressing and routing packets between networks.

Imagine your HTTP data needs to travel:

```text
Your computer
     ↓
Home router
     ↓
    ISP
     ↓
Several networks
     ↓
Server network
     ↓
  Server
```

IP provides the addressing/routing foundation that allows packets to travel toward the destination.


## 17. TCP — Transmission Control Protocol

TCP operates above IP.

A simplified idea is:

> IP gets packets toward the destination; TCP provides a reliable ordered connection between applications.

TCP provides things such as:

* Reliable delivery
* Ordering
* Retransmission of lost data
* Flow control
* Connection management

Imagine sending:

```text
Packet 1
Packet 2
Packet 3
Packet 4
```

If packet 3 is lost, TCP can detect the missing data and arrange for it to be retransmitted.

The application can then receive the data as an ordered stream.


## 18. TCP Connection

Before sending data over a traditional TCP connection, the two sides establish a connection.

This is commonly associated with the:

```text
TCP three-way handshake
```

Conceptually:

```text
Client                 Server

  SYN  ────────────────>

       <──────────── SYN + ACK

  ACK  ────────────────>
```

You don't need to memorize the packet flags yet.

The important concept is:

> TCP establishes a reliable connection before application data is exchanged.


## 19. TCP/IP Mental Model

For a web application, think roughly like this:

```text
HTTP
  ↓
TCP
  ↓
IP
  ↓
Network hardware
  ↓
Internet
  ↓
Network hardware
  ↓
IP
  ↓
TCP
  ↓
HTTP
```

This is one of the most important networking concepts to understand.

HTTP doesn't directly "travel through the Internet."

It is carried by lower-level networking protocols.


## 20. HTTP

HTTP stands for:

> Hypertext Transfer Protocol

HTTP is an **application-layer protocol** used for communication between clients and servers.

For example:

```text
Browser
   │
   │ HTTP request
   ↓
Server
   │
   │ HTTP response
   ↓
Browser
```

HTTP defines how these messages are structured.


## 21. HTTP Request

A simplified HTTP request looks like:

```http
GET /products HTTP/1.1
Host: example.com
Accept: text/html
```

The important parts are:

```text
Method
Path
Headers
Body (when applicable)
```

For example:

```http
POST /api/users HTTP/1.1
Host: example.com
Content-Type: application/json

{
  "name": "Raj"
}
```


## 22. HTTP Response

The server responds:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "message": "User created"
}
```

A response contains things such as:

```text
Status code
Headers
Body
```

So the basic communication is:

```text
Client
  │
  │ Request
  ↓
Server
  │
  │ Response
  ↓
Client
```


## 1.23 HTTPS

HTTPS means:

> HTTP Secure

It is HTTP communication protected using TLS.

Conceptually:

```text
HTTP
 +
TLS
 =
HTTPS
```

HTTPS provides important security properties such as:

* Encryption
* Server authentication
* Integrity protection

This prevents network attackers from simply reading or modifying HTTPS traffic in transit.


## 24. Why Is HTTPS Necessary?

Imagine you're logging into:

```text
https://bank.example
```

Your browser might send:

```json
{
  "username": "raj",
  "password": "..."
}
```

You obviously don't want someone on the network to be able to read that information.

With HTTPS:

```text
Browser
   │
   │ Encrypted data
   ↓
Internet
   │
   │ Encrypted data
   ↓
Server
```

An observer may see network metadata such as the destination and traffic characteristics, but the protected HTTP contents are encrypted.


## 25. TLS

TLS stands for:

> Transport Layer Security

TLS is the cryptographic protocol used to secure HTTPS connections.

You may still hear people say:

> SSL certificate

Historically, SSL was an older predecessor to TLS.

Modern HTTPS uses TLS.

So when people casually say:

> "SSL"

they often mean:

> TLS-based HTTPS security.


## 26. What Does TLS Actually Do?

At a high level, TLS provides three important properties.

### 1. Encryption

Protects data from being read by unauthorized observers.

```text
Original:

GET /account

        ↓ TLS

Encrypted data

        ↓

      Server
```


### 2. Authentication

The browser can verify that it is communicating with the intended website/server.

This is where **certificates** come into play.

For example:

```text
example.com
     ↓
TLS certificate
     ↓
Certificate authority
```

The browser verifies the certificate chain according to its trust rules.


### 3. Integrity

TLS helps ensure that data isn't silently modified while traveling across the network.

Conceptually:

```text
Browser
   ↓
Protected message
   ↓
Internet
   ↓
Protected message
   ↓
Server
```

If someone modifies the protected data, the integrity checks should detect the tampering.


## 27. TLS Handshake — High Level

When establishing a secure HTTPS connection, the client and server perform a TLS handshake.

Very simplified:

```text
Browser
   │
   │ "I want a secure connection"
   ↓
Server
   │
   │ Certificate + handshake information
   ↓
Browser
   │
   │ Verify certificate
   │ Establish cryptographic keys
   ↓
Secure connection
```

After the handshake, application data can be sent securely.

You don't need to implement TLS yourself.

Your browser, operating system, web server, and TLS libraries handle it.


## 28. Putting TCP + TLS + HTTP Together

This is an important mental model.

For:

```text
https://example.com
```

you can think of the stack as:

```text
HTTP
  ↓
TLS
  ↓
TCP
  ↓
IP
  ↓
Network
```

At the receiving side:

```text
Network
  ↓
IP
  ↓
TCP
  ↓
TLS
  ↓
HTTP
```

This is a simplified model, but it is extremely useful.


## 29. What Happens When You Type https://example.com?

Now let's put everything together.

This is the most important section.

Suppose you type:

```text
https://example.com
```

into your browser.


### Step 1 — Browser Parses the URL

The browser sees:

```text
https://example.com
```

It determines:

```text
Scheme: https
Host: example.com
Port: 443
Path: /
```

Because HTTPS normally uses port `443`.


### Step 2 — Browser Checks Its Cache

The browser may already know information about:

```text
example.com
```

It can have cached DNS information, connections, resources, etc.

If the required information is available, some work may be avoided.

If not, it continues.


### Step 3 — DNS Lookup

The browser needs the server's IP address.

It asks the configured DNS resolver to resolve:

```text
example.com
```

Conceptually:

```text
example.com
     ↓
    DNS
     ↓
93.184.216.34
```

The actual address can vary depending on DNS configuration, infrastructure, geographic routing, CDN usage, and other factors.

The important point is:

```text
Domain name
     ↓
    DNS
     ↓
IP address
```


### Step 4 — Determine the Destination

Now the browser knows something like:

```text
example.com
↓
93.184.216.34
```

and HTTPS normally means:

```text
Port 443
```

So the destination is conceptually:

```text
93.184.216.34:443
```


### Step 5 — Establish a Network Connection

The browser needs to communicate with that destination.

With traditional HTTP/1.1 or HTTP/2 over TLS, this involves establishing a TCP connection.

Conceptually:

```text
Browser
   │
   │ TCP connection
   ↓
Server
```

The TCP handshake occurs.

```text
SYN
   ↓
SYN + ACK
   ↓
ACK
```

Now a TCP connection exists.

> Note: Modern HTTP/3 uses QUIC over UDP rather than TCP. You can learn that later. For now, TCP is the essential foundation for understanding traditional web traffic.


### Step 6 — TLS Handshake

Because we're using:

```text
https://
```

the connection needs TLS security.

The browser and server perform a TLS handshake.

Conceptually:

```text
Browser
   │
   │ TLS negotiation
   ↓
Server
   │
   │ Certificate
   ↓
Browser
   │
   │ Verify certificate
   ↓
Secure cryptographic connection
```

Now the browser has a secure channel to the server.


### Step 7 — Browser Sends HTTP Request

Now HTTP can be sent through the secure connection.

The browser might send something conceptually like:

```http
GET / HTTP/1.1
Host: example.com
Accept: text/html
User-Agent: ...
```

Notice the important distinction:

```text
TCP
```

provides the reliable transport.

```text
TLS
```

protects the connection.

```text
HTTP
```

defines the actual web request.


### Step 8 — Request Travels Through the Internet

The request is broken into network data and transported across networks.

Conceptually:

```text
Browser
   ↓
Home network
   ↓
Router
   ↓
  ISP
   ↓
Internet
   ↓
Server network
   ↓
Server
```

There can be many routers and networks involved.

You don't normally need to know their exact path.

The networking infrastructure uses routing information to move packets toward the destination.


### Step 9 — Server Receives the Request

Eventually the request reaches the server.

The server might have an architecture such as:

```text
Internet
   ↓
Load Balancer / Reverse Proxy
   ↓
Web Server
   ↓
Application
```

For example:

```text
Nginx
   ↓
Node.js
   ↓
Express
```

or:

```text
Cloud Load Balancer
   ↓
Application Server
   ↓
Java/Spring
```

### Step 10 — Web Server Processes the Request

The web server/application looks at:

```http
GET /
```

and determines what should happen.

For example:

```text
GET /
 ↓
Router
 ↓
Homepage handler
 ↓
Application logic
 ↓
Database/cache if necessary
 ↓
Generate response
```

For a simple static website, the server might simply return:

```text
index.html
```

For a dynamic application, it may execute application code.


### Step 11 — Application May Talk to a Database

A full-stack application often does something like:

```text
HTTP Request
     ↓
Backend
     ↓
Business Logic
     ↓
Database
     ↓
Backend
     ↓
HTTP Response
```

For example:

```text
GET /products
     ↓
Backend
     ↓
SELECT * FROM products
     ↓
PostgreSQL
     ↓
Product rows
     ↓
Backend
     ↓
JSON response
```

The browser usually doesn't communicate directly with the database.

A common architecture is:

```text
Browser
   ↓
Backend/API
   ↓
Database
```

not:

```text
Browser
   ↓
Database
```

### Step 12 — Server Sends HTTP Response

The server might respond:

```http
HTTP/1.1 200 OK
Content-Type: text/html
```

followed by HTML:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Example</title>
  </head>

  <body>
    <h1>Hello</h1>
  </body>
</html>
```

The response travels back through the connection.

Conceptually:

```text
Server
   ↓
Internet
   ↓
Router
   ↓
Home network
   ↓
Browser
```

### Step 13 — Browser Receives the Response

The browser receives the HTTP response.

It sees:

```text
Status: 200 OK
Content-Type: text/html
```

Then it processes the HTML.

Conceptually:

```text
HTML
 ↓
Parse HTML
 ↓
DOM
 ↓
CSS
 ↓
CSSOM / styles
 ↓
Layout
 ↓
Paint
 ↓
Display
```

### Step 14 — Browser Requests Additional Resources

The HTML might contain:

```html
<link rel="stylesheet" href="/styles.css">
<script src="/app.js"></script>
<img src="/logo.png">
```

The browser then makes additional requests:

```text
GET /styles.css
GET /app.js
GET /logo.png
```

So loading one webpage can involve many HTTP requests.

Conceptually:

```text
                    ┌── GET /
Browser ────────────┼── GET /styles.css
                    ├── GET /app.js
                    ├── GET /logo.png
                    └── GET /favicon.ico
```

### Step 15 — Browser Renders the Page

The browser processes the downloaded resources and eventually displays the webpage.

Very simplified:

```text
HTML
 ↓
DOM

CSS
 ↓
Styles

JavaScript
 ↓
Behavior / DOM updates

Everything
 ↓
Layout
 ↓
Paint
 ↓
Screen
```

You see:

```text
┌─────────────────────────────┐
│         Example.com         │
├─────────────────────────────┤
│                             │
│           Hello             │
│                             │
└─────────────────────────────┘
```


## 30. Responsibilities of Each Layer

Memorize this mental model:

| Layer / Concept | Main Responsibility                   |
| --------------- | ------------------------------------- |
| DNS             | Find the IP address for a domain      |
| IP              | Address and route packets             |
| TCP             | Reliable ordered transport            |
| TLS             | Encryption, authentication, integrity |
| HTTP            | Web request/response protocol         |
| Web server      | Receive and serve requests            |
| Application     | Business logic                        |
| Database        | Persistent data storage               |
| Browser         | Parse, execute, render                |

This table is more useful than memorizing networking terminology without understanding what each part does.