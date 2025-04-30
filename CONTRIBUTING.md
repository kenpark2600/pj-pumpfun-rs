# Contributing to PumpFun Rust SDK

Thank you for your interest in contributing to the PumpFun Rust SDK! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/pumpfun-rs.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

1. Install Rust and Cargo using [rustup](https://rustup.rs/)
2. Install Solana CLI tools following the [official guide](https://docs.solana.com/cli/install-solana-cli-tools)
3. Run `cargo build` to ensure everything compiles
4. Run `cargo test -- --test-threads 1` to run the test suite
5. For local Solana testing, use the included test validator script:

### Using the Test Validator

The repository includes a utility script to set up a local Solana test validator with the Pump.fun program:

```sh
# Navigate to the scripts directory
cd scripts

# Run the test validator
./pumpfun-test-validator.sh
```

This script:
- Downloads the Pump.fun program binary from mainnet
- Downloads the MPL Token Metadata program binary from mainnet
- Gets the required account data for the Pump.fun Global Account
- Configures a local test validator with these components

Options:
- Custom program directory: `PROGRAMS_DIR=./my-programs ./pumpfun-test-validator.sh`
- Custom accounts directory: `ACCOUNTS_DIR=./my-accounts ./pumpfun-test-validator.sh`
- Pass additional arguments to solana-test-validator: `./pumpfun-test-validator.sh --log`

The validator runs on:
- RPC: http://127.0.0.1:8899
- WebSocket: ws://127.0.0.1:8900

You can connect the SDK to this local validator for development:

```rust
let client = PumpFun::new(
    Cluster::Custom(
        "http://127.0.0.1:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    ),
    payer.clone(),
    Some(CommitmentConfig::confirmed()),
    None,
);
```

## Making Changes

1. Use conventional commit messages with emojis:
   - `✨ feat: add new feature`
   - `🐛 fix: fix bug in X`
   - `📝 docs: update README`
   - `♻️ refactor: refactor X component`
   - `🎨 style: format code`
   - `✅ test: add tests for X`
   - `⚡️ perf: improve performance`
   - `🌱 init: initial commit`
   - `🔧 chore: update dependencies`
2. Follow Rust coding conventions and style guidelines
3. Add tests for new functionality
4. Update documentation as needed
5. Ensure all tests pass: `cargo test`
6. Run `cargo fmt` to format code
7. Run `cargo clippy` to check for common mistakes

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Ensure your PR description clearly describes the problem and solution
3. Reference any related issues
4. Your PR will be reviewed by maintainers
5. Make requested changes if any
6. Once approved, your PR will be merged

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Document public APIs using rustdoc
- Keep functions focused and concise
- Use proper error handling

## Testing

- Write unit tests for new functionality
- Include integration tests where appropriate
- Test edge cases and error conditions
- Maintain test coverage

## Documentation

- Update API documentation for any changed functions
- Keep README and other documentation up to date
- Include examples for new features
- Document breaking changes

## Test Validator Maintenance

When maintaining the test validator script:

1. Keep the script compatible with different environments (Linux, macOS, Windows with WSL)
2. Document any changes to command-line options or environment variables
3. Update the script when new program versions are deployed to mainnet
4. Test changes with different configuration options

## Questions or Problems?

- Open an issue for bugs or feature requests
- Join our community channels for discussion
- Tag maintainers for urgent issues

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as specified in the repository.
