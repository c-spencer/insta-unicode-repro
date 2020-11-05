Reproduction repo for issue https://github.com/mitsuhiko/insta/issues/137

Tested on:

- `insta`/`insta-cargo` 1.1.0
- Rust 1.47.0
- Arch Linux / MacOS

### Steps

- `cargo install cargo-insta`
- `RUST_BACKTRACE=1 cargo insta test --accept`

```
thread 'main' panicked at 'byte index 78 is not a char boundary; it is inside '∃' (bytes 76..79) of `        @"(∀ν : ℕ. (∀α : ⋆. ((α → (1 + 1)) → (Vec ν α → (∃κ : ℕ. Vec κ α)))))"`', /Users/chris/.cargo/registry/src/github.com-1ecc6299db9ec823/cargo-insta-1.1.0/src/inline.rs:79:22
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::str::slice_error_fail
   3: cargo_insta::inline::FilePatcher::set_new_content
   4: cargo_insta::cargo::SnapshotContainer::commit
   5: cargo_insta::cli::process_snapshots
   6: cargo_insta::cli::run
   7: cargo_insta::main
```
