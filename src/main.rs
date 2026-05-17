use actix_web::{App, HttpServer, web};
use rust_api::routes;
use rust_api::state::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting on http://127.0.0.1:8080");
    let state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(routes::health)
            .service(routes::list_items)
            .service(routes::create_item)
            .service(routes::get_item)
            .service(routes::delete_item)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
