# 🚀 Mini Ripgrep in Rust

A high-performance, multithreaded command-line search tool inspired by `grep` and `ripgrep`, implemented in Rust.

This project demonstrates concurrent file traversal, regex-based pattern matching, and scalable search using multiple threads.

---

## 📌 Overview

This tool allows you to search for patterns in files and directories efficiently. It supports multiple modes including recursive search, depth-limited traversal, reverse matching, and configurable multithreading.

Designed as a systems-level project, it showcases practical usage of Rust’s concurrency primitives and file system handling.

---

## ✨ Features

* 🔍 Search for patterns inside a single file
* 📁 Recursive directory traversal
* 🧵 Multithreaded search for improved performance
* 🔄 Reverse search (show non-matching lines)
* 📏 Depth-limited directory search
* 🧠 Regex-based pattern matching
* ⚡ Efficient file reading using buffered I/O

---

## 🧠 Architecture

The project is built around a shared work queue:

* Uses `Arc<Mutex<Vec<PathBuf>>>` for thread-safe task sharing
* Multiple threads consume tasks concurrently
* Each thread processes files or explores directories
* Regex matching is applied line-by-line for efficiency

---

## ⚙️ Installation

Make sure you have Rust installed:

```bash
rustc --version
```

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/mini-ripgrep-rust.git
cd mini-ripgrep-rust
```

Build the project:

```bash
cargo build --release
```

---

## 🚀 Usage

Run the program using:

```bash
cargo run -- <mode> <path> <pattern> [extra]
```

---

## 🔧 Modes

### 1. File Search (`f`)

Search for a pattern inside a single file:

```bash
cargo run -- f <file_path> <pattern>
```

---

### 2. Directory Search (`s`)

Recursively search all files in a directory:

```bash
cargo run -- s <directory_path> <pattern>
```

---

### 3. Depth-Limited Search (`sd`)

Limit search to a maximum directory depth:

```bash
cargo run -- sd <directory_path> <pattern> <depth>
```

---

### 4. Reverse Grep (`rg`)

Show lines that do NOT match the pattern:

```bash
cargo run -- rg <directory_path> <pattern>
```

---

### 5. Multi-threaded Search (`nt`)

Specify number of threads:

```bash
cargo run -- nt <directory_path> <pattern> <num_threads>
```

---

## 📌 Example

```bash
cargo run -- s ./src "fn main"
```

Output:

```
File Path: src/main.rs, Line 120: fn main() {
```

---

## 🧪 Error Handling

* Invalid file paths are handled gracefully
* Invalid regex patterns may cause runtime panic (can be improved)
* Input validation is performed for required arguments

---

## ⚠️ Limitations

* Uses `Vec` as queue → not optimal for heavy workloads (O(n) removal)
* Regex is recompiled multiple times (can be optimized)
* No ignore rules (like `.gitignore`)
* No colored output or ranking

---

## 🔮 Future Improvements

* Replace `Vec` with `VecDeque` for efficient queue operations
* Precompile and reuse regex across threads
* Add `.gitignore` support
* Improve CLI with argument parsing (e.g. `clap`)
* Add colored output for better readability
* Benchmark performance vs `grep` / `ripgrep`

---

## 🛠️ Technologies

* Rust
* regex crate
* std::thread
* Arc & Mutex for synchronization
* File system APIs

---

## 📚 Learning Outcomes

This project demonstrates:

* Multithreading in Rust
* Safe shared state management
* File system traversal
* Regex-based search
* CLI application design

---

## 📄 License

This project is open-source and available under the MIT License.
