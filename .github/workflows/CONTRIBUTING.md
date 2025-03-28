# Contributing to demand-cli

Thank you for your interest in contributing to demand-cli! This document provides guidelines and instructions for contributing to this project.

## Project Overview

demand-cli is a Rust-based CLI tool for connecting to the first FULLY StratumV2 Bitcoin Mining Pool. It implements a translation proxy that enables both StratumV1 and StratumV2 miners to connect to StratumV2 pools.

Key features include:
- Full StratumV2 support
- Translation proxy for StratumV1 miners
- NOISE protocol authentication and encryption
- Optimized performance with reduced bandwidth usage
- Automatic difficulty adjustment with PID controller
- Efficient job distribution

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment Setup](#development-environment-setup)
- [Building the Project](#building-the-project)
- [Coding Standards](#coding-standards)
- [Making Changes](#making-changes)
- [Branching Strategy](#branching-strategy)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Testing](#testing)
- [Protocol-Specific Testing](#protocol-specific-testing)
- [Environment Variables](#environment-variables)
- [Pull Request Process](#pull-request-process)
- [Documentation](#documentation)
## Code of Conduct

We expect all contributors to follow our Code of Conduct. Please ensure you are welcoming and respectful in all project interactions.

## Getting Started

### Development Environment Setup

1. **Install Rust:**
- Follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- We recommend using `rustup` to manage your Rust toolchain
- Minimum Rust version 1.70.0 is recommended for this project

2. **Clone the Repository:**
```sh
git clone https://github.com/demand-open-source/demand-cli.git
cd demand-cli
```

3. **Install Dependencies:**
- Ensure you have all required system dependencies
- Run `cargo build` to fetch Rust dependencies

### Building the Project

- Build the project with: `cargo build`
- For release builds: `cargo build --release`
- Run the project with: `cargo run`
- Run in test mode: `export TOKEN=oFzg1EUmceEcDuvzT3qt && cargo run --release -- --test -d 50T`
## Coding Standards

We follow standard Rust conventions and idioms:

1. **Formatting:**
- Use `cargo fmt` before submitting code
- We follow standard Rust formatting as defined by `rustfmt`

2. **Linting:**
- Run `cargo clippy` to check for common issues
- Address all clippy warnings or justify exceptions in PR descriptions

3. **Code Style:**
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Prioritize readability and maintainability
- Use meaningful variable/function names
- Comment complex algorithms or non-obvious behavior
- Keep functions small and focused on a single responsibility

4. **Error Handling:**
- Use proper error handling, avoiding `.unwrap()` and `.expect()` in production code
- Consider using the `thiserror` crate for defining custom errors

## Making Changes

### Branching Strategy

- The `main` branch contains the latest stable code
- Create feature branches from `main` with descriptive names:
- `feature/add-new-capability`
- `bugfix/fix-connection-issue`
- `docs/update-readme`

### Commit Message Guidelines

We follow a structured commit message format:

```
type(scope): concise description

[optional body with more detailed explanation]

[optional footer with breaking changes and issue references]
```

Types include:
- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation changes
- **style**: Formatting changes
- **refactor**: Code restructuring without changing external behavior
- **perf**: Performance improvements
- **test**: Adding or correcting tests
- **chore**: Maintenance tasks, dependency updates, etc.

Example:
```
feat(proxy): add connection pooling for mining pool connections

Implements a connection pool to reuse existing connections to mining pools.
This reduces connection overhead and improves performance under high load.

Closes #123
```

## Testing

- Write tests for all new functionality
- Run tests with `cargo test`
- Ensure all existing tests pass with your changes
- Include tests for both StratumV1 and StratumV2 protocol interactions where applicable
- Test performance impacts for bandwidth-sensitive code

## Protocol-Specific Testing

### StratumV1 Testing
- Test the translation proxy functionality for StratumV1 miners
- Verify proper job distribution and share submission
- Test difficulty adjustment mechanisms
- Validate error handling for malformed messages

### StratumV2 Testing
- Test the NOISE protocol authentication and encryption
- Verify template distribution mechanism
- Test connection handling and reconnection logic
- Validate the full job negotiation workflow

## Environment Variables

When testing and running the application, the following environment variables are available:

- `TOKEN` (Required): Your mining pool authentication token
- For testing, use: `oFzg1EUmceEcDuvzT3qt`
- Set with: `export TOKEN=<your_token>`

- `SV1_DOWN_LISTEN_ADDR` (Optional): Address for StratumV1 downstream connections
- Default: `0.0.0.0:32767`

- `TP_ADDRESS` (Optional): Template Provider address for job declaration

Always ensure you're running tests with appropriate environment variables set.
## Pull Request Process

1. **Before Opening a PR:**
- Ensure your code builds without errors
- Run all tests locally
- Format your code with `cargo fmt`
- Run `cargo clippy` and address warnings

2. **PR Requirements:**
- Include a clear description of the changes
- Reference any related issues

- Explain any non-obvious technical decisions

3. **Review Process:**
- PRs require at least one approval from a maintainer
- Address review feedback promptly
- Be responsive to questions about your implementation

4. **After Merge:**
- Monitor the CI pipeline to ensure your changes integrate well
- Help address any issues that may arise from your changes

## Documentation

- Update documentation to reflect your changes
- Update README.md if relevant
- Consider adding examples for complex functionality
- Document protocol-specific behaviors and configurations
- Update the environment variables section if you add new configuration options
- Include details on performance implications for significant changes

## Project-Specific Guidelines

### Protocol Implementation

- Follow the StratumV2 protocol specifications closely
- Maintain backward compatibility with StratumV1 miners
- Consider bandwidth and latency implications of changes
- Ensure security features like NOISE protocol are properly implemented
- Document any protocol extensions or modifications

### Performance Considerations

- Bitcoin mining is latency-sensitive; optimize for quick job distribution
- Consider memory usage patterns for high-volume deployments
- Test with realistic mining hashrates when implementing difficulty adjustment
- Profile code changes that affect core message handling paths

Thank you for contributing to demand-cli!
