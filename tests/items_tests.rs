use actix_web::{test, App};
use rust_api::{routes, state::AppState};
use actix_web::web;

#[actix_rt::test]
async fn test_create_and_get_item() {
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(routes::create_item)
            .service(routes::get_item)
    )
    .await;

    let created: serde_json::Value = test::read_body_json(
        test::TestRequest::post()
            .uri("/items")
            .set_json(&serde_json::json!({ "name": "TestItem" }))
            .send_request(&mut app)
            .await
    ).await;
    let id = created["id"].as_str().unwrap();

    let fetched: serde_json::Value = test::read_body_json(
        test::TestRequest::get()
            .uri(&format!("/items/{}", id))
            .send_request(&mut app)
            .await
    ).await;

    assert_eq!(fetched["name"], "TestItem");
}
