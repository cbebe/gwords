# G-Words

G-wordify Korean text:

```rust
gword("안녕하세요") // "아간녀겅하가세게요고"

gword("냉면") // "내갱며건"
```

## Background

G-words is a form of language game in Filipino, similar to Pig Latin in English. It alters the words by adding an extra "G" syllable after every syllable:

```
Magandang umaga (Ma-gan-dang u-ma-ga) // Good morning
Magagagandagang ugumagagaga (MaGA-gaGAn-daGAng uGU-maGA-gaGA)
```

Because all Filipino syllables follow the CVC (consonant-vowel-consonant), CV, or V patterns (Yes, 'ng' is its own consonant in Filipino), you can "G-wordify" any Filipino text using this simple Regular Expression:

```regex
s/([aeiou])/\1g\1/g
```

G-words can also be applied programmatically to other writing systems that follow this pattern.

This however does not work for English text:

```
food -> fogoogod X
food -> fogood
```

An interesting case is Hangul, which is a writing system for the Korean language. It uses an alphabet, but each combination of vowels and consonant has its own Unicode encoding. This means that to g-wordify a single letter (e.g. '글'), you will have to break it down into its components ('ㄱ', 'ㅡ', 'ㄹ'), then create two letters with those components ('그글').

## Testing

To test the library:

```sh
cargo test
```

### Code Coverage

To get code coverage, you will need `grcov`:

```sh
cargo install grcov
```

Then:

```sh
./scripts/cargo-coverage
```

Alternatively, if `./scripts` is in your `PATH`:

```sh
cargo coverage
```

