# Similar String
[![Crate](https://img.shields.io/crates/v/similar-string.svg)](https://crates.io/crates/similar-string)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/similar-string/)

...the library for finding string similarities 🔎
 
With this library you can easily find rate of similarity of two strings or array of strings.
Under the hood LCS (length finding variant) algorithm is used with O(n * m) time complexity and O(min(n, m)) memory complexity.

# Example
```rust
use similar_string::*;

// Compares similarity of two strings and returns similarity rating.
// The rating is returned as a f64 value in range from 0.0 to 1.0.
compare_similarity("age", "page"); // 0.75

let options = vec!["fill", "night", "ride"];

// The functions below return `None` if the provided slice is empty

// Finds the best match amongst the options
// and returns match with it's rating
find_best_similarity("fight", &options); // Some(("night", 0.8))

// Returns all the similarity ratings
// of the provided options
get_similarity_ratings("fight", &options); // Some([0.4, 0.8, 0.2])
```

# LCS Algorithm

You can also use the `lcs_length` that is used under the hood to compute length of longest common subsequence.

```rust
use similar_string::lcs_length;

// The longest common subsequence in this case is "one"
lcs_length("longest", "stone"); // 3
```

# Change log 🚀

## Version 1.4.1
### Fix:
- Case when given slice is empty is now handled with `Option`
- Improved code documentation

## Version 1.3.0
### Feature:
- Add `get_similarity_ratings` function that retrieves all the ratings of all options
- Improve algorithm to take O(min(n, m)) memory complexity

## Version 1.2.0
### Feature:
- Add `find_best_similarity` function that finds string that matches the target one best