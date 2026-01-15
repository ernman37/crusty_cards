# Contributing to crusty_cards

Thank you for your interest in contributing to crusty_cards! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) (for coverage testing)

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rusty_cards.git
   cd rusty_cards
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/ernman37/rusty_cards.git
   ```
4. Build the project:
   ```bash
   cargo build
   ```

## Development Workflow

### Branching

- Create a feature branch from `main`:
  ```bash
  git checkout -b feature/your-feature-name
  ```
- Use descriptive branch names:
  - `feature/add-pinochle-deck`
  - `fix/shuffle-empty-deck`
  - `docs/improve-card-examples`

### Making Changes

1. Write your code
2. Add or update tests
3. Update documentation if needed
4. Run the checks (see below)

### Before Submitting

Run all checks locally:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Check coverage (should be ≥95%)
cargo llvm-cov
```

## Code Style

### Formatting

- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes

### Documentation

- All public items must have doc comments
- Include examples in doc comments where helpful
- Use `///` for item documentation
- Use `//!` for module-level documentation

Example:
```rust
/// Creates a new card with the given suit and rank.
///
/// # Examples
///
/// ```rust
/// use crusty_cards::{Card, Suit, Rank};
///
/// let ace = Card::new(Suit::Spades, Rank::Ace);
/// ```
pub fn new(suit: Suit, rank: Rank) -> Self {
    Card { suit, rank }
}
```

### Testing

- Write tests for all new functionality
- Place unit tests in the `tests/` directory
- Test edge cases and error conditions
- Maintain minimum 95% code coverage

Test file naming:
- `card_tests.rs` - tests for `Card`
- `deck_tests.rs` - tests for `Deck`
- etc.

## Pull Request Process

1. **Update your branch** with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Open a Pull Request** on GitHub

4. **PR Requirements**:
   - Clear description of changes
   - All CI checks passing
   - Code coverage ≥95%
   - Documentation updated (if applicable)
   - Tests added for new functionality

5. **Address review feedback** by pushing additional commits

## What to Contribute

### Good First Issues

Look for issues labeled `good first issue` for beginner-friendly tasks.

### Ideas for Contributions

- **New DeckFactory implementations** (e.g., Pinochle, Euchre, Blackjack)
- **New CardComparator implementations** (e.g., game-specific orderings)
- **Additional shuffle algorithms**
- **Performance improvements**
- **Documentation improvements**
- **Test coverage improvements**

### Out of Scope

The following are intentionally out of scope for this library:
- Game logic (hands, scoring, rules)
- GUI/TUI implementations
- Network/multiplayer functionality

This is a **library for cards and decks**, not a game engine.

## Reporting Issues

### Bug Reports

Include:
- Rust version (`rustc --version`)
- crusty_cards version
- Minimal code to reproduce
- Expected vs actual behavior

### Feature Requests

Include:
- Clear description of the feature
- Use case / motivation
- Example API (if applicable)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Assume good intentions

## License

By contributing, you agree that your contributions will be licensed under the [BEERWARE](LICENSE) license.

## Questions?

Open an issue or start a discussion on GitHub.

---

Thank you for contributing!
