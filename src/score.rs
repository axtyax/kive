
mod score {

    mod structs;
    pub use structs::ts;

    fn inverse_document_frequency(term: &impl ts::Term,
                                  collection: &impl ts::Collection)
                                        -> ts::Score {
        0.0
    }

    fn term_frequency(term: &impl ts::Term,
                      document: &impl ts::Document)
                            -> i64 {
        0.0
    }

    pub fn best_match25(document: &impl ts::Document,
                query: &impl ts::Query,
                collection: &impl ts::Collection,
                free_params: &impl ts::params::best_match)
                    -> ts::Score {
        let mut score : f64 = 0;
        for i in 0..query.length() {
            score +=
                f64::from(
                    inverse_document_frequency(query.get_term(i), collection) *
                    (   (term_frequency(query.get_term(i), document) * (free_params.k(i) + 1) /
                        (term_frequency(query.get_term(i), document) + (free_params.k(i) *
                            (1.0 - free_params.b() + free_params.b() * document.length() / Collection.average_document_length())
                        ))
                    ))
                );
        }
    }

}
