# Python uv

## 1. Introduction to uv

### 1.1 What is uv?

`uv` is an extremely fast Python package and project manager developed by Astral and written in Rust.

It provides a unified solution for managing Python versions, virtual environments, dependencies, projects, and command-line tools.

Instead of using multiple tools for different Python development tasks, we can use `uv` to manage most of them through a single command-line interface.

### 1.2 Why was uv created?

Traditional Python development often involves multiple tools:

- `pip` for installing packages.
- `venv` for creating virtual environments.
- `pip-tools` for dependency locking.
- `pyenv` for managing Python versions.
- `pipx` for installing Python CLI tools.

Managing these tools separately can make Python project management complicated.

`uv` brings these capabilities together into a single tool with a consistent interface and fast dependency resolution.

### 1.3 Key Features of uv

- **Fast package installation:** Uses efficient dependency resolution and caching.
- **Python version management:** Downloads and manages multiple Python versions.
- **Virtual environment management:** Creates and manages isolated environments.
- **Project management:** Initializes projects and manages dependencies.
- **Dependency locking:** Uses `uv.lock` to maintain consistent dependency versions.
- **Script execution:** Runs standalone Python scripts with isolated dependencies.
- **Tool management:** Installs and runs Python command-line tools.
- **pip compatibility:** Provides a familiar `uv pip` interface.
- **Workspace support:** Manages multiple related Python packages within one workspace.

### 1.4 uv vs Traditional Python Tools

| Traditional Tool | uv Equivalent |
|---|---|
| `pip` | `uv pip` / `uv add` |
| `venv` | `uv venv` |
| `pip-tools` | `uv pip compile` / `uv lock` |
| `pyenv` | `uv python` |
| `pipx` | `uv tool` |

**Important:** These are functional equivalents, not exact replacements. Their commands and behavior can differ.

---

## 2. Installing uv

uv supports Windows, Linux, and macOS.

### 2.1 Windows

Open PowerShell and execute:

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

### 2.2 Linux/macOS

Open a terminal and execute:

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 2.3 Alternative Installation Methods

uv can also be installed using package managers such as:

**Windows (WinGet):**

```powershell
winget install --id astral-sh.uv -e
```

**macOS (Homebrew):**

```bash
brew install uv
```

**Python pip:**

```bash
pip install uv
```

The standalone installer is generally a convenient option because uv does not require an existing Python installation.

### 2.4 Verify Installation

```bash
uv --version
```

This displays the installed uv version.

---

## 3. Managing Python Versions with uv

uv can download, install, and manage multiple Python versions without requiring a separate Python version manager.

### 3.1 Listing Available Python Versions

```bash
uv python list --all-versions
```

This displays available Python versions.

### 3.2 Listing Installed Python Versions

```bash
uv python list --only-installed
```

This displays Python versions installed and managed by uv, along with other installations detected by uv when applicable.

### 3.3 Installing a Specific Python Version

```bash
uv python install 3.12
```

This downloads and installs Python 3.12.

We can also install a specific patch version:

```bash
uv python install 3.12.7
```

### 3.4 Finding an Installed Python Version

```bash
uv python find 3.12
```

This locates a suitable Python 3.12 interpreter.

### 3.5 Pinning a Python Version

```bash
uv python pin 3.12
```

This creates or updates a `.python-version` file in the current directory.

Example:

```text
3.12
```

uv uses this file as the default Python version for the project.

### 3.6 Removing a Python Version

```bash
uv python uninstall 3.12
```

This uninstalls the specified uv-managed Python version.

**Note:** uv can also use Python installations managed outside uv. Uninstalling a uv-managed version does not remove an independently installed system Python.

---

## 4. Understanding uv Projects and Project Structure

A uv project is a Python project whose dependencies and configuration are managed through a `pyproject.toml` file.

uv uses several files and directories to manage the project.

### 4.1 Understanding pyproject.toml

`pyproject.toml` is the central configuration file for a Python project.

It contains information such as:

- Project name.
- Project version.
- Python version requirements.
- Dependencies.
- Build system configuration.
- Development dependency groups.

Example:

```toml
[project]
name = "my-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "requests>=2.32.0",
]
```

uv updates this file when we add or remove project dependencies.

### 4.2 Understanding .python-version

This file specifies the default Python version for the project.

Example:

```text
3.12
```

It helps uv select the appropriate Python interpreter when creating the project's virtual environment.

### 4.3 Understanding .venv

`.venv` is the project's virtual environment directory.

uv automatically creates it when needed during project operations such as `uv sync` or `uv run`.

It contains the environment-specific Python interpreter and installed dependencies.

### 4.4 Understanding uv.lock

`uv.lock` is the dependency lockfile.

It records the exact resolved dependency versions and other information required to reproduce the project's dependency resolution.

Unlike `pyproject.toml`, which specifies dependency requirements, `uv.lock` records the resolved versions.

It should be committed to version control and should generally not be edited manually.

### 4.5 How These Files Work Together

A typical uv project looks like this:

```text
my-project/
│
├── .venv/
├── .python-version
├── .gitignore
├── pyproject.toml
├── uv.lock
├── README.md
│
└── src/
    └── my_project/
        └── __init__.py
```

| File/Directory | Purpose |
|---|---|
| `.venv/` | Project virtual environment |
| `.python-version` | Default Python version |
| `pyproject.toml` | Project configuration and dependencies |
| `uv.lock` | Exact resolved dependency information |
| `README.md` | Project documentation |
| `src/` | Python source code for a packaged project |

The `src/` directory is specific to this example's packaged project layout. Non-packaged projects can use a flat structure.

---

## 5. Creating Different Types of uv Projects

uv supports creating packaged and non-packaged projects.

Understanding the difference helps us choose the right project structure.

### 5.1 Packaged Project

A packaged project is a Python project that can be built and installed as a package.

It is useful when:

- Developing reusable Python libraries.
- Creating distributable packages.
- Using a `src/` directory structure.
- Defining command-line entry points.

#### Creating a Packaged Project

```bash
uv init --package my-project
```

Navigate into the project:

```bash
cd my-project
```

#### Directory Structure

```text
my-project/
│
├── .python-version
├── .gitignore
├── pyproject.toml
├── README.md
│
└── src/
    └── my_project/
        └── __init__.py
```

#### Understanding __init__.py

`__init__.py` identifies a regular Python package directory.

It can contain package initialization code and functions or classes exposed by the package.

Example:

```python
def greet():
    print("Hello from my-project!")
```

#### Understanding [build-system]

A packaged project includes a build system configuration in `pyproject.toml`.

Example:

```toml
[build-system]
requires = ["uv_build>=0.10.0,<0.11"]
build-backend = "uv_build"
```

The build system defines how the project is built into distributable formats such as wheels and source distributions.

The exact version constraint may differ depending on the uv version used to initialize the project.

#### Running the Project

```bash
uv run python -c "from my_project import greet; greet()"
```

uv ensures that the project environment is synchronized and that the project package is available for import.

### 5.2 Non-Packaged Project

A non-packaged project is a Python project that manages dependencies without building and installing the project itself as a package.

It is useful for:

- Standalone applications.
- Scripts.
- Learning projects.
- Simple backend applications.
- Projects using a flat directory structure.

#### Creating a Non-Packaged Project

```bash
uv init --no-package my-project
```

Navigate into the project:

```bash
cd my-project
```

#### Directory Structure

```text
my-project/
│
├── .python-version
├── .gitignore
├── pyproject.toml
├── README.md
└── main.py
```

#### Understanding the Project Configuration

A non-packaged project's `pyproject.toml` contains project information and dependencies but does not declare a build system for installing the project itself.

Example:

```toml
[project]
name = "my-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []
```

#### Running the Project

```bash
uv run main.py
```

uv installs the project's dependencies into its environment and executes the script.

**Important:** The project itself is not installed as a package. Its external dependencies are still managed normally.

### 5.3 Packaged vs Non-Packaged Projects

| Feature | Packaged | Non-Packaged |
|---|---|---|
| Creation | `uv init --package` | `uv init --no-package` |
| Directory structure | `src/` layout by default | Flat layout |
| Build system | Included | Not included by default |
| Project installation | Yes | No |
| Reusable library | Suitable | Not intended |
| Simple applications | Suitable | Suitable |
| Distribution | Supported | Requires packaging configuration |

**Which one should you choose?**

- Choose a packaged project when developing a reusable library or distributable application.
- Choose a non-packaged project when developing scripts or applications that do not need to be installed as a package.

### 5.4 Initializing an Existing Directory

We can also initialize an existing directory instead of creating a new project directory.

Navigate into the directory:

```bash
cd my-existing-project
```

Initialize a project:

```bash
uv init
```

For a non-packaged project:

```bash
uv init --no-package
```

For a packaged project:

```bash
uv init --package
```

uv creates the necessary project files in the current directory.

---

## 6. Managing Project Dependencies

uv provides simple commands to add, remove, and manage project dependencies.

### 6.1 Adding a Dependency

```bash
uv add requests
```

This:

1. Adds `requests` to `pyproject.toml`.
2. Updates `uv.lock`.
3. Synchronizes the project environment.

### 6.2 Adding a Specific Version

```bash
uv add "requests==2.32.3"
```

We can also specify version constraints:

```bash
uv add "requests>=2.30,<3.0"
```

### 6.3 Adding Multiple Dependencies

```bash
uv add requests flask django
```

All specified dependencies are added to the project.

### 6.4 Removing a Dependency

```bash
uv remove requests
```

This removes the dependency from the project configuration, updates the lockfile, and synchronizes the environment.

### 6.5 Adding Development Dependencies

Some packages are required only during development.

For example, testing and formatting tools.

Add them using:

```bash
uv add --dev pytest ruff
```

These dependencies are added to the development dependency group rather than the main runtime dependencies.

### 6.6 Managing Optional Dependencies

Optional dependencies are useful when a project provides additional features that users can install separately.

Example:

```bash
uv add --optional database sqlalchemy
```

This adds SQLAlchemy to an optional dependency group named `database`.

Optional dependencies are defined under `[project.optional-dependencies]` in `pyproject.toml`.

### 6.7 Understanding Dependency Groups

Dependency groups organize packages that are not necessarily runtime dependencies.

For example:

```bash
uv add --group test pytest
uv add --group lint ruff
```

This creates separate `test` and `lint` dependency groups.

Dependency groups are useful for organizing development, testing, and quality-assurance tools.

### 6.8 Viewing the Dependency Tree

```bash
uv tree
```

This displays the project's dependency tree, including direct and transitive dependencies.

---

## 7. Understanding uv.lock and Dependency Resolution

### 7.1 What is uv.lock?

`uv.lock` is a lockfile that records the exact resolved versions of project dependencies.

For example, `pyproject.toml` might contain:

```toml
dependencies = [
    "requests>=2.30"
]
```

This allows compatible versions of Requests.

However, `uv.lock` records the exact resolved version and relevant dependency information.

This makes it easier to reproduce the same dependency environment across machines.

### 7.2 Why Do We Need a Lockfile?

Without a lockfile, installing dependencies at different times may result in different compatible package versions.

This can cause unexpected behavior.

A lockfile helps:

- Maintain consistent package versions.
- Reproduce development environments.
- Avoid unexpected dependency upgrades.
- Simplify team collaboration.
- Improve deployment consistency.

### 7.3 pyproject.toml vs uv.lock

| pyproject.toml | uv.lock |
|---|---|
| Defines project configuration | Records resolved dependencies |
| Specifies version constraints | Records exact versions |
| Usually maintained by developers | Managed by uv |
| Defines direct dependencies | Includes direct and transitive dependencies |
| Should be committed | Should be committed |

### 7.4 Generating and Updating the Lockfile

To generate or update the lockfile:

```bash
uv lock
```

uv resolves dependencies and updates `uv.lock`.

### 7.5 Upgrading All Dependencies

```bash
uv lock --upgrade
```

This attempts to update dependencies to the latest versions permitted by the project's version constraints.

### 7.6 Upgrading a Specific Dependency

```bash
uv lock --upgrade-package requests
```

This attempts to upgrade Requests while keeping other locked dependencies unchanged where possible.

### 7.7 Checking the Lockfile

```bash
uv lock --check
```

This checks whether the lockfile is up to date with the project configuration without modifying it.

### 7.8 Reproducible Installations

When another developer clones the project, they can run:

```bash
uv sync --locked
```

This installs dependencies from the existing lockfile and fails if the lockfile needs updating.

**Best practice:** Commit both `pyproject.toml` and `uv.lock` to version control.

---

## 8. Synchronizing and Running Projects

uv provides `uv sync` and `uv run` to manage the project environment and execute commands.

### 8.1 Understanding uv sync

```bash
uv sync
```

This synchronizes the project's virtual environment with its dependency specification and lockfile.

It:

- Creates `.venv` if necessary.
- Installs required dependencies.
- Installs the project itself when it is a packaged project.
- Removes extraneous packages by default.

To synchronize strictly with the existing lockfile:

```bash
uv sync --locked
```

### 8.2 Understanding uv run

`uv run` executes commands inside the project environment.

Example:

```bash
uv run main.py
```

Before execution, uv checks whether the lockfile and environment need synchronization.

### 8.3 Running Python Modules

We can execute Python modules using:

```bash
uv run python -m pytest
```

This runs pytest using the project's Python environment.

### 8.4 Running External Commands

We can also execute tools installed as project dependencies.

For example:

```bash
uv run ruff check .
```

This executes Ruff using the project's environment.

### 8.5 Running with Development Dependencies

If a project has development dependencies, we can include them using:

```bash
uv run --group dev pytest
```

This ensures that the specified dependency group is available when running the command.

### 8.6 uv run vs uv sync

| `uv run` | `uv sync` |
|---|---|
| Synchronizes the environment when needed | Explicitly synchronizes the environment |
| Executes a command | Does not execute application commands |
| Useful for running scripts and tools | Useful for preparing an environment |

Example workflow:

```bash
uv sync
uv run python main.py
```

We can also simply use:

```bash
uv run main.py
```

### 8.7 Running Commands Without uv run

We can activate the environment manually after synchronization.

```bash
uv sync
```

Windows PowerShell:

```powershell
.venv\Scripts\Activate.ps1
python main.py
```

Linux/macOS:

```bash
source .venv/bin/activate
python main.py
```

---

## 9. Using uv with Python Scripts

Apart from managing complete projects, uv can also execute standalone Python scripts with their own dependencies.

### 9.1 Running a Standalone Script

Consider a file named `hello.py`:

```python
print("Hello from uv!")
```

Run it using:

```bash
uv run hello.py
```

uv executes the script using an available Python interpreter.

Unlike project-based execution, this does not require a project `pyproject.toml`.

### 9.2 Adding Dependencies to a Script

Suppose a script requires the `requests` package.

We can add a dependency directly to the script:

```bash
uv add --script hello.py requests
```

uv adds dependency metadata to the script.

### 9.3 Understanding Inline Script Metadata (PEP 723)

The script may now contain:

```python
# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "requests",
# ]
# ///

import requests

response = requests.get("https://example.com")
print(response.status_code)
```

The metadata block specifies the Python version requirement and dependencies.

This follows PEP 723, which defines inline script metadata.

### 9.4 Running Scripts with Isolated Dependencies

Run the script:

```bash
uv run hello.py
```

uv creates or reuses an isolated environment for the script and installs its declared dependencies.

This keeps standalone script dependencies separate from project dependencies.

### 9.5 Removing Script Dependencies

```bash
uv remove --script hello.py requests
```

This removes the specified dependency from the script metadata.

### 9.6 Project-Based vs Script-Based Workflows

| Project-Based | Script-Based |
|---|---|
| Uses `pyproject.toml` | Uses inline script metadata |
| Uses project dependencies | Uses script-specific dependencies |
| Uses `uv.lock` | Can use a script lockfile |
| Suitable for applications and packages | Suitable for standalone scripts |

---

## 10. Managing Python Tools with uv

uv can install and execute Python command-line tools without manually creating virtual environments for each tool.

Examples of Python CLI tools include:

- Ruff – Python linter and formatter.
- Black – Python code formatter.
- HTTPie – Command-line HTTP client.
- Cookiecutter – Project template generator.

### 10.1 Running a Tool Temporarily

Use:

```bash
uv tool run ruff check .
```

This runs Ruff in an isolated environment without permanently installing it.

### 10.2 Understanding uvx

`uvx` is a convenient alias for:

```bash
uv tool run
```

Example:

```bash
uvx ruff check .
```

It is useful for running tools without adding them to a project's dependencies.

### 10.3 Installing Tools Permanently

```bash
uv tool install ruff
```

This installs Ruff into an isolated tool environment and makes its executable available on `PATH`.

### 10.4 Listing Installed Tools

```bash
uv tool list
```

This displays tools installed using `uv tool install`.

### 10.5 Upgrading Tools

Upgrade a specific tool:

```bash
uv tool upgrade ruff
```

Upgrade all installed tools:

```bash
uv tool upgrade --all
```

### 10.6 Uninstalling Tools

```bash
uv tool uninstall ruff
```

This removes the installed tool environment and its executable entry.

### 10.7 uv tool vs Project Dependencies

| uv tool | Project dependencies |
|---|---|
| Installs standalone CLI tools | Manages project-specific packages |
| Uses isolated tool environments | Uses the project environment |
| Useful across multiple projects | Specific to an individual project |
| Example: `uv tool install ruff` | Example: `uv add ruff` |

Use `uv tool` for globally accessible command-line utilities and project dependencies when a package is needed by your application or development workflow.

---

## 11. Using uv as a pip Replacement

uv provides a pip-compatible interface through the `uv pip` command.

This allows developers to migrate existing pip workflows without immediately changing their project structure.

### 11.1 Understanding the uv pip Interface

The `uv pip` interface provides commands similar to traditional pip.

| pip | uv pip |
|---|---|
| `pip install` | `uv pip install` |
| `pip uninstall` | `uv pip uninstall` |
| `pip list` | `uv pip list` |
| `pip freeze` | `uv pip freeze` |

### 11.2 Installing Packages

```bash
uv pip install requests
```

Install a specific version:

```bash
uv pip install requests==2.32.3
```

### 11.3 Uninstalling Packages

```bash
uv pip uninstall requests
```

### 11.4 Listing Installed Packages

```bash
uv pip list
```

### 11.5 Freezing Installed Packages

```bash
uv pip freeze
```

Save the output:

```bash
uv pip freeze > requirements.txt
```

### 11.6 Compiling Dependencies

`uv pip compile` is similar to the functionality provided by pip-tools.

It converts dependency requirements into a resolved requirements file.

For example, create `requirements.in`:

```text
requests
flask
```

Compile it:

```bash
uv pip compile requirements.in -o requirements.txt
```

The generated file contains resolved package versions.

### 11.7 Synchronizing Dependencies

```bash
uv pip sync requirements.txt
```

This synchronizes the target environment with the specified requirements file, removing packages not included in the compiled requirements.

### 11.8 Migrating Existing pip Workflows

Suppose an existing project uses:

```bash
python -m venv .venv
```

After activating the environment, we can replace:

```bash
pip install -r requirements.txt
```

With:

```bash
uv pip install -r requirements.txt
```

Or use the compiled requirements workflow:

```bash
uv pip compile requirements.in -o requirements.txt
uv pip sync requirements.txt
```

**Important:** The `uv pip` interface does not automatically turn an existing project into a uv-managed project. It is a lower-level package management interface that can work with existing environments.

For new uv-managed projects, prefer `uv add`, `uv remove`, `uv sync`, and `uv run`.

---

## 12. Advanced uv Features

### 12.1 Understanding the Global Cache

uv maintains a global cache to avoid downloading and building the same packages repeatedly.

This improves installation performance and reduces unnecessary network requests.

View the cache location:

```bash
uv cache dir
```

### 12.2 Cleaning the Cache

Remove outdated cache entries:

```bash
uv cache prune
```

Remove all cache entries:

```bash
uv cache clean
```

Cleaning the cache does not delete project source code or project configuration.

### 12.3 Environment Variables and Configuration

uv supports environment variables and configuration files to customize its behavior.

For example:

```bash
UV_CACHE_DIR=/custom/cache uv sync
```

This specifies a custom cache directory for the command.

Another example:

```bash
UV_PROJECT_ENVIRONMENT=.venv uv sync
```

This specifies the project environment location.

For more configuration options, refer to the official configuration documentation.

### 12.4 Using Alternative Package Indexes

By default, uv uses PyPI as its package index.

We can specify another package index:

```bash
uv add requests --index-url https://pypi.org/simple
```

For private package repositories, configure the appropriate index and authentication settings.

Avoid putting passwords or access tokens directly into commands or committed configuration files.

### 12.5 Git and URL Dependencies

uv supports installing packages directly from Git repositories.

Example:

```bash
uv add git+https://github.com/psf/requests
```

We can also specify a Git branch:

```bash
uv add "requests @ git+https://github.com/psf/requests@main"
```

For production projects, prefer a specific tag or commit when reproducibility matters.

### 12.6 Local Path Dependencies

We can add a package located on the local filesystem.

Example:

```bash
uv add --editable ../my-library
```

The `--editable` option installs the local package in editable mode, allowing changes to its source code to be reflected without reinstalling it.

### 12.7 Workspace Management

uv supports workspaces, allowing multiple related packages to be managed together.

Example structure:

```text
my-workspace/
│
├── pyproject.toml
│
├── packages/
│   ├── core/
│   │   └── pyproject.toml
│   │
│   └── api/
│       └── pyproject.toml
│
└── uv.lock
```

A workspace is useful when multiple packages belong to the same repository and need to share dependency resolution.

Workspaces are configured using `[tool.uv.workspace]` in `pyproject.toml`.

### 12.8 Building Python Packages

uv can build Python packages into distributable formats.

Run:

```bash
uv build
```

This typically generates:

- A source distribution (`.tar.gz`).
- A wheel distribution (`.whl`).

The generated files are placed in the `dist/` directory.

---

## 13. Best Practices

Following these practices helps maintain clean and reliable Python projects.

### 13.1 Use pyproject.toml for Project Configuration

Keep project metadata and dependencies in `pyproject.toml`.

Use commands such as:

```bash
uv add requests
uv remove requests
```

Avoid manually managing project dependencies when uv commands can do it for you.

### 13.2 Commit uv.lock to Version Control

Always commit `uv.lock` for applications and other projects where consistent dependency versions matter.

This allows team members and deployment environments to use the same resolved dependencies.

### 13.3 Add .venv to .gitignore

Do not commit the virtual environment.

Add:

```text
.venv/
```

to `.gitignore`.

### 13.4 Use uv sync to Synchronize Dependencies

When setting up a project on another machine, use:

```bash
uv sync --locked
```

This installs dependencies using the committed lockfile without updating it.

### 13.5 Use uv run to Execute Project Commands

Prefer:

```bash
uv run main.py
```

Instead of manually activating the environment every time.

uv automatically ensures that the project environment is synchronized before execution.

### 13.6 Pin Python Versions Appropriately

Use:

```bash
uv python pin 3.12
```

This creates a `.python-version` file and helps maintain a consistent development Python version.

Also specify the supported Python version range in `pyproject.toml` using `requires-python`.

### 13.7 Separate Production and Development Dependencies

Use normal dependencies for packages required by the application.

Use development dependency groups for tools such as:

- pytest
- ruff
- mypy

Example:

```bash
uv add --dev pytest ruff mypy
```

### 13.8 Avoid Manually Editing uv.lock

Let uv manage the lockfile.

Whenever dependencies change, use:

```bash
uv lock
```

or the appropriate project dependency command.

### 13.9 Choose the Appropriate Project Type

- Use packaged projects for libraries and distributable packages.
- Use non-packaged projects for standalone applications and scripts.

### 13.10 Keep uv Updated

Regularly update uv to benefit from performance improvements, bug fixes, and new features.

```bash
uv self update
```
