// Okapi BM25 ranking algorithm

// Resources:
//  - https://en.wikipedia.org/wiki/Okapi_BM25
//  - https://xapian.org/docs/bm25.html

use crate::model;

pub fn score_okapi_bm25<T> (D: &model::Document,
                            Q: &model::Query,
                            C: &T) -> f64
                            where T: model::SearchContext {
    let mut score : f64 = 0.0;
    for i in 0..5 {
        score += 1.0;
    }
    score
}

// --- notes ---
// - Abstract over Document, and Query objects
