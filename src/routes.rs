use actix_web::{ get, post, patch, delete, web, HttpResponse, Responder };
use serde_json::json;
use uuid::Uuid;
use crate::{AppState, schema::{CreateNoteSchema, UpdateNoteSchema}, dao::{create_note, query_note, query_all_notes, update_note, delete_note}};

#[get("/health")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
    .json(json!({
        "status": 200,
        "alive": true
    }))
}

#[get("/notes")]
async fn get_all_notes(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.pool;
    match query_all_notes(pool).await {
        Ok(notes) => {
            HttpResponse::Ok().json(json!({
                "status": 200,
                "success": true,
                "notes": notes,
                "total_notes": notes.len()
            }))
        }
        Err(_) => {
            HttpResponse::InternalServerError()
            .json(json!({
                "status": 500,
                "success": false
            })) 
        }
    }
}

#[get("/notes/{id}")]
async fn get_note(form: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let id = form.to_string();
    let pool = &data.pool;
    match query_note(pool, id).await {
        Ok(Some(note)) => {
            Ok(HttpResponse::Ok()
            .json(json!({
                "status": 200,
                "success": true,
                "note": note
            })))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(json!({
                "status": 404,
                "success": false,
                "error": "Note not found"
            })))
        }
        Err(_) => {
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "success": false,
                "error": "Internal server error"
            })))
        }
    }
}

#[post("/notes")]
async fn post_note(body: web::Json<CreateNoteSchema>, data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let pool = &data.pool;
    let note_uuid = Uuid::new_v4().to_string();
    let title = body.title.to_string();
    let content = body.content.to_string();
    if title.len() < 1 || title.len() > 33 {
        return Ok(HttpResponse::BadRequest()
        .json(json!({
            "status": 400,
            "success": false,
            "message": "Invalid title length"
        })));
    }
    if content.len() < 1 || content.len() > 500 {
        return Ok(HttpResponse::BadRequest()
        .json(json!({
            "status": 400,
            "success": false,
            "message": "Invalid content length"
        })));
    }
    
    match create_note(pool, &note_uuid, &title, &content, false).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(json!({
                "status": 200,
                "success": true,
                "message": "Note created successfully",
                "note_id": &note_uuid
            })))
        }
        Err(_) => {
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "success": false,
                "error": "Internal server error"
            })))
        }
    }

}

#[patch("/notes")]
async fn patch_note(body: web::Json<UpdateNoteSchema>, data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let pool = &data.pool;

    let note_id = body.note_id.to_string();
    let title = body.title.to_string();
    let content = body.content.to_string();
    let done = body.done;

    if title.len() < 1 || title.len() > 33 {
        return Ok(HttpResponse::BadRequest()
        .json(json!({
            "status": 400,
            "success": false,
            "message": "Invalid title length"
        })));
    }

    if content.len() < 1 || content.len() > 500 {
        return Ok(HttpResponse::BadRequest()
        .json(json!({
            "status": 400,
            "success": false,
            "message": "Invalid content length"
        })));
    }

    match update_note(
        pool,
        UpdateNoteSchema { note_id, title, content, done }
    ).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(json!({
                "status": 200,
                "success": true,
                "message": "Note successfully updated"
            })))
        }
        Err(_) => {
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "success": false,
                "error": "Internal server error"
            })))
        }
    }

}

#[delete("/notes/{id}")]
async fn del_note(form: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let id = form.to_string();
    let pool = &data.pool;
    match delete_note(
        pool,
        &id
    ).await {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(json!({
                "status": 200,
                "success": true,
                "message": "Note successfully deleted"
            })))
        }
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(json!({
                "status": 404,
                "success": false,
                "error": "Note not found"
            })))
        }
        Err(_) => {
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "success": false,
                "error": "Internal server error"
            })))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(get_note);
    cfg.service(get_all_notes);
    cfg.service(post_note);
    cfg.service(patch_note);
    cfg.service(del_note);
}