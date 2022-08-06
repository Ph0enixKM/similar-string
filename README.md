# Similar String
[![Crate](https://img.shields.io/crates/v/similar-string.svg)](https://crates.io/crates/similar-string)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/similar-string/)

...the library for finding string similarities 🔎
 
With this library you can easily find rate of similarity of two strings or array of strings.
Under the hood LCS (length finding variant) algorithm is used with O(n * m) time complexity and O(1) memory complexity.

# Example
```rust
use similar_string::{compare_similarity, find_best_similarity};

compare_similarity("age", "page"); // 0.75
find_best_similarity("fight", &vec!["blight", "night", "stride"]); // ("night", 0.8)
```

# LCS Algorithm

You can also use the `lcs_length` that is used under the hood to compute length of longest common subsequence.

```rust
use similar_string::lcs_length;

// The longest common subsequence in this case is "one"
lcs_length("longest", "stone"); // 3
```