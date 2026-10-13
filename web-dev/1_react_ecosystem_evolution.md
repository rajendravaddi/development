# React Web Development: From React to Vite, TypeScript, and Next.js

## Introduction

There are several ways to build a website using React, and terms such as **React**, **Vite**, **TypeScript**, and **Next.js** can initially be confusing because they are not all alternatives to one another.

They solve different problems and can be combined.

A useful mental model is:

```text
React       = UI library
TypeScript  = programming language/superset used with JavaScript
Vite        = development/build tooling
Next.js     = full web application framework built around React
```

This document explains the evolution and relationship between these technologies, what problems they solve, and why modern React development commonly uses combinations such as **React + Vite + TypeScript** or **Next.js + TypeScript**.

---

# 1. React — The Foundation

## What is React?

React is a JavaScript library for building user interfaces.

Instead of writing a large page as one piece of HTML and JavaScript, React encourages developers to break the UI into reusable **components**.

For example:

```jsx
function App() {
  return <h1>Hello</h1>;
}
```

A larger application might be divided into:

```text
App
├── Navbar
├── Sidebar
├── ProductList
│   └── ProductCard
└── Footer
```

Each part can be implemented as a reusable React component.

## What problem did React solve?

As web applications became more interactive, manually updating HTML using JavaScript became increasingly difficult.

React introduced a component-based and declarative approach.

Instead of manually telling the browser:

```text
Find this element
Change its text
Add this element
Remove that element
Update this attribute
```

you describe what the UI should look like based on the application's state.

React then manages the UI updates.

### Core idea

```text
Application State
       ↓
     React
       ↓
      UI
```

## What React provides

React primarily provides:

- Components
- JSX
- State
- Props
- Event handling
- Hooks
- Declarative UI development
- Efficient UI updates

However, React itself is mainly concerned with the **UI layer**.

It does not automatically provide everything needed to build a complete production website.

For example, you may need separate solutions for:

- Routing
- Development server
- Bundling
- Production builds
- Server-side rendering
- Backend/API functionality
- SEO infrastructure
- Deployment

This led to additional tools and frameworks.

---

# 2. Create React App — Making React Easier to Start

## What was Create React App?

Create React App (CRA) was a popular way to create a React project without manually configuring the development tooling.

Example:

```bash
npx create-react-app my-app
```

Instead of manually configuring tools such as:

```text
React
Webpack
Babel
Development server
Build configuration
```

CRA provided a preconfigured setup.

## What problem did it solve?

React itself did not provide a complete project setup.

A developer could otherwise spend significant time configuring:

```text
React
   +
Webpack
   +
Babel
   +
Development server
   +
Production build
```

CRA packaged much of this configuration into a simpler starting point.

## Why did the industry move away from it?

As modern web development evolved, CRA's tooling became comparatively heavy and slow for development.

Developers wanted:

- Faster startup
- Faster module updates
- Modern build tooling
- Better development experience
- More flexible configuration

This contributed to the popularity of tools such as Vite.

---

# 3. Vite — Modern Development and Build Tooling

## What is Vite?

Vite is a modern development and build tool.

It is not a replacement for React itself.

Instead, Vite can be used to create and develop React applications.

For example:

```bash
npm create vite@latest
```

You can then select:

```text
React
```

or:

```text
React + TypeScript
```

## What problem did Vite solve?

Vite focuses heavily on improving the development experience.

Traditional tooling often involved a large bundling process before the development server could serve the application.

Vite uses modern browser capabilities and a fast development architecture to provide very fast startup and updates.

The result is generally:

```text
Edit code
   ↓
Very fast update
   ↓
See the change in browser
```

This is especially useful for larger applications.

## What Vite provides

Vite provides things such as:

- Development server
- Fast Hot Module Replacement (HMR)
- Production build process
- Modern JavaScript/TypeScript handling
- Plugin system
- Development tooling

But Vite is not itself a complete web application framework.

It does not turn React into a full-stack framework.

Think of it as:

```text
React
  +
Vite
  =
React application with modern development/build tooling
```

---

# 4. JavaScript vs TypeScript

TypeScript is a different concept from Vite.

It is not a competing framework.

You can use React with either JavaScript or TypeScript.

For example:

```text
React + JavaScript
```

or:

```text
React + TypeScript
```

You can also combine TypeScript with Vite:

```text
React + Vite + TypeScript
```

## What is TypeScript?

TypeScript is a superset of JavaScript that adds static typing and other developer-oriented features.

For example:

```ts
function add(a: number, b: number): number {
  return a + b;
}
```

The types indicate that:

- `a` should be a number
- `b` should be a number
- the function returns a number

## What problem does TypeScript solve?

JavaScript is dynamically typed.

This provides flexibility, but in large applications it can make certain mistakes harder to detect before runtime.

TypeScript can detect many type-related problems during development.

For example:

```ts
function add(a: number, b: number) {
  return a + b;
}

add(10, "20");
```

A TypeScript development environment can flag this as a type error.

## Why is TypeScript useful with React?

React applications can contain:

- Many components
- Complex props
- API responses
- State objects
- Large teams
- Shared interfaces/types

TypeScript can make these structures easier to understand and maintain.

---

# 5. React + Vite + TypeScript

These technologies work together.

A common modern setup is:

```text
React
   ↓
Builds the UI
   +
Vite
   ↓
Development server + build tooling
   +
TypeScript
   ↓
Static typing
```

So:

```text
React + Vite + TypeScript
```

does not mean three competing technologies.

Each has a different responsibility.

### Example

```text
                    React
                     │
                 UI Components
                     │
          ┌──────────┴──────────┐
          │                     │
        Vite                TypeScript
          │                     │
 Development/build          Type checking
 tooling
```

This is a very common way to build client-side React applications.

---

# 6. The Need for a React Framework

As applications became more complex, developers wanted more than just a UI library and build tool.

A production web application may need:

```text
UI
Routing
SEO
Server rendering
Data fetching
Authentication
Backend/API logic
Caching
Performance optimization
Deployment
```

With basic React, developers could assemble these pieces independently.

That provides flexibility, but also creates more architectural decisions and configuration.

This created demand for React frameworks.

One of the most popular examples is **Next.js**.

---

# 7. Next.js — React Application Framework

## What is Next.js?

Next.js is a web application framework built around React.

The important distinction is:

```text
React
= UI library
```

while:

```text
Next.js
= framework for building complete web applications using React
```

Next.js adds many capabilities around React.

A simplified view is:

```text
Next.js
├── React
├── Routing
├── Server Components
├── Client Components
├── Server-side rendering
├── Static generation
├── Data fetching
├── Route handlers / API capabilities
├── Layouts
├── Loading and error UI
├── Metadata / SEO support
├── Image optimization
├── Font optimization
└── Other performance and deployment features
```

---

# 8. Next.js at the UI Side

At the UI level, you are still writing React.

For example:

```tsx
function Home() {
  return (
    <div>
      <h1>Welcome</h1>
      <p>This is my website.</p>
      <button>Get Started</button>
    </div>
  );
}
```

The important difference is that Next.js provides additional structure around those React components.

## File-based routing

For example:

```text
app/
├── page.tsx
├── about/
│   └── page.tsx
└── contact/
    └── page.tsx
```

This can represent:

```text
app/page.tsx
        ↓
      /

app/about/page.tsx
        ↓
      /about

app/contact/page.tsx
        ↓
      /contact
```

This is called **file-based routing**.

You create files and folders according to the application's URL structure.

## Layouts

Next.js also provides layouts that can be shared across pages.

For example:

```text
app/
├── layout.tsx
├── page.tsx
├── about/
│   └── page.tsx
└── contact/
    └── page.tsx
```

The layout can contain common UI such as:

```text
Navbar
Main content
Footer
```

while individual pages provide their own content.

## Loading and error UI

Next.js also provides conventions for handling loading and error states at the route level.

This helps organize application UI without manually building the entire structure from scratch.

---

# 9. Next.js at the Server Side

One of the biggest differences between a basic React client-side application and Next.js is that Next.js can execute application code on the server.

A simplified architecture is:

```text
Browser
   │
   │ Request
   ↓
Next.js Server
   │
   ├── React rendering
   ├── Data fetching
   ├── Server components
   ├── Authentication logic
   ├── Route handlers
   │
   ↓
Database / External APIs
   │
   ↓
Response
   │
   ↓
Browser
```

For example, a server-side component can retrieve data from a database or external API and use that data while rendering the page.

This can reduce the amount of work that must happen in the browser.

---

# 10. Server Components and Client Components

Modern Next.js distinguishes between server-side and client-side React components.

## Server Components

Server Components can execute on the server.

They are useful for things such as:

- Fetching data
- Accessing server-side resources
- Rendering content
- Keeping server-only logic away from the browser

## Client Components

Client Components are used when the UI needs browser-side interactivity.

For example:

```text
Button clicks
Forms
Browser APIs
Interactive state
Animations requiring client-side logic
```

A simplified model is:

```text
Next.js
│
├── Server Components
│      └── Server
│
└── Client Components
       └── Browser
```

This gives developers more control over where application code executes.

---

# 11. What Problem Did Next.js Solve?

The fundamental problem was:

> React was excellent for building UI, but developers needed to assemble many additional technologies and architectural patterns to build a complete production web application.

Before a framework, a project might look conceptually like:

```text
React
+
React Router
+
Build tooling
+
SSR solution
+
SEO solution
+
API solution
+
Code splitting
+
Performance configuration
+
Deployment configuration
+
Other libraries
```

Next.js brings many of these concerns together.

Conceptually:

```text
Before:

React
 ├── Router
 ├── SSR solution
 ├── Build tooling
 ├── API solution
 ├── SEO configuration
 └── Performance configuration


With Next.js:

Next.js
 └── React
      ├── Routing
      ├── Rendering
      ├── Server capabilities
      ├── Optimization
      ├── Metadata
      └── More application infrastructure
```

The exact capabilities and architecture have evolved over time, but this is the main reason the framework became important.

---

# 12. Why Did Next.js Become Popular?

Several factors contributed to its popularity.

## 12.1 Server-side rendering

Next.js supports rendering React applications on the server.

This can improve initial content delivery and can be useful for pages where search engines need readily available HTML.

---

## 12.2 Static generation

Pages can also be generated ahead of time when appropriate.

This can provide fast delivery for content that does not need to be generated on every request.

---

## 12.3 File-based routing

Instead of manually defining every route, the project structure can represent the URL structure.

Example:

```text
app/
├── page.tsx
├── products/
│   ├── page.tsx
│   └── [id]/
│       └── page.tsx
```

This makes larger applications easier to organize.

---

## 12.4 Server + UI in one framework

Next.js allows developers to build both UI and server-side functionality within the same application.

For example:

```text
Next.js application
│
├── UI
│   ├── Pages
│   ├── Components
│   └── Layouts
│
└── Server
    ├── Data fetching
    ├── Route handlers
    ├── Authentication logic
    └── Database interaction
```

This reduces the need to always maintain completely separate frontend and backend projects.

---

## 12.5 Performance features

Next.js provides features for optimizing applications, including mechanisms around:

- Code splitting
- Images
- Fonts
- Caching
- Server rendering
- Static generation

The goal is to make common performance patterns easier to implement.

---

## 12.6 Developer ecosystem

Next.js also benefited from a large ecosystem, documentation, community, tutorials, libraries, and company adoption.

Its close relationship with Vercel also made deployment particularly straightforward for many projects.

---

# 13. The Overall Evolution

The evolution can be understood like this:

```text
React
  │
  │ "I need a better way to build interactive UIs."
  ↓
React
  │
  │ "I need project/build tooling."
  ↓
Create React App
  │
  │ "I need faster, more modern development tooling."
  ↓
Vite
  │
  │ "I need typing for larger applications."
  ↓
TypeScript
  │
  │ "I need more than just a UI library."
  ↓
React Frameworks
  │
  ↓
Next.js and other frameworks
```

Important: this is a conceptual evolution, not a strict replacement chain.

For example:

```text
Vite did not replace React.
TypeScript did not replace React.
Next.js did not replace React.
```

Instead, they operate at different layers.

---

# 14. How the Technologies Fit Together

A useful layered model is:

```text
┌─────────────────────────────────────┐
│             Application             │
│                                     │
│             Next.js                 │
│                                     │
│  Routing / Server / Rendering / UI  │
├─────────────────────────────────────┤
│               React                 │
│                                     │
│        Components / JSX / State     │
├─────────────────────────────────────┤
│             TypeScript              │
│                                     │
│          Static type checking       │
├─────────────────────────────────────┤
│          Build / Dev Tooling        │
│                                     │
│               Vite                  │
└─────────────────────────────────────┘
```

However, Next.js has its own build and development tooling, so **Vite is not normally a required part of a Next.js project**.

This is an important distinction.

---

# 15. Common Modern Choices

## Option A: React + Vite + JavaScript

```text
React
+
Vite
+
JavaScript
```

Good for a straightforward client-side React application.

---

## Option B: React + Vite + TypeScript

```text
React
+
Vite
+
TypeScript
```

A common modern choice for client-side applications.

It gives:

- React UI
- Fast development/build tooling
- Static typing

---

## Option C: Next.js + TypeScript

```text
Next.js
+
TypeScript
```

A common choice when building a larger web application that benefits from framework-level features such as:

- Routing
- Server rendering
- Server Components
- Server-side data fetching
- API/route handlers
- SEO/metadata support
- Performance features

---

# 16. Simple Comparison

| Technology | What it is | Main problem it solves |
|---|---|---|
| React | UI library | Building reusable interactive UIs |
| Create React App | React project tooling | Easy initial React setup |
| Vite | Development/build tool | Fast modern development and builds |
| TypeScript | Typed JavaScript language | Detecting many errors during development |
| Next.js | React web framework | Building complete production web applications |

---

# 17. The Most Important Mental Model

Do not think:

```text
React vs Vite vs TypeScript vs Next.js
```

as if they are four competing technologies.

Instead think:

```text
What am I using to build my application?

React
    ↓
How am I developing/building it?

Vite
    ↓
Do I want static typing?

TypeScript
    ↓
Do I want a full web application framework?

Next.js
```

And in practice, you might choose:

```text
Client-side application:

React + Vite + TypeScript
```

or:

```text
Full web application:

Next.js + TypeScript
```

---

# 18. Final Summary

The progression is easiest to remember with four sentences:

### React

> **React gives you a component-based way to build the UI.**

### Vite

> **Vite gives you fast modern development and build tooling for applications such as React apps.**

### TypeScript

> **TypeScript adds static typing to JavaScript, helping developers catch many mistakes during development.**

### Next.js

> **Next.js builds a complete web application framework around React, adding routing, server-side capabilities, rendering options, optimization, and other application-level features.**

The central idea is:

```text
React
"I can build the UI."

React + Vite
"I can develop and build the UI efficiently."

React + Vite + TypeScript
"I can develop a typed React application efficiently."

Next.js
"I can build the broader web application around React,
including both UI and server-side capabilities."
```

This is why **React + Vite + TypeScript** and **Next.js + TypeScript** are common combinations today, depending on the type of application being built.

# 19. Creating a Next.js Application

For a React + Vite application, you can use:

```bash
npm create vite@latest
```

and then select:

```text
React
React + TypeScript
```

For a Next.js application, the equivalent project creation tool is **Create Next App**.

## Basic command

```bash
npx create-next-app@latest
```

It will ask configuration questions such as:

```text
Would you like to use TypeScript?        Yes
Would you like to use ESLint?            Yes
Would you like to use Tailwind CSS?      Yes/No
Would you like to use App Router?        Yes
```

You can also provide the project name directly:

```bash
npx create-next-app@latest my-app
```

Then enter the project directory:

```bash
cd my-app
```

Start the development server:

```bash
npm run dev
```

The application will normally be available at:

```text
http://localhost:3000
```

## React + Vite vs Next.js project creation

```text
React + Vite:

npm create vite@latest


Next.js:

npx create-next-app@latest
```

The important distinction is:

```text
create-vite
    ↓
Creates a Vite-based project
    ↓
You choose React, TypeScript, etc.


create-next-app
    ↓
Creates a Next.js project
    ↓
You configure TypeScript, ESLint, Tailwind,
App Router, and other Next.js options
```

So **Create Next App** is the standard scaffolding tool for starting a Next.js application.
