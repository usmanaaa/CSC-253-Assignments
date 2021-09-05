# Box Vector 1 - Hung Vu

## Organization

All project code is in `./src/lib.rs`

Specifically, `BoxVec` and its methods are implemented at the top of `lib.rs`.

At the top of the `tests` module are test functions for BoxVec (2+ per method, as specified in the prompt).

Below that are `test_option()` and `test_result()`, two test functions that contain examples for all specified 11 methods of Option: and, or, flatten, map, map or, map or else, ok or, transpose, unwrap, unwrap or, zip, and all specified 10 methods of Result: ok, err, and, or, unwrap, expect, unwrap or, unwrap err, map err, transpose.

## Run

```
cargo run
```
for a quick demonstration of BoxVec.

```
cargo test
```
to run all tests.
