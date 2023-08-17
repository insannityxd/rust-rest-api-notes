use sqlx::Error;

use crate::schema::GetNoteSchema;

pub async fn create_note(pool: &sqlx::Pool<sqlx::Postgres>, note_id: &str, title: &str, content: &str, done: bool) -> Result<(), Error> {
    let _query = sqlx::query(
        "
        INSERT INTO notes (note_id, title, content, done) 
        VALUES ($1, $2, $3, $4)
        "
    )
        .bind(note_id)
        .bind(title)
        .bind(content)
        .bind(done)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn query_all_notes(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<GetNoteSchema>, Error> {
    let query = sqlx::query_as::<_, GetNoteSchema>("SELECT * FROM notes")
        .fetch_all(pool)
        .await?;
    Ok(query)
}

pub async fn query_note(pool: &sqlx::Pool<sqlx::Postgres>, id: String) -> Result<Option<GetNoteSchema>, Error> {
    let query = sqlx::query_as!(
        GetNoteSchema,
        r#"
        SELECT note_id, title, content, done 
        FROM notes 
        WHERE note_id = $1
        "#
        , id
    )
    .fetch_optional(pool)
    .await?;
    Ok(query)
}