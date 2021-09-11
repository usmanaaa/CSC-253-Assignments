# Rust Iterators - Hung Vu

## Organization

All project code is in `./src/lib.rs`.

- Part 1: All the tests are in the `tests` module, specifically:
  - any: `test_any()`
  - chain: `test_chain()`
  - collect: various functions, including `test_chain()` and `test_scan`
  - partition: `test_partition()`
  - try_fold: `test_try_fold()`
  - find_map: `test_find_map()`
  - rposition: `test_rposition()`
  - max_by: `test_max_by()`
  - unzip: `test_unzip()`
  - scan: `test_scan()`
  - flat_map: `test_flat_map()`
  - cycle: `test_cycle()`
- Part 2: All 3 functions are at the top of the file:
  - a. `pairing()`
  - b. `frequency_count()`
  - c. `calendar()`
  - Tests are in `tests` module, 3 for each function

## Run

```
cargo run
```
for a quick demonstration of the 3 functions in part 2.

```
cargo test
```
to run all tests.
