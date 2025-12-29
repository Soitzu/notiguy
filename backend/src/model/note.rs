use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub done: bool,
    pub importance: Importance,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteForm {
    pub title: String,
    pub content: String,
    pub done: bool,
    pub importance: Importance,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Importance {
    Important,
    Unimportant,
}
