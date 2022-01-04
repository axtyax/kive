mod model;
mod okapi_bm25;

#[cfg(test)]
mod tests {

    use crate::model::Word;

    use super::*;

    #[test]
    fn test_score() {
        let score = okapi_bm25::score_okapi_bm25(
                &model::Document{words: Vec::new()},
                &model::Query{words: Vec::new()}
            );
        assert_eq!(score, 5.0);
    }
}
