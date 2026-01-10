# TODO - crusty_cards

## 🔴 High Priority

### Documentation
- [ ] **README needs real examples** - The `Example usage:` section just says "TBD"
- [ ] **Add crate-level documentation** - `lib.rs` should have `//!` doc comments explaining the library
- [ ] **Document public API** - Many public items lack doc comments (e.g., `Color`, `iter()`, `iter_mut()`)

### Package Metadata
- [ ] **Version is 0.0.0** - Update to proper semver (e.g., `0.1.0`) before publishing
- [ ] **Add repository URL** to `Cargo.toml`
- [ ] **Add authors field** to `Cargo.toml`
- [ ] **Crate name mismatch** - Repo is `rusty_cards` but crate is `crusty_cards` - intentional?

---

## 🟡 Medium Priority

### Missing Features
- [ ] **Hand struct** - A `Hand` abstraction for player hands (smaller collection, different semantics than Deck)
- [ ] **Card parsing from string** - `impl FromStr for Card` (e.g., "A♠" or "AS" → Card)
- [ ] **Rank parsing from string** - `impl FromStr for Rank`
- [ ] **Suit parsing from string** - `impl FromStr for Suit`
- [ ] **Default for Deck** - Should `Deck::default()` return empty or standard 52?
- [ ] **PartialEq for Deck** - Currently can't compare decks for equality
- [ ] **Eq/Hash for Deck** - Enable using Deck in HashSets/HashMaps

### Additional Factories
- [ ] **Pinochle deck** (48 cards - double deck without 2-8)
- [ ] **Euchre deck** (24 cards - 9-A only)
- [ ] **Blackjack shoe** (multiple decks combined)
- [ ] **Tarot deck** (78 cards with Major Arcana)

### Comparators
- [ ] **EuchreComparator** - Left bower (same color Jack) is second highest
- [ ] **PinochleComparator** - Different rank ordering

### Code Quality
- [ ] **Remove unnecessary clones in operators** - `Sub<Card>`, `SubAssign`, `Sub<Deck>` all clone excessively
- [ ] **Inefficient Mul implementation** - `add_card` is O(n), making `*` O(n²); use `extend` instead
- [ ] **lifetime elision** - `iter<'a>(&'a self)` can be simplified to `iter(&self)`

---

## 🟢 Low Priority / Nice to Have

### Features
- [ ] **Colored terminal output** - Use ANSI colors for red/black suits
- [ ] **Multiple ASCII art styles** - Compact, detailed, etc.
- [ ] **Deck::contains(&Card)** - Check if deck contains a specific card
- [ ] **Deck::find(&Card)** - Find position of card in deck
- [ ] **Deck::remove_at(index)** - Remove card at specific position
- [ ] **Deck::insert_at(index, Card)** - Insert card at specific position
- [ ] **Deck::reverse()** - Reverse deck order
- [ ] **Deck::split_at(index)** - Split into two decks
- [ ] **Deal multiple cards** - `deal_n(n: usize) -> Vec<Card>`
- [ ] **Riffle shuffle** - More realistic shuffle simulation
- [ ] **Overhand shuffle** - Another shuffle variant

### Testing
- [ ] **Property-based tests** - Use `proptest` or `quickcheck`
- [ ] **Fuzzing** - Fuzz the `TryFrom<usize>` implementations
- [ ] **Benchmark shuffling** - Performance comparison of shuffle methods
- [ ] **Test edge cases** - Empty deck operations, single-card deck, etc.

### CI/CD
- [ ] **GitHub Actions workflow** - Run tests, clippy, rustfmt on PRs
- [ ] **Code coverage** - Add coverage reporting (tarpaulin or llvm-cov)
- [ ] **Publish to crates.io** - Once stable

### Traits
- [ ] **Index trait** - `deck[0]` to get card at position
- [ ] **IndexMut trait** - `deck[0] = card` to set card at position
- [ ] **Extend trait** - `deck.extend(other_cards)`
- [ ] **FromIterator trait** - `cards.into_iter().collect::<Deck>()`

---

## 🐛 Bugs / Issues

- [ ] **Joker handling in Standard54** - Only adds 2 jokers (Hearts, Spades), but typical decks have 2 identical jokers. The suit assignment is arbitrary and may confuse users.
- [ ] **TryFrom<usize> allows 56 cards** - Error message mentions "4 jokers, one of each suit" but Standard54 only has 2 jokers. The 56-card mapping is inconsistent with factories.
- [ ] **Deck subtraction is O(n²)** - Each card removal is O(n), repeated n times

---

## 📝 Test Gaps

Based on current test coverage (96 tests), these areas need more testing:

- [ ] **Empty deck edge cases** - `cut(0)`, `shuffle()`, `deal()` on empty
- [ ] **Single card deck operations**
- [ ] **Deck * 0** - Should produce empty deck
- [ ] **Deck * 1** - Should be identical to original
- [ ] **Subtracting non-existent card** - What happens?
- [ ] **Clone verification** - Cloned deck is independent
- [ ] **Debug output format** - Verify Debug impl produces useful output
- [ ] **Serialization roundtrip with all card types** - Including jokers

---

## 🔧 Refactoring Suggestions

1. **Consider using `SmallVec` for Hand** - Hands are typically small (5-13 cards)
2. **Make Card fields public?** - Currently requires getters; consider `pub suit: Suit`
3. **Builder pattern for comparators** - `TrumpComparator::builder().trump(Suit::Hearts).ace_low(true).build()`
4. **Generic over card type** - `Deck<C: CardLike>` for custom card types

---

## 📚 Documentation Improvements

- [ ] Add module-level docs to `objects/mod.rs`, `traits/mod.rs`, `utils/mod.rs`
- [ ] Add examples to each public method
- [ ] Create a `examples/` directory with runnable examples
- [ ] Add a CHANGELOG.md
- [ ] Add CONTRIBUTING.md guidelines
