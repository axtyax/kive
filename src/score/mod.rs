

mod structs;
pub use structs::ts;

fn inverse_document_frequency(term: &ts::Term,
                              collection: &impl ts::Collection)
                                    -> f64 {
    0.0
}

fn term_frequency(term: &ts::Term,
                  document: &impl ts::Document)
                        -> f64 {
    0.0
}

pub fn best_match25(document: &impl ts::Document,
            query: &impl ts::Query,
            collection: &impl ts::Collection,
            free_params: ts::params::BestMatch)
                -> f64 {
    let mut score : f64 = 0.0;
    for i in 0..query.length() {
        score +=
            f64::from(
                inverse_document_frequency(query.get_term(i), collection) *
                (   (term_frequency(query.get_term(i), document) * (free_params.k[i] + 1.0) /
                    (term_frequency(query.get_term(i), document) + (free_params.k[i]) *
                        (1.0 - free_params.b + free_params.b * f64::from(document.length() as i32) / collection.average_document_length())
                    )
                ))
            );
    }
    score
}

