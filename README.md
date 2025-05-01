# 🛡️ Rustprox

A lightweight command-line proxy utility written in Rust.  
Easily configure proxy settings across Linux, macOS, and Windows — with one simple install command.

---

## ✨ Features

- 🌍 Automatically sets system-wide proxy variables
- 🖥️ Supports `.bashrc`, `.zshrc`, `.npmrc`, `apt`, and Git global config
- 🧠 Detects OS and applies platform-specific logic
- 🔧 Works on **Linux**, **macOS**, and **Windows**
- 🧵 Written in fast, reliable Rust

---

## 🚀 Installation

You can install the latest version directly using `curl` on linux:

```bash
curl -fsSL curl -fsSL https://raw.githubusercontent.com/07CalC/rustprox/master/install.sh | sh
 ```

for windows: 
```bash
irm https://raw.githubusercontent.com/07CalC/rustprox/master/install.ps1 | iex
```

for macos (build from code):
### ✅ Prerequisites

Before you begin, ensure the following are installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/)
- Xcode Command Line Tools:
  ```bash
  xcode-select --install
  ```
  clone the repository
  ```bash
  git clone https://github.com/07CalC/rustprox.git
  cd rustprox
  ```
  You can build the release version using:
  ```bash
  cargo build --release
  ```
  The final binary will be located at:
  ```bash
  target/release/rustprox
  ```
  To install rustprox globally so it can be used from anywhere:
  ```bash
  cargo install --path .
  ```
  Then use:
  ```bash
  rustprox --help
  ```
  use sudo if permission denied
