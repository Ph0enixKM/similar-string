#![warn(missing_docs)]

//! # Similar String - the library for finding string similarities
//! 
//! With this library you can easily find rate of similarity of two strings or array of strings.
//! Under the hood LCS (length finding variant) algorithm is used with O(n * m) time complexity and O(1) memory complexity.
//! 
//! # Example
//! ```
//! use similar_string::{compare_similarity, find_best_similarity};
//! 
//! compare_similarity("age", "page"); // 0.75
//! find_best_similarity("fight", &vec!["blight", "night", "stride"]); // ("night", 0.8)
//! ```
//! 
//! # LCS Algorithm
//! 
//! You can also use the `lcs_length` that is used under the hood to compute length of longest common subsequence.
//! 
//! ```
//! use similar_string::lcs_length;
//! 
//! // The longest common subsequence in this case is "one"
//! lcs_length("longest", "stone"); // 3
//! ```

use std::cmp::max;

#[inline]
fn get_shorter_longer_strings<S: AsRef<str>>(left: S, right: S) -> (String, String) {
    (left.as_ref().to_string(), right.as_ref().to_string())
}

/// Get length of the longest common subsequence
pub fn lcs_length<S: AsRef<str>>(left: S, right: S) -> usize {
    let (left, right) = get_shorter_longer_strings(left, right);
    let mut table = vec![vec![0 as usize; left.len() + 1]; 2];

    for rletter in right.chars() {
        for (col, lletter) in left.chars().enumerate() {
            if rletter == lletter {
                table[1][col + 1] = 1 + table[0][col];
            } else {
                table[1][col + 1] = max(table[0][col + 1], table[1][col]);
            }
        }
        table[0] = table.pop().unwrap();
        table.push(vec![0 as usize; left.len() + 1]);
    }
    *table[0].last().unwrap()
}

/// Get score of similarity of two certain strings
pub fn compare_similarity<S: AsRef<str>>(left: S, right: S) -> f64 {
    let (len1, len2) = (left.as_ref().len(), right.as_ref().len());
    let lcs_len = lcs_length(left, right);
    let size = max(len1, len2);
    lcs_len as f64 / size as f64
}

/// Find the string amongs the options that is the most similar to the target one
pub fn find_best_similarity<S: AsRef<str>>(taregt: S, options: &[S]) -> (String, f64) {
    let mut high_score: f64 = -1.0;
    let mut position: usize = 0;
    for (index, option) in options.iter().enumerate() {
        let score = compare_similarity(option, &taregt);
        if score > high_score {
            high_score = score;
            position = index;
        }
    }
    (options[position].as_ref().to_string(), high_score)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn lcs_works() {
        let length = lcs_length("longest", "stone");
        assert_eq!(length, 3);
    }

    #[test]
    fn identity_check() {
        let score = compare_similarity("hello", "hello");
        assert_eq!(score, 1.0);
    }

    #[test]
    fn ratio_is_symetrical() {
        let left = "longest";
        let right = "stone";
        let score1 = compare_similarity(left, right);
        let score2 = compare_similarity(right, left);
        assert_eq!(score1, score2);
    }

    #[test]
    fn find_best() {
        let target = "fight";
        let options = vec!["blight", "night", "stride"];
        let (matched, score) = find_best_similarity(target, &options);
        assert_eq!(matched, "night");
        assert_eq!(score, 0.8);
    }
}
