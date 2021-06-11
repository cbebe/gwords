# G-Words

G-wordify Korean text:

```rust
gword("안녕하세요") // "아간녀겅하가세게요고"

gword("냉면") // "내갱며건"
```

## Background

## Testing

To test the library:

```
cargo test
```

To get code coverage, you will need `grcov`:

```
cargo install grcov
```

Then run:

```
cargo coverage

```
