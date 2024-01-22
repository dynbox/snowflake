# Getting Started

## Prerequisites
Before getting started with the project, ensure that you have the following tools and dependencies installed:

- [Git](https://git-scm.com/downloads) - A version control system widely used for tracking changes in code;
- [Rust](https://www.rust-lang.org/tools/install) - Rust is used in the backend of the project to build fast and reliable server-side applications;

If you are on linux, most of these dependencies are available in your distro's package manager, just install them via command line.

## Preparing the Environment

Clone the repository with git and navigate to the project:

```bash
git clone https://github.com/dynbox/{{PROJECT_NAME}}.git
cd {{PROJECT_NAME}}
```

## Compiling
To compile the project, simply use the following command:

```bash
cargo build --dev
```

This command will use Cargo to compile the Rust code, applying the specified build scripts and configurations.

## Running Examples
You can run all binaries from examples using:

```bash
cargo run --dev --example name_of_example
```

## Additional Resources
- [Structure](https://github.com/dynbox/{{PROJECT_NAME}}/blob/dev/docs/STRUCTURE.md) - This file provides an overview of the different modules or subprojects within the main project;