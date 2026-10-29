
# Running Rust Programs
Rust programs can be executed in two main ways: **directly using `rustc`** for single files, or using **Cargo** for proper Rust projects.

## 1. Running a Single Rust File

For a simple Rust program, you don't need Cargo.
Create a file such as:
```rust
// hello.rs
fn  main() {
println!("Hello, Rust!");
}
```
Compile it using the Rust compiler (rustc):
```bash
rustc  hello.rs
```
This creates an executable:
```bash
hello
```
Run it:
```bash
./hello
```
You can also compile and run it in a single command:
```bash
rustc  hello.rs && ./hello
```
To store executables in a separate directory
```
mkdir bin
rustc hello.rs -o bin/hello
```
By this we can just add the `bin` directory to `.gitignore` to prevent pushing executables to git.

Why does this work?

`rustc` is the official Rust compiler. It takes the Rust source code (.rs) and compiles it into a native executable.

The flow is:

```
hello.rs
↓
rustc
↓
hello (executable)
↓
./hello
↓
Program runs
```

This approach is useful for:
* Learning basic Rust syntax
* Practicing small programs
* Testing a single Rust file

Writing simple standalone programs

## 2. Running a Rust Project with Cargo

For larger or real-world Rust projects, Cargo is normally used.

Cargo is Rust's build system and package manager. It handles things such as:

* Creating projects
* Compiling code
* Managing dependencies (crates)
* Running programs
* Running tests
* Building release versions

Create a new project:
```
cargo new my_project
```
 
This creates:
```
my_project/
├── Cargo.toml
└── src/
└── main.rs
```
  
The `Cargo.toml` file contains project metadata and dependencies.

The Rust program is usually placed in:  
```
src/main.rs
```
 
Run the project:
```
cd my_project
cargo run
```

Cargo will compile the program and then execute it.

The flow is:
```
src/main.rs
↓
Cargo
↓
rustc
↓
executable
↓
Program runs
```
## 3. Deep Dive: Cargo Build Output & The `target/` Directory

When you build or run a Rust project using Cargo, you will notice a new directory called `target/` created in your project root.

### Initial vs. Post-Build Project Layout

Suppose you create a new project and run it:
```bash
cargo new hello
cd hello
cargo run
```

Initially, your project directory contains only your source files and configuration:
```
hello/
├── Cargo.toml
└── src/
    └── main.rs
```

After executing `cargo run`, Cargo compiles your project and generates build artifacts. The directory structure becomes:
```
hello/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
└── target/
    ├── debug/
    │   ├── build/
    │   ├── deps/
    │   ├── examples/
    │   ├── incremental/
    │   ├── .fingerprint/
    │   ├── hello
    │   └── hello.d
    └── ...
```

---

### Breakdown of `target/` & Build Files

#### 1. `target/` (Build Artifact Root)
This is where Cargo stores all generated files during compilation. 
* **Note:** You generally **never edit anything inside `target/` directly**.
* **Compilation Flow:**
  ```
  Your Source Code (src/)
            ↓
          Cargo
            ↓
         target/
            ↓
  Executables & Intermediate Build Files
  ```
* **Cleaning Build Artifacts:** You can safely delete the `target/` directory at any time to clean up disk space. Cargo will simply re-create it on the next build.
  ```bash
  cargo clean
  # or manually:
  rm -rf target
  ```

#### 2. `target/debug/`
By default, running `cargo run` or `cargo build` compiles your program in **Debug Mode** (fast compilation, extra debug checks, unoptimized binary).
* Executable output path: `target/debug/hello`
* You can run the compiled binary directly:
  ```bash
  ./target/debug/hello
  ```
* **Behind the scenes of `cargo run`:**
  ```
  Cargo → Compiles Rust Code → Saves binary to target/debug/hello → Executes binary
  ```

#### 3. `target/release/`
When preparing your code for production, compile with optimization flags:
```bash
cargo build --release
```
Cargo creates `target/release/` and places the optimized binary inside (`target/release/hello`).

| Mode | Command | Output Directory | Compilation Time | Execution Speed |
| --- | --- | --- | --- | --- |
| **Debug** | `cargo run` / `cargo build` | `target/debug/` | Fast | Normal / Unoptimized |
| **Release** | `cargo build --release` | `target/release/` | Slower (Optimized) | Maximum Speed |

#### 4. `target/debug/deps/`
`deps` stands for **dependencies**. 
* When your project depends on external Rust libraries (crates) listed in `Cargo.toml` (e.g., `serde = "1"`), Cargo compiles those crates first and stores their compiled outputs in `target/debug/deps/`.
* You do not need to interact with these files directly.

#### 5. `target/debug/build/`
Contains artifacts produced by **build scripts** (`build.rs`).
* Certain crates run custom build scripts to generate code, detect C libraries, configure compiler options, or set up native build steps.
* Cargo manages script outputs inside this folder.

#### 6. `target/debug/incremental/`
Stores data for **incremental compilation**.
* Instead of recompiling your entire project on every small change, Rust reuses unchanged compiled chunks.
* **Flow:**
  * *First Build:* Compile full project.
  * *Small Change in `main.rs`:* Recompile only modified parts using cached data from `incremental/`.

#### 7. `target/debug/.fingerprint/`
Contains compiler fingerprints and dependency tracking data.
* Cargo uses these files to answer:
  * What has already been compiled?
  * Has source code or configuration changed?
  * Do dependencies need to be rebuilt?

#### 8. `Cargo.lock` (Dependency Lockfile)
Located in the project root (outside `target/`).
* While `Cargo.toml` specifies broad dependency version requirements (e.g., `serde = "1"`), `Cargo.lock` records the **exact versions** of every crate in your dependency tree.
* **Dependency Tree Resolution Example:**
  ```
  serde 1.x → serde_derive → proc-macro2 → quote → syn
  ```
* `Cargo.lock` ensures that builds are **100% deterministic and reproducible** across different machines and environments.

---

### Why are there so many generated files?

Even if your project has a single `main.rs` file, adding external libraries expands your project into a dependency tree:
```
Your Application
       │
       └── library_A
              │
              ├── dependency_1
              └── dependency_2
```
Cargo compiles every dependency layer and caches intermediate artifacts under `target/` so subsequent builds are fast.

---

## 4. Git, Version Control & Cargo vs. `uv`

### Should you commit `target/` to Git?
**No.** You should **never** commit the `target/` directory to version control.
* Build artifacts are binary, large, machine-specific, and can easily be regenerated using `cargo build`.

A standard Rust Git repository looks like this:
```
my-project/
├── .gitignore
├── Cargo.toml
├── Cargo.lock       ← Commit for binaries/applications
└── src/
    └── main.rs
```

### Why doesn't `cargo new` always create a `.gitignore` file?
Cargo is VCS-agnostic by design. It does not assume you are using Git; it can work with Mercurial, Fossil, or no version control system at all.

You can specify the version control system explicitly when creating a project:
```bash
cargo new my-project --vcs git   # Automatically initializes Git & .gitignore
cargo new my-project --vcs none  # Creates project without VCS files
```
*If a project lacks `.gitignore`, you can create it manually:*
```bash
echo "/target" > .gitignore
```

### Clarification: `Cargo` vs. `uv` (Python Package Manager)
You might wonder why `uv` (a Python tool written in Rust) automatically manages `.gitignore` while Cargo treats VCS as an option.

```
                  Rust (Language)
                         │
        ┌────────────────┴────────────────┐
        │                                 │
      Cargo                              uv
(Rust Ecosystem Tool)           (Python Ecosystem Tool)
```

* **`uv`**: Built specifically for modern Python workflows, where virtual environments (`.venv`) and build caches need automatic git exclusion by default.
* **`Cargo`**: Built as a general-purpose build system for Rust, designed to support multiple version control backends via `--vcs`.

### Quick Reference: Crucial Rust Files & Commands

**Core Files to Track:**
* `src/` — Your actual Rust source code.
* `Cargo.toml` — Project configuration and dependency specification.
* `Cargo.lock` — Exact locked version manifest (commit this for application projects).
* Ignore `target/` using `.gitignore`.

**Essential Cargo Commands:**
| Command | Action |
| --- | --- |
| `cargo run` | Build and run in **debug** mode |
| `cargo build` | Build binary without running |
| `cargo build --release` | Create optimized **release** binary |
| `cargo clean` | Delete the `target/` directory to free disk space |

---

## 5. `rustc` vs Cargo

| `rustc` | Cargo |
| --- | --- |
| Rust compiler | Rust build system + package manager |
| Good for single files | Good for complete projects |
| Compiles `.rs` files directly | Manages the whole project |
| No project structure required | Uses a standard project structure |
| Dependency management is manual | Dependency management is built in |
| Simple and lightweight | Recommended for real projects |

