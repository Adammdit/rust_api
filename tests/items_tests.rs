use actix_web::{test, App};
use rust_api::{routes, state::AppState};
use actix_web::web;

#[actix_rt::test]
async fn test_create_and_get_item() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(routes::create_item)
            .service(routes::get_item)
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(&serde_json::json!({ "name": "TestItem" }))
        .to_request();

    let created: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let id = created["id"].as_str().unwrap();

    let req = test::TestRequest::get()
        .uri(&format!("/items/{}", id))
        .to_request();

    let fetched: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(fetched["name"], "TestItem");
}
