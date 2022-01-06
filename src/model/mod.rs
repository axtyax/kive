
pub struct Document {
    pub words: Vec<Word>,
}

impl Document {
    pub fn length(&self) -> i32 {
        self.length()
    }

    pub fn from(s: &String) -> Document {
        Document {
            words:  s.split(' ')
                     .map(|s| Word::from(s.to_string()) )
                     .collect::<Vec<Word>>(),
        }
    }
}

pub struct Query {
    pub words: Vec<Word>,
}

impl Query {
    pub fn length(&self) -> i32 {
        self.length()
    }
}

pub struct Word {
    pub data: String,
}

// dummy change haha
impl Word {
    pub fn from(str: String) -> Word {
        Word {
            data: str,
        }
    },
}

