# Test Gates

A Rust library that provides a `require_env` attribute macro to conditionally run tests based on the presence of environment variables.

## Installation

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
test-gates = "0.1.0"
```

## Usage

The `require_env` macro allows you to skip tests when certain environment variables are not available, rather than having them fail. This is useful for tests that interact with external services or require specific environment setup.

### Basic Usage

```rust
use test_gates::require_env;

#[test]
#[require_env("API_KEY")]
fn test_api_integration() {
    // This test only runs if API_KEY is set
    let api_key = std::env::var("API_KEY").unwrap();
    // Test implementation...
}
```

### Multiple Environment Variables

You can require multiple environment variables:

```rust
#[test]
#[require_env("DATABASE_URL", "API_KEY", "SECRET_TOKEN")]
fn test_with_multiple_requirements() {
    // This test only runs if ALL the specified env vars are set
    // Otherwise, it will be skipped with a message
    // Test implementation...
}
```

### Works with Other Test Frameworks

The macro is compatible with other test frameworks like Tokio:

```rust
#[tokio::test]
#[require_env("DATABASE_URL")]
async fn test_database_connection() {
    // This test only runs if DATABASE_URL is set
    // Test implementation...
}
```

## How It Works

When a test with the `require_env` attribute is run:

1. The macro checks if each specified environment variable is set
2. If any of the variables are missing, the test body is skipped and a message is printed
3. If all variables are present, the test runs normally

This allows you to include tests in your test suite that depend on environment variables without causing failures when those variables aren't available.

## Example Output

When a required environment variable is missing:

```
running 1 test
Skipping test: Required environment variable 'API_KEY' not found
test test_api_integration ... ok
```

The test is marked as "passed" because it didn't panic, but the actual test code was skipped.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 