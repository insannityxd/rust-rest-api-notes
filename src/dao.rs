use sqlx::Error;

use crate::schema::{NoteSchema, UpdateNoteSchema};

pub async fn query_all_notes(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<NoteSchema>, Error> {
    let query = sqlx::query_as::<_, NoteSchema>("SELECT * FROM notes")
        .fetch_all(pool)
        .await?;
    Ok(query)
}

pub async fn query_note(pool: &sqlx::Pool<sqlx::Postgres>, note_id: String) -> Result<Option<NoteSchema>, Error> {
    let query = sqlx::query_as!(
        NoteSchema,
        r#"
        SELECT note_id, title, content, done 
        FROM notes 
        WHERE note_id = $1
        "#
        , note_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(query)
}

pub async fn create_note(pool: &sqlx::Pool<sqlx::Postgres>, note_id: &str, title: &str, content: &str, done: bool) -> Result<(), Error> {
    sqlx::query(
        r#"
        INSERT INTO notes (note_id, title, content, done) 
        VALUES ($1, $2, $3, $4)
        "#
    )
    .bind(note_id)
    .bind(title)
    .bind(content)
    .bind(done)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_note(pool: &sqlx::Pool<sqlx::Postgres>, note: UpdateNoteSchema) -> Result<(), Error> {
    sqlx::query(
        r#"
        UPDATE notes
        SET title = $1, content = $2, done = $3
        WHERE note_id = $4
        "#
    )
    .bind(&note.title)
    .bind(&note.content)
    .bind(&note.done)
    .bind(&note.note_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_note(pool: &sqlx::Pool<sqlx::Postgres>, note_id: &str) -> Result<bool, Error> {
    let query = sqlx::query(
        r#"
        DELETE FROM notes
        WHERE note_id = $1
        RETURNING note_id
        "#
    )
    .bind(note_id)
    .fetch_optional(pool)
    .await?;
    Ok(query.is_some())
}