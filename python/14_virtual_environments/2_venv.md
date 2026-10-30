# Python venv

## 1. Introduction to venv

### 1.1 What is venv?

`venv` is a built-in Python module used to create virtual environments.

It allows us to create isolated Python environments for individual projects. 

Each environment maintains its own Python interpreter reference, installed packages, and package management tools.

Using `venv`, we can install and manage project-specific dependencies without affecting other Python projects or the system-wide Python installation.

### 1.2 What does the venv module do?

The `venv` module creates a directory containing the necessary files and directories for an isolated Python environment.

A typical virtual environment includes:

- A Python interpreter or a reference to the base Python installation.
- A `site-packages` directory for installing third-party packages.
- A `pip` installation for managing packages.
- Activation scripts for different operating system shells.
- Configuration information about the environment.

### 1.3 How does venv work internally?

When we create a virtual environment, Python creates a directory containing the environment-specific files.

For example:

```bash
python -m venv .venv
```

This creates a `.venv` directory in the current working directory.

When the environment is activated, the environment's executable directory is added to the beginning of the `PATH` environment variable.

As a result:

- Running `python` uses the environment's Python interpreter.
- Running `pip` uses the environment's package installer.
- Packages are installed inside the environment rather than the global Python installation.

Python also uses the environment's configuration file, `pyvenv.cfg`, to identify the base Python installation and environment settings.

**Important:** A virtual environment is isolated from other environments' installed packages by default, but it is not a security boundary. It is also generally tied to the Python installation used to create it.

### 1.4 Prerequisites and Python version requirements

Before creating a virtual environment, ensure that Python is installed on your system.

Check the installed Python version:

```bash
python --version
```

On some systems, you may need:

```bash
python3 --version
```

On Windows, you can also use the Python launcher:

```bash
py --version
```

The `venv` module is included with standard Python installations.

On some Linux distributions, the `venv` component may need to be installed separately through the system package manager.

For example, on Ubuntu/Debian:

```bash
sudo apt install python3-venv
```

For a specific Python version:

```bash
sudo apt install python3.12-venv
```


## 2. Creating a Virtual Environment

The `venv` module provides the `python -m venv` command to create virtual environments.

### 2.1 Basic Syntax

```bash
python -m venv <environment_name>
```

Here:

- `python` invokes the Python interpreter.
- `-m` executes a Python module as a script.
- `venv` specifies the module used to create the virtual environment.
- `<environment_name>` specifies the directory where the environment will be created.

Example:

```bash
python -m venv myenv
```

This creates a virtual environment named `myenv` in the current directory.

### 2.2 Creating an Environment with a Custom Name

We can give any valid directory name to our virtual environment.

For example:

```bash
python -m venv myenv
```

Or:

```bash
python -m venv development
```

Or:

```bash
python -m venv test-env
```

However, `.venv` is a commonly recommended naming convention.

```bash
python -m venv .venv
```

The leading dot makes the directory hidden by default on Unix-like operating systems. Many development tools also recognize `.venv` automatically.

### 2.3 Creating an Environment in a Specific Directory

We can provide an absolute or relative path instead of just a directory name.

**Relative path:**

```bash
python -m venv environments/myenv
```

**Windows absolute path:**

```powershell
python -m venv C:\Projects\myenv
```

**Linux/macOS absolute path:**

```bash
python -m venv /home/user/projects/myenv
```

Python creates the environment at the specified location.

### 2.4 Creating a Virtual Environment with a Specific Python Version

Sometimes, different projects require different Python versions.

For example:

- Project A requires Python 3.11.
- Project B requires Python 3.12.
- Project C requires Python 3.13.

We can create virtual environments using specific Python installations.

**Windows:**

Windows provides the Python Launcher (`py`) to select installed Python versions.

Check available Python versions:

```powershell
py --list
```

Create an environment using Python 3.12:

```powershell
py -3.12 -m venv .venv
```

Create an environment using Python 3.11:

```powershell
py -3.11 -m venv .venv
```

**Linux/macOS:**

If the required Python version is installed and available in `PATH`, use its executable directly.

```bash
python3.12 -m venv .venv
```

Or:

```bash
python3.11 -m venv .venv
```

**Important points:**

- The requested Python version must already be installed.
- `venv` does not download or install Python versions.
- The environment uses the Python version specified when creating it.
- To use another Python version, create a new environment using that interpreter.

### 2.5 Understanding the .venv Naming Convention

A common convention is to name the virtual environment directory `.venv`.

Example project structure:

```text
my-project/
│
├── .venv/
├── main.py
├── requirements.txt
└── README.md
```

Why use `.venv`?

- It clearly identifies the virtual environment.
- It is recognized by many Python development tools.
- It keeps the project structure organized.
- It can be excluded from version control easily.

### 2.6 Understanding the Generated Directory Structure

The exact structure varies depending on the operating system and Python version.

**Windows:**

```text
.venv/
│
├── Include/
├── Lib/
│   └── site-packages/
│
├── Scripts/
│   ├── activate
│   ├── activate.bat
│   ├── Activate.ps1
│   ├── deactivate.bat
│   └── python.exe
│
└── pyvenv.cfg
```

**Linux/macOS:**

```text
.venv/
│
├── bin/
│   ├── activate
│   ├── activate.csh
│   ├── activate.fish
│   ├── python
│   ├── pip
│   └── ...
│
├── include/
├── lib/
│   └── python3.x/
│       └── site-packages/
│
└── pyvenv.cfg
```

Important directories and files:

| File/Directory | Purpose |
|---|---|
| `Scripts/` | Contains Windows environment executables and activation scripts. |
| `bin/` | Contains Linux/macOS executables and activation scripts. |
| `Lib/` | Contains Python libraries on Windows. |
| `lib/` | Contains Python libraries on Linux/macOS. |
| `site-packages/` | Stores installed third-party Python packages. |
| `pyvenv.cfg` | Stores virtual environment configuration information. |

The environment directory should generally be treated as generated content rather than source code.

---

## 3. Activating a Virtual Environment

Creating a virtual environment does not automatically activate it.

We need to activate it before using its Python interpreter and package management commands through the normal shell commands.

### 3.1 Windows – Command Prompt (CMD)

```cmd
.venv\Scripts\activate.bat
```

After activation, the environment name appears at the beginning of the command prompt.

Example:

```text
(.venv) C:\Projects\my-project>
```

### 3.2 Windows – PowerShell

```powershell
.venv\Scripts\Activate.ps1
```

After activation:

```text
(.venv) PS C:\Projects\my-project>
```

**Common PowerShell issue:**

Sometimes PowerShell prevents activation scripts from running because of its execution policy.

You may see an error similar to:

```text
running scripts is disabled on this system
```

One option is to allow locally created scripts for the current user:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Alternatively, you can temporarily change the policy for the current PowerShell process:

```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
```

Then activate the environment again:

```powershell
.venv\Scripts\Activate.ps1
```

The process-scoped setting is temporary and does not persist after closing that PowerShell session.

### 3.3 Linux/macOS – Bash or Zsh

```bash
source .venv/bin/activate
```

After activation:

```text
(.venv) user@computer:~/my-project$
```

### 3.4 Linux/macOS – Fish Shell

```fish
source .venv/bin/activate.fish
```

### 3.5 What Happens When We Activate an Environment?

Activation mainly modifies the current shell environment.

The most important change is that the virtual environment's executable directory is added to the beginning of the `PATH` variable.

For example, before activation:

```text
PATH=/usr/bin:/usr/local/bin
```

After activation, it may look like:

```text
PATH=/home/user/my-project/.venv/bin:/usr/bin:/usr/local/bin
```

Now, when we execute:

```bash
python
```

The shell finds the Python executable inside `.venv` first.

Activation also sets the `VIRTUAL_ENV` environment variable and updates the shell prompt by default.

**Important:** Activation is optional. We can use the environment's Python executable directly without activating it.

### 3.6 How to Verify That the Environment Is Active

There are several ways to verify the active Python environment.

**Method 1: Check the Python executable location**

```bash
python -c "import sys; print(sys.executable)"
```

Expected output on Windows:

```text
C:\Projects\my-project\.venv\Scripts\python.exe
```

Expected output on Linux/macOS:

```text
/home/user/my-project/.venv/bin/python
```

**Method 2: Check the Python environment prefix**

```bash
python -c "import sys; print(sys.prefix)"
```

This should display the virtual environment directory.

**Method 3: Check the VIRTUAL_ENV variable**

Linux/macOS:

```bash
echo $VIRTUAL_ENV
```

Windows CMD:

```cmd
echo %VIRTUAL_ENV%
```

Windows PowerShell:

```powershell
echo $env:VIRTUAL_ENV
```

**Method 4: Check the Python version**

```bash
python --version
```

This confirms the interpreter version, although it does not independently prove that a virtual environment is active.

---

## 4. Deactivating a Virtual Environment

When we finish working with a virtual environment, we can deactivate it.

Use:

```bash
deactivate
```

This command works after activation in the supported shells.

Before deactivation:

```text
(.venv) C:\Projects\my-project>
```

After deactivation:

```text
C:\Projects\my-project>
```

### What Happens After Deactivation?

- The virtual environment is removed from the active shell's `PATH` priority.
- The `VIRTUAL_ENV` variable is unset.
- The shell returns to using the previously available Python interpreter.
- Installed packages remain inside the virtual environment.

**Important:** Deactivating an environment does not delete it or uninstall its packages. It only stops using that environment as the active shell environment.

---

## 5. Installing Python Packages

Once a virtual environment is activated, we can install Python packages using `pip`.

`pip` is Python's package installer. It downloads and installs packages from package repositories such as PyPI (Python Package Index).

### 5.1 Understanding pip Inside a Virtual Environment

When we activate a virtual environment, the environment's executable directory is added to `PATH`.

Therefore, running:

```bash
pip install requests
```

installs `requests` into the active virtual environment.

To verify which pip executable is being used:

**Windows:**

```cmd
where pip
```

**Linux/macOS:**

```bash
which pip
```

You can also use:

```bash
python -m pip --version
```

Using `python -m pip` is generally recommended because it explicitly runs pip through the selected Python interpreter.

### 5.2 Installing a Package

Syntax:

```bash
pip install <package_name>
```

Example:

```bash
pip install requests
```

This installs the latest compatible version of `requests` available from the configured package index.

Recommended alternative:

```bash
python -m pip install requests
```

### 5.3 Installing a Specific Package Version

Sometimes a project requires a specific version of a package.

Syntax:

```bash
pip install <package_name>==<version>
```

Example:

```bash
pip install requests==2.32.3
```

This installs the specified version if it is available and compatible with the environment.

### 5.4 Installing Multiple Packages

We can install multiple packages in a single command.

```bash
pip install requests flask django
```

Or:

```bash
python -m pip install requests flask django
```

### 5.5 Installing Packages from requirements.txt

When a project contains a `requirements.txt` file, we can install all listed dependencies using:

```bash
pip install -r requirements.txt
```

Example `requirements.txt`:

```text
requests==2.32.3
flask==3.1.0
django==5.1.4
```

The command reads the file and installs the specified packages.

---

## 6. Managing Installed Packages

After installing packages, we may need to inspect, update, or remove them.

`pip` provides several commands for package management.

### 6.1 Listing Installed Packages

Use:

```bash
pip list
```

This displays the packages installed in the current Python environment.

Example output:

```text
Package    Version
---------- -------
Flask      3.1.0
requests   2.32.3
urllib3    2.2.3
```

### 6.2 Inspecting Package Information

Use:

```bash
pip show <package_name>
```

Example:

```bash
pip show requests
```

This displays information such as:

- Package name.
- Version.
- Summary.
- Installation location.
- Dependencies.

### 6.3 Checking Outdated Packages

Use:

```bash
pip list --outdated
```

This displays installed packages for which newer versions are available.

Example output:

```text
Package    Version   Latest    Type
---------- --------- --------- -----
requests   2.28.0    2.32.3    wheel
```

### 6.4 Upgrading Packages

To upgrade a package to the latest compatible version:

```bash
pip install --upgrade <package_name>
```

Example:

```bash
pip install --upgrade requests
```

To upgrade pip itself:

```bash
python -m pip install --upgrade pip
```

### 6.5 Uninstalling Packages

Use:

```bash
pip uninstall <package_name>
```

Example:

```bash
pip uninstall requests
```

To uninstall without an interactive confirmation:

```bash
pip uninstall -y requests
```

### 6.6 Checking the pip Version

Use:

```bash
pip --version
```

Or:

```bash
python -m pip --version
```

This displays the installed pip version and the Python environment associated with it.

---

## 7. Managing Dependencies with requirements.txt

When developing Python applications, we often depend on multiple third-party packages.

For example, a web application might use:

- Flask for the web framework.
- SQLAlchemy for database operations.
- Requests for HTTP requests.
- Pytest for testing.

When another developer works on the same project, they need to install the required dependencies.

The `requirements.txt` file helps us share the project's package requirements.

### 7.1 What is requirements.txt?

`requirements.txt` is a text file commonly used to specify Python package requirements.

Example:

```text
flask==3.1.0
requests==2.32.3
sqlalchemy==2.0.36
pytest==8.3.4
```

Each line specifies a package requirement.

The `==` operator specifies an exact package version.

A requirements file can also contain version ranges, environment markers, URLs, and other supported pip requirement specifications.

### 7.2 Generating requirements.txt Using pip freeze

The `pip freeze` command displays installed packages in a format suitable for requirements files.

Use:

```bash
pip freeze
```

To save the output into a file:

```bash
pip freeze > requirements.txt
```

This creates a `requirements.txt` file containing the installed packages and their versions.

Example:

```text
blinker==1.9.0
click==8.1.8
Flask==3.1.0
itsdangerous==2.2.0
Jinja2==3.1.5
MarkupSafe==3.0.2
Werkzeug==3.1.3
```

**Important:** `pip freeze` generally lists all installed distributions in the environment, including transitive dependencies. It does not distinguish between packages explicitly installed by you and packages installed as dependencies of other packages.

### 7.3 Understanding the Contents of requirements.txt

Consider:

```text
Flask==3.1.0
requests==2.32.3
```

Here:

- `Flask` is the package name.
- `3.1.0` is the required version.
- `==` means that the specified version is required.

Common version specifiers include:

| Operator | Meaning | Example |
|---|---|---|
| `==` | Exact version | `requests==2.32.3` |
| `>=` | Minimum version | `requests>=2.30.0` |
| `<=` | Maximum version | `requests<=2.32.3` |
| `!=` | Exclude a version | `requests!=2.31.0` |
| `~=` | Compatible release | `requests~=2.32` |

### 7.4 Recreating an Environment Using requirements.txt

Suppose another developer receives your project with the following structure:

```text
my-project/
│
├── main.py
└── requirements.txt
```

They can create a virtual environment:

```bash
python -m venv .venv
```

Activate it.

Then install all dependencies:

```bash
python -m pip install -r requirements.txt
```

This installs the packages specified in the requirements file.

### 7.5 Updating requirements.txt

Whenever we install or upgrade packages, we may need to update the requirements file.

For example:

```bash
pip install flask
pip install requests
```

After installing the packages, regenerate the file:

```bash
pip freeze > requirements.txt
```

This updates the file with the currently installed distributions.

**Best practice:** Review the generated file before committing it. For larger applications, consider maintaining a separate file for direct dependencies and using a dependency-locking workflow when stronger reproducibility is required.

---

## 8. Running Python Programs Inside a Virtual Environment

Once a virtual environment is created, we can execute Python programs using its interpreter.

There are two approaches.

### 8.1 Running Scripts After Activating the Environment

First, activate the environment.

For example:

```bash
# Windows PowerShell
.venv\Scripts\Activate.ps1

# Linux/macOS
source .venv/bin/activate
```

Now run the Python script:

```bash
python main.py
```

The script executes using the Python interpreter from the active virtual environment.

### 8.2 Running Scripts Without Activating the Environment

Activation is not mandatory.

We can directly execute the Python interpreter inside the virtual environment.

**Windows:**

```powershell
.venv\Scripts\python.exe main.py
```

**Linux/macOS:**

```bash
.venv/bin/python main.py
```

This executes the script using the environment's Python interpreter without changing the current shell's activation state.

### 8.3 Using the Correct Python Interpreter

When multiple Python installations or virtual environments exist, it is important to verify which interpreter is executing the program.

Use:

```bash
python -c "import sys; print(sys.executable)"
```

This prints the full path of the Python executable.

We can also inspect the environment prefix:

```bash
python -c "import sys; print(sys.prefix)"
```

And compare it with the base Python installation:

```bash
python -c "import sys; print(sys.base_prefix)"
```

When running inside a standard virtual environment:

- `sys.prefix` points to the virtual environment.
- `sys.base_prefix` points to the base Python installation.

A difference between these two values is a reliable way to identify a standard virtual environment.

---

## 9. Understanding venv Options

The `venv` module supports several optional arguments that control how virtual environments are created.

General syntax:

```bash
python -m venv [options] <environment_directory>
```

### 9.1 --system-site-packages

By default, a virtual environment cannot access packages installed in the base Python installation's site-packages directory.

The `--system-site-packages` option allows the environment to access those packages.

Example:

```bash
python -m venv --system-site-packages .venv
```

Use this option when you intentionally want the environment to access globally installed packages.

**Note:** Packages installed inside the virtual environment remain separate from the base environment's packages.

### 9.2 --clear

The `--clear` option removes the existing contents of the target environment directory before creating the new environment.

Example:

```bash
python -m venv --clear .venv
```

This is useful when you want to recreate an existing environment in the same directory.

**Warning:** Existing files in the target environment directory may be deleted. Do not use this option on a directory containing files you need to preserve.

### 9.3 --upgrade

The `--upgrade` option upgrades an existing virtual environment to use the current Python installation.

Example:

```bash
python -m venv --upgrade .venv
```

This is useful when the base Python installation has been upgraded and you want to update the existing environment accordingly.

It does not automatically upgrade all third-party packages installed in the environment.

### 9.4 --without-pip

By default, `venv` installs pip into the new environment.

The `--without-pip` option skips this step.

Example:

```bash
python -m venv --without-pip .venv
```

This creates an environment without installing pip through the normal `venv` creation process.

### 9.5 --prompt

The `--prompt` option allows us to customize the environment name displayed in the shell prompt after activation.

Example:

```bash
python -m venv --prompt my-project .venv
```

After activation, the shell prompt displays the custom name instead of the directory name.

### 9.6 --copies

By default, Python may use symbolic links or copies when setting up the environment, depending on the operating system and Python implementation.

The `--copies` option instructs `venv` to copy files instead of using symbolic links wherever possible.

Example:

```bash
python -m venv --copies .venv
```

This can be useful in environments where symbolic links are undesirable or unsupported.

**Note:** The exact behavior of these options can vary by platform and Python implementation.

---
## 10. Best Practices

Following good practices helps keep Python projects organized and easier to maintain.

### 10.1 Keep the Environment Inside the Project

Use `.venv` as the virtual environment directory name.

Example:

```text
my-project/
│
├── .venv/
├── main.py
├── requirements.txt
└── README.md
```

This keeps the environment associated with its project.

### 10.2 Add .venv to .gitignore

Virtual environments should generally not be committed to Git repositories.

They contain generated files, installed packages, and machine-specific paths.

Add the following entry to `.gitignore`:

```text
.venv/
```

This prevents Git from tracking the virtual environment directory.

### 10.3 Avoid Committing Virtual Environment Directories

Do not upload the entire `.venv` directory to GitHub or other source control systems.

Instead, commit the dependency information required to recreate the environment.

For example:

```text
my-project/
│
├── .gitignore
├── main.py
├── requirements.txt
└── README.md
```

Other developers can create their own virtual environments and install the required dependencies.

### 10.4 Use requirements.txt to Share Dependencies

Maintain a dependency file so other developers can install the packages required by your project.

Generate it using:

```bash
pip freeze > requirements.txt
```

Install dependencies using:

```bash
pip install -r requirements.txt
```

For more advanced projects, consider using a dedicated dependency-management and locking workflow.

### 10.5 Use a Separate Environment for Each Project

Avoid using one virtual environment for multiple unrelated projects.

For example:

```text
projects/
│
├── project-a/
│   ├── .venv/
│   └── main.py
│
├── project-b/
│   ├── .venv/
│   └── main.py
│
└── project-c/
    ├── .venv/
    └── main.py
```

Each project can maintain its own dependencies and Python environment.

### 10.6 Don't Move or Copy Virtual Environments Between Machines

Virtual environments are generally not portable.

They may contain:

- Absolute paths to the base Python installation.
- References to the original environment location.
- Platform-specific executable files.
- Operating-system-specific dependencies.

Instead of copying an environment to another machine, create a new environment and install the project's dependencies.

### 10.7 Always Verify the Active Environment Before Installing Packages

Before installing packages, verify that the correct environment is active.

Use:

```bash
python -c "import sys; print(sys.executable)"
```

This helps prevent accidentally installing packages into the global Python installation or another project's environment.

