use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LegalDocument {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    text: String,
    context: String,
    model: String,
}
