# Rust Web Server

A high-performance, scalable, and secure web server written in **Rust**. This project serves as an alternative to traditional web servers like Nginx, offering memory safety, asynchronous I/O, and modern protocol support.

---

## Features

- **Asynchronous I/O**: Built using `tokio` and `hyper` for non-blocking performance.
- **Concurrency**: Handles thousands of simultaneous requests.
- **Static File Serving**: Efficiently serves static files.
- **Easy Configuration**: Configurable via a `default.toml` file.
- **Systemd Integration**: Includes a systemd service file for Linux deployments.

---

## Requirements

- **Rust** (1.60+)
- Cargo (Rust package manager)

---

## Installation

### Clone the Repository

```bash
git clone https://github.com/goldenhippo58/rust_web_server.git
cd rust_web_server
