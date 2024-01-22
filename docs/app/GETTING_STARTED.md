# Getting Started

## Prerequisites
Before getting started with the project, ensure that you have the following tools and dependencies installed:

- [Git](https://git-scm.com/downloads) - A version control system widely used for tracking changes in code;
- [Rust](https://www.rust-lang.org/tools/install) - Rust is used in the backend of the project to build fast and reliable server-side applications;
- [Node.js](https://nodejs.org/) - A JavaScript runtime environment used in frontend;
- [PNPM](https://pnpm.io/installation) - A fast and disk space-efficient package manager for Node.js projects;
- [PostgreSQL](https://www.postgresql.org/download/) - An open-source relational database system, used to store and manage data.

If you are on linux, most of these dependencies are available in your distro's package manager, just install them via command line.

## Setting Up PostgreSQL

Open the terminal in your system and run the following commands:

> ⚠️ Replace "version" with the version you download on the website, e.g., "15", "16"

1. Start PostgreSQL Service:
   - On Linux systemd, start the PostgreSQL service using:
     ```
     sudo systemctl start postgresql
     ```
   - On Windows, start the PostgreSQL service from the Command Prompt as an administrator:
     ```
     net start postgresql-x64-version
     ```

2. Access the PostgreSQL Command Line:
   - On Linux, open a terminal and run the following command to access the PostgreSQL command line:
     ```
     sudo -u postgres psql
     ```
   - On Windows, navigate to the PostgreSQL bin directory using the following command:
     ```
     cd C:\"Program Files"\PostgreSQL\version\bin
     ```
     And run:
     ```
     ./psql -U postgres
     ```

3. Initialize the Database:
   - In the PSQL terminal, create the project database:
     ```
     \i project_path/init.sql
     ```

## Preparing the Environment

Clone the repository with git and navigate to the project:

```bash
git clone https://github.com/dynbox/{{PROJECT_NAME}}.git
cd {{PROJECT_NAME}}
```

Install cargo-make to automate and simplify the build and execution tasks:
```bash
cargo install --force cargo-make
```

## Compiling
To compile the project, simply use the following command:

```bash
cargo make --no-workspace build
```

This command will use Cargo to compile the Rust code and PNPM to build the frontend, applying the specified build scripts and configurations.

## Running
You can run all binaries generated on build using:

```bash
cargo make --no-workspace run
```

## Variables
| Name     | Default  | Description |
|----------|----------|----------   |
| TODO     | TODO     | TODO        |
| TODO     | TODO     | TODO        |
| TODO     | TODO     | TODO        |

## Additional Resources
- [Structure](https://github.com/dynbox/{{PROJECT_NAME}}/blob/dev/docs/STRUCTURE.md) - This file provides an overview of the different modules or subprojects within the main project;