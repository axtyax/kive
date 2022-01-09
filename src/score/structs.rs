
pub mod ts {

    pub struct Term {
        pub data: String,
    }

    impl Term {
        pub fn from(term_str: String) -> Term {
            Term {
                data: term_str,
            }
        }
    }

    pub trait Query {
        fn get_term(&self, _: usize) -> &Term;
        fn length(&self) -> usize;
    }

    pub trait Document {
        fn length(&self) -> usize;
    }

    pub trait Collection {
        fn average_document_length(&self) -> f64;
    }


    pub trait Score {
        fn foo(&self) -> f64;
    }

    pub mod params {

        pub struct BestMatch {
            pub k : Vec<f64>,
            pub b : f64,
        }

    }

}
