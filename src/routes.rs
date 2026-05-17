use actix_web::{get, post, delete, web, Responder};
use crate::{state::AppState, errors::ApiError};
use uuid::Uuid;

#[get("/health")]
pub async fn health() -> impl Responder {
    "OK"
}

#[get("/items")]
pub async fn list_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    web::Json(items.clone())
}

#[post("/items")]
pub async fn create_item(
    data: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> Result<impl Responder, ApiError> {
    let name = body.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ApiError::BadRequest("Missing name".into()))?;

    let item = data.add_item(name.into());
    Ok(web::Json(item))
}

#[get("/items/{id}")]
pub async fn get_item(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = Uuid::parse_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID".into()))?;

    let items = data.items.lock().unwrap();
    let item = items.iter().find(|i| i.id == id).ok_or(ApiError::NotFound)?;

    Ok(web::Json(item.clone()))
}

#[delete("/items/{id}")]
pub async fn delete_item(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = Uuid::parse_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID".into()))?;

    let mut items = data.items.lock().unwrap();
    let len_before = items.len();
    items.retain(|i| i.id != id);

    if items.len() == len_before {
        return Err(ApiError::NotFound);
    }

    Ok("Deleted")
}
