use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String
}

#[derive(sqlx::FromRow, Serialize)]
pub struct GetNoteSchema {
    pub note_id: String,
    pub title: String,
    pub content: String,
    pub done: bool
}