// Okapi BM25 ranking algorithm

// Resources:
//  - https://en.wikipedia.org/wiki/Okapi_BM25
//  - https://xapian.org/docs/bm25.html

// --- notes ---
// - Abstract over Document, and Query objects

mod score;

#[cfg(test)]
mod tests {

    #[test]
    fn test_score() {
        assert_eq!(5.0, 5.0);
    }
}
