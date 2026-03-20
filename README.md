# Isolenv

Isolenv is a Rust-based engine designed to spin up isolated environments on cloud or PC infrastructure. It allows you to define reproducible environment rules through simple YAML configuration files.

These environments are perfectly suited for tasks such as cloning repositories, running software integration (SI) code, or setting up dedicated development containers for specific projects (e.g., Python data science, Rust building).

## How It Works

Isolenv parses a user-defined `.yaml` configuration file that describes the requirements of the isolated environment. The configuration schema supports the following properties:

- **Environment**: Define a unique name, base image (e.g., Docker image), and necessary environment variables.
- **Dependencies**: Declare system-level dependencies (like `apt` packages), as well as language-specific packages for Python, Node.js, and Rust.
- **Setup**: A list of shell commands to execute to prepare the environment before runtime.
- **Run**: The primary payload or command to execute once the isolated environment is fully provisioned.

## Example Configuration

Here is an example of an Isolenv configuration intended to set up a Python data science environment (`examples/python_env.yaml`):

```yaml
version: "1.0"
environment:
  name: "python-data-science"
  base_image: "ubuntu:22.04"
  env_vars:
    PYTHONUNBUFFERED: "1"
    MODEL_PATH: "/app/models"
    
dependencies:
  system:
    - build-essential
    - python3-dev
    - python3-pip
  python:
    - numpy
    - pandas
    - scikit-learn
    
setup:
  - echo "Setting up data science environment"
  - mkdir -p /app/models
  - mkdir -p /app/data
  
run:
  command: "python3 /app/main.py"
```

## Installation & Usage

Isolenv is built with Rust. You can compile a highly optimized executable directly from the source code.

### 1. Build and Run Locally
You can compile the release file and execute it directly from the `target` folder:

```bash
# Clone the repository and navigate into the project directory
cd Isolenv

# Build the release executable
cargo build --release

# Run the local executable directly
./target/release/isolenv spin up --config examples/python_env.yaml
```

### 2. Install Globally (Recommended)
Since Isolenv is an engine you will likely use across many projects and folders, you can install the CLI globally onto your system using Cargo. 

Just run this command from the `Isolenv` directory:
```bash
cargo install --path .
```

Once installed, you will be able to run it from absolutely anywhere on your system simply by typing:
```bash
isolenv spin up --config examples/python_env.yaml
```

### Global Configuration Argument
Fortunately, because Isolenv's CLI is built on `clap`, we defined `--config` as a global argument on the top-level CLI struct. This means the CLI natively supports both positions:

```bash
isolenv -c examples/python_env.yaml spin up
isolenv spin up -c examples/python_env.yaml
```
This gives users maximum flexibility while keeping the engine perfectly clean and scalable!

*Note: Isolenv will default to looking for `isolenv.yaml` in the current directory if a configuration file path is not explicitly provided.*
