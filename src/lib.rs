#![warn(missing_docs)]

//! # Similar String - the library for finding string similarities
//! 
//! With this library you can easily find rate of similarity of two strings or array of strings.
//! Under the hood LCS (length finding variant) algorithm is used with O(n * m) time complexity and O(min(n, m)) memory complexity.
//! 
//! # Example
//! ```
//! use similar_string::*;
//! 
//! // Compares similarity of two strings and returns similarity rating.
//! // The rating is returned as a f64 value in range from 0.0 to 1.0.
//! compare_similarity("age", "page"); // 0.75
//! 
//! let options = vec!["fill", "night", "ride"];
//! 
//! // Finds the best match amongst the options
//! // and returns match with it's rating
//! find_best_similarity("fight", &options); // ("night", 0.8)
//! 
//! // Returns all the similarity ratings
//! // of the provided options
//! get_similarity_ratings("fight", &options); // [0.4, 0.8, 0.2]
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
fn get_shorter_longer_strings(left: impl AsRef<str>, right: impl AsRef<str>) -> (String, String) {
    if left.as_ref().len() < right.as_ref().len() {
        (left.as_ref().to_string(), right.as_ref().to_string())
    } else {
        (right.as_ref().to_string(), left.as_ref().to_string())
    }
}

/// Get length of the longest common subsequence
pub fn lcs_length(left: impl AsRef<str>, right: impl AsRef<str>) -> usize {
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
pub fn compare_similarity(left: impl AsRef<str>, right: impl AsRef<str>) -> f64 {
    let (len1, len2) = (left.as_ref().len(), right.as_ref().len());
    let lcs_len = lcs_length(left.as_ref(), right.as_ref());
    let size = max(len1, len2);
    lcs_len as f64 / size as f64
}

/// Find the string amongs the options that is the most similar to the target one
pub fn find_best_similarity(taregt: impl AsRef<str>, options: &[impl AsRef<str>]) -> (String, f64) {
    let mut high_score: f64 = -1.0;
    let mut position: usize = 0;
    for (index, option) in options.iter().enumerate() {
        let score = compare_similarity(option.as_ref(), taregt.as_ref());
        if score > high_score {
            high_score = score;
            position = index;
        }
    }
    (options[position].as_ref().to_string(), high_score)
}

/// Get all similarity scores against the target string
pub fn get_similarity_ratings(taregt: impl AsRef<str>, options: &[impl AsRef<str>]) -> Vec<f64> {
    let mut result = vec![];
    for option in options.iter() {
        let score = compare_similarity(option.as_ref(), taregt.as_ref());
        result.push(score);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, vec};

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

    #[test]
    fn find_best_with_set() {
        let target = format!("fight");
        let mut options = BTreeSet::new();
        options.insert("blight");
        options.insert("night");
        options.insert("stride");
        let vector: Vec<_> = options.iter().collect();
        let (matched, score) = find_best_similarity(target, &vector);
        assert_eq!(matched, "night");
        assert_eq!(score, 0.8);
    }

    #[test]
    fn similarity_ratings() {
        let expected = vec![0.4, 0.8, 0.2];
        let options = vec![
            "fill",
            "night",
            "ride"
        ];
        let ratings = get_similarity_ratings("fight", &options);
        assert_eq!(expected, ratings);
    }
}
