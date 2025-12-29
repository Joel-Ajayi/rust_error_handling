# Rust Error Handling
# Rust Error Handling

This project demonstrates error handling in Rust using various techniques and libraries, including manual enum-based errors, `thiserror`, and `anyhow`.

## Features
- Custom error types using manual enums
- Convenience error types using `thiserror`
- Application-level error handling with `anyhow`
- Examples of decoding base64 strings

## Approaches shown

- Plain enums: demonstrates creating a custom `enum` for errors and implementing `std::fmt::Display`, `std::fmt::Debug`, and `std::error::Error` manually. This shows how to control error formatting and conversions explicitly.
- `thiserror`: uses `#[derive(thiserror::Error)]` and `#[from]` on variants to automatically generate `Display`, `Error`, and `From` conversions so the `?` operator converts errors into your `AppError`.
- `anyhow`: demonstrates returning `anyhow::Error` for application code when you want ergonomic context-aware errors without defining a rigid error type.

The repo includes both the manual enum-based examples (to illustrate how `Display`/`Error`/`From` work) and the `thiserror`/`anyhow` examples (to show the ergonomic alternatives).

## Pending work

- Create a small procedural macro that mimics the `#[from]` conversion behavior from `thiserror` so variant-level `From` impls are generated automatically. This is intentionally left as a follow-up exercise.

## Getting Started

### Prerequisites
- Rust installed on your machine.

### Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust_error_handling
   ```
2. Build the project:
   ```bash
   cargo build
   ```

### Usage
Run the application:
```bash
cargo run
```

### Contributing
Contributions are welcome! Please open an issue or submit a pull request.

### License
This project is licensed under the MIT License.