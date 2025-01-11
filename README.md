# Unaccent

`unaccent` is a simple and efficient Rust crate designed to remove accents (diacritical marks) from strings. Inspired by the [PostgreSQL `unaccent`](https://www.postgresql.org/docs/current/unaccent.html) extension, this crate offers an easy-to-use API for developers who need to normalize text by removing accents in their Rust applications.

## Features

- **Unicode Support**: Fully supports Unicode, ensuring accurate normalization for a wide range of languages. (Work in progress, though...)
- **Lightweight**: Minimal dependencies, keeping the crate efficient and easy to integrate.
- **High Performance**: Uses the `unicode-normalization` crate under the hood for robust and efficient text processing.
- **Cross-Platform**: Works seamlessly on all platforms supported by Rust.

## Installation

Add `unaccent` to your `Cargo.toml`:

```toml
[dependencies]
unaccent = "0.1.0"
```

Then, include it in your project:

```rust
use unaccent::unaccent;
```

## Usage

Here’s a quick example:

```rust
use unaccent::unaccent;

fn main() {
    let input = "Café au lait élégant";
    let result = unaccent(input);
    println!("Unaccented: {}", result); // Outputs: "Cafe au lait elegant"
}
```

## Example Use Cases

- Text preprocessing for search or indexing.
- Standardizing user input for comparison.
- Cleaning text for machine learning or natural language processing.

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

### Development Setup

1. Clone the repository:

   ```sh
   git clone https://github.com/crowdtech-io/unaccent.git
   cd unaccent
   ```

2. Run tests:

   ```sh
   cargo test
   ```

### Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this standard.

## License

This project is licensed under the [MIT License](./LICENSE).

## Acknowledgments

Special thanks to the creators of the PostgreSQL `unaccent` extension and the maintainers of the `unicode-normalization` crate for their foundational work.

---

**Note**: This crate is not affiliated with or endorsed by the PostgreSQL project.

