use crate::model::note::{Importance, Note, NoteForm};
use axum::Json;
use chrono::Utc;

pub async fn get_notes() -> Json<Vec<Note>> {
    println!("{}: get_notes", Utc::now());
    Json(vec![
        Note {
            id: uuid::Uuid::new_v4(),
            title: "Erstes Note".to_string(),
            content: "Mach mal dies oder das".to_string(),
            done: true,
            importance: Importance::Unimportant,
            date: Some(Utc::now()),
        },
        Note {
            id: uuid::Uuid::new_v4(),
            title: "Zweites Note".to_string(),
            content: "Mach mal dies oder das".to_string(),
            done: false,
            importance: Importance::Important,
            date: Some(Utc::now()),
        },
    ])
}

pub async fn create_note(Json(mut note): Json<NoteForm>) -> Json<NoteForm> {
    Json(NoteForm {
        title: note.title.to_string(),
        content: note.content.to_string(),
        done: false,
        importance: Importance::Important,
        date: Some(Utc::now()),
    })
}
