// Okapi BM25 ranking algorithm

// Resources:
//  - https://en.wikipedia.org/wiki/Okapi_BM25
//  - https://xapian.org/docs/bm25.html

use crate::model;

pub fn inverse_document_frequency
        (D: &model::Document, q: &model::Word) -> f64
{
    0.0
}

pub fn score_okapi_bm25(D: &model::Document, Q: &model::Query) -> f64 {
    let mut score : f64 = 0.0;
    for i in 0..5 {
        score += 1.0;
    }
    score
}


// --- notes ---
// - Abstract over Document, and Query objects
