// Okapi BM25 ranking algorithm

// Resources:
//  - https://en.wikipedia.org/wiki/Okapi_BM25
//  - https://xapian.org/docs/bm25.html

fn inverse_document_frequency(D: &Document, q: &Query::Keyword) -> f64 {
    0.0
}

fn score_okapi_bm25(D: &Document, Q: &Query) -> f64 {
    let mut score : f64 = 0.0;
    for i in 1..10 {
    }
    score
}


// --- notes ---
// - Abstract over Document, and Query objects
