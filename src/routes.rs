use actix_web::{delete, get, patch, post, web, Responder};
use crate::{errors::ApiError, state::AppState};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub completed: Option<bool>,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    "OK"
}

#[get("/items")]
pub async fn list_items(
    data: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> impl Responder {
    let items = data.list_items(query.completed);
    web::Json(items)
}

#[post("/items")]
pub async fn create_item(
    data: web::Data<AppState>,
    body: web::Json<CreateItemRequest>,
) -> Result<impl Responder, ApiError> {
    let item = data.add_item(body.name.clone(), body.description.clone());
    Ok(web::Json(item))
}

#[patch("/items/{id}")]
pub async fn update_item(
    data: web::Data<AppState>,
    id: web::Path<String>,
    body: web::Json<UpdateItemRequest>,
) -> Result<impl Responder, ApiError> {
    let id = Uuid::parse_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID".into()))?;

    let item = data
        .update_item(id, body.name.clone(), body.description.clone(), body.completed)
        .ok_or(ApiError::NotFound)?;

    Ok(web::Json(item))
}

#[get("/items/{id}")]
pub async fn get_item(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = Uuid::parse_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID".into()))?;

    let item = data.get_item(id).ok_or(ApiError::NotFound)?;
    Ok(web::Json(item))
}

#[delete("/items/{id}")]
pub async fn delete_item(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = Uuid::parse_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID".into()))?;

    if !data.delete_item(id) {
        return Err(ApiError::NotFound);
    }

    Ok("Deleted")
}
