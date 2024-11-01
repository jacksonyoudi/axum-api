use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn create_route() -> Router {
    Router::new()
        .route("/cats", post(create_cat))
        .route("/cats", get(query_cats))
        .route("/cats/:id", get(get_cat_by_id))
        .route("/cats/:id", delete(remove_cat_by_id))
        .route("/cats/:id", put(update_cat_by_id))
}

