
pub enum DocumentType {
    
}

pub struct Document<T> {
    metadata: T::MetaData,
    content: String,
}
