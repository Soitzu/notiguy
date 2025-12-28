use axum::Json;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};




#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: Uuid,
    title: String,
    content: String,
    done: bool,
    importance: Importance,
    date: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note_Form {
    title: String,
    content: String,
    done: bool,
    importance: Importance,
    date: Option<DateTime<Utc>>
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Importance {
    Important,
    Unimportant,
}

pub async fn get_notes() -> Json<Vec<Note>> {
    println!("{}: get_notes", Utc::now());
    Json(
        vec![
            Note {
                id: uuid::Uuid::new_v4(),
                title: "Erstes Note".to_string(),
                content: "Mach mal dies oder das".to_string(),
                done: true,
                importance: Importance::Unimportant,
                date: Some(Utc::now())
            },
            Note {
                id: uuid::Uuid::new_v4(),
                title: "Zweites Note".to_string(),
                content: "Mach mal dies oder das".to_string(),
                done: false,
                importance: Importance::Important,
                date: Some(Utc::now())
            }
        ])
}

pub async fn create_note(Json(mut note): Json<Note_Form>) -> Json<Note_Form> {
    Json(Note_Form{
        title: note.title.to_string(),
        content: note.content.to_string(),
        done: false,
        importance: Importance::Important,
        date: Some(Utc::now())
    })
}