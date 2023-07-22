use axum::{
    routing::get, routing::post,
    extract::Path, extract::Json, extract::State,
    Router,
};

use serde::Deserialize;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;



type Handle = u64;

#[derive(Deserialize)]
struct Vec2D {
    x: f32,
    y: f32,
}

impl Hash for Vec2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_le_bytes().hash(state);
        self.y.to_le_bytes().hash(state);
    }
}


#[derive(Deserialize, Hash)]
struct Movement {
    dest: Vec2D,
    pen_down: bool
}

#[derive(Clone)]
struct AppState {}


#[tokio::main]
async fn main() {

    let state = AppState {};

    let app = Router::new()
    .route("/run", get(get_run).post(post_run))
    .route("/analysis", get(get_analysis).post(post_analysis))
    .route("/movements", post(post_movements))
    .route("/pause", post(post_pause))
    .route("/machine", get(get_machine))
    .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_machine() {}
async fn post_movements(State(state): State<AppState>, Json(movements): Json<Vec<Movement>>) -> String {
    let mut s = DefaultHasher::new();
    movements.hash(&mut s);
    let hash = s.finish();

    hash.to_string()
}
async fn post_pause(Path(handle): Path<Handle>) {}
async fn post_run(Path(handle): Path<Handle>) {}
async fn get_run(Path(handle): Path<Handle>) {}
async fn post_analysis(Path(handle): Path<Handle>) {}
async fn get_analysis(Path(handle): Path<Handle>) {}
