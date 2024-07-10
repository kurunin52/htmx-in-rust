use crate::model::order;
use crate::view::html;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/order")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    let orders = order::get_order_balance_list(&state.pool).await;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_index_page(&orders))
}

#[derive(Deserialize)]
struct SearchQuery {
    status: String,
}

#[get("/order/search")]
async fn search(query: web::Query<SearchQuery>, state: web::Data<AppState>) -> impl Responder {
    let orders = if &query.status != "" {
        order::search_order_balance(&state.pool, &query.status).await
    } else {
        order::get_order_balance_list(&state.pool).await
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html::order::render_order_rows(&orders))
}
