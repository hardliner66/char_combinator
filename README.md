# char_combinator

An iterator to create all combination of a given char range.

e.g.:
```
a
b
c
...
aa
ab
ac
...
```

## Features

The iterator uses an u128 as internal counter. If you need more strings than that, you can use the `bigint` feature.

## Example
```rust
fn main() {
    for (i, s) in CharCombinator::default().take(104).enumerate() {
        println!("{}: {}", i, s);
    }
}
```