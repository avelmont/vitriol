# Contributing Guide

Thank you for your interest in contributing to Vitriol! This guide will help you understand our development workflow, coding standards, and how to submit your contributions.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Git and Commits](#git-and-commits)
- [Testing](#testing)
- [Pull Requests](#pull-requests)

## Getting Started

### 1. Clone and Setup

```bash
git clone <repository-url>
cd vitriol
mise run setup
```

### 2. Verify Your Setup

Run the full pre-flight check:

```bash
mise run check
```

This ensures your environment is ready for development.

## Development Workflow

### Working on a Feature

1. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** in the appropriate crate:
   - `crates/lapis/` for main application changes
   - `crates/shared/` for shared utilities

3. **Format your code** before committing:
   ```bash
   mise run fmt
   ```

4. **Run tests locally** to verify your changes don't break anything:
   ```bash
   mise run test
   ```

5. **Run the full check** to catch any issues:
   ```bash
   mise run check
   ```

6. **Commit with conventional commits** (see [Git and Commits](#git-and-commits)):
   ```bash
   mise run commit
   ```

7. **Push your branch** and create a pull request

### Crate-Specific Work

When working on a specific crate:

```bash
# Build a specific crate
cargo build -p lapis

# Test a specific crate
cargo test -p lapis

# Lint a specific crate
cargo clippy -p lapis --all-targets --all-features -- -D warnings
```

## Code Standards

### Formatting

We use `rustfmt` for code formatting. The project enforces a consistent style.

**Before committing, always format:**

```bash
mise run fmt
```

### Linting

We use `clippy` with strict rules. All code must pass:

```bash
mise run lint
```

**Key rules enforced:**
- No unsafe code (forbidden by policy)
- No `.unwrap()` usage (generates warnings)
- Pedantic clippy checks must pass
- Missing documentation generates warnings
- No wildcard imports from enums

### Documentation

- Write doc comments for public items:
  ```rust
  /// Brief description of what this does.
  ///
  /// More detailed explanation if needed.
  pub fn my_function() {}
  ```

- Keep comments concise and focused

### Error Handling

Prefer explicit error handling over panicking:

```rust
//  Avoid
let value = result.unwrap();

// ✅ Prefer
let value = result.map_err(|e| {
    eprintln!("Error: {}", e);
    e
})?;
```

### Testing

Write tests for all public functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        // Arrange
        let input = "test";

        // Act
        let result = my_function(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

## Git and Commits

### Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/). All commits must follow this format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Code change that improves performance
- **test**: Adding or updating tests
- **chore**: Changes to build system, dependencies, etc.

### Examples

```
feat(lapis): add user authentication module

Implement JWT-based authentication with token refresh support.

Closes #42
```

```
fix(shared): correct error handling in parser

Handle edge case where empty input would panic.
```

```
docs(readme): update installation instructions
```

### Making Commits

Use the interactive commit tool:

```bash
mise run commit
```

This guides you through creating a properly formatted conventional commit.

You can also commit manually, but ensure your messages follow the conventional commit format:

```bash
git commit -m "feat(lapis): add new feature"
```

### Pre-commit Hooks

Git hooks automatically run before each commit:
- Format checking with `rustfmt`
- Linting with `clippy`

If these checks fail, the commit is blocked. Fix the issues and try again:

```bash
mise run fmt
mise run lint
git add .
git commit -m "..."
```

## Testing

### Running Tests

```bash
# Run all tests
mise run test

# Run tests for a specific crate
cargo test -p lapis

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with multiple threads (faster)
cargo test -- --test-threads=4
```

### Test Organization

Place tests in the same file as the code they test:

```rust
// src/my_module.rs

pub fn my_function() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert_eq!(my_function(), 42);
    }
}
```

## Testing CI Locally with Act

Before pushing, you can run the entire CI pipeline locally using [act](https://nektosact.com/), which simulates GitHub Actions in Docker.

### Installation

**macOS:**
```bash
brew install act
```

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

**Windows or manual installation:**
Visit [https://github.com/nektos/act/releases](https://github.com/nektos/act/releases)

### Using Act

```bash
# Run all CI jobs locally
act

# Run a specific job
act --job check          # Format check, linting, and tests
act --job commit-lint    # Commit message validation

# Run with verbose output
act --verbose

# Run on a push event (instead of pull_request)
act push

# Run with a specific event payload
act --eventpath .github/workflows/vitriol.yml
```

### Prerequisites

- **Docker**: Must be installed and running
- **Disk space**: Container images take ~2GB on first run
- **Permissions**: Your user should have permission to run Docker

### Troubleshooting Act

**"Docker socket not found"**
- Ensure Docker is running
- On Linux, you may need to add your user to the docker group: `sudo usermod -aG docker $USER`

**"Permission denied"**
- Run with sudo: `sudo act`
- Or configure Docker permissions (see above)

**Slow first run**
- Act downloads container images (~2GB) on first run. Subsequent runs are much faster.

### Recommended Workflow

Before committing and pushing:

```bash
mise run check
mise run commit
git push
# if your CI fails and you are uncertain run it locally
act
```

## Pull Requests

### Before Submitting

1. **Ensure your code passes all checks:**
   ```bash
   mise run check
   ```

2. **Update documentation** if your changes affect public APIs

3. **Rebase against main** to avoid merge conflicts:
   ```bash
   git fetch origin
   git rebase origin/main
   ```

### PR Guidelines

1. **Descriptive title**: Clearly describe what your PR does

2. **Clear description**:
   - What problem does this solve?
   - What changes did you make?
   - How can this be tested?

3. **Conventional commits**: Ensure all commits in your PR follow conventional commit format

4. **Tests**: Include tests for new functionality

5. **Documentation**: Update README or docs if applicable

### Review Process

- At least one approval is required before merging
- All CI checks must pass
- Keep feedback conversations professional and constructive

## Troubleshooting

### "format check failed"

Your code needs formatting:
```bash
mise run fmt
git add .
git commit --amend
```

### "clippy check failed"

Fix linting issues:
```bash
cargo clippy --fix --allow-dirty
mise run fmt
git add .
git commit --amend
```

### Pre-commit hook won't run

Ensure hooks are installed:
```bash
lefthook install
```

### Tool versions mismatch

Update tools to match project specifications:
```bash
mise install
```

## Questions?

If you have questions about contributing, feel free to:
- Check the [README.md](README.md) for project overview
- Review existing code for patterns and conventions
- Ask in an issue or discussion

Thank you for contributing!
