use axum::{extract::Json, extract::Path, extract::State, routing::get, routing::post, Router};

use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use gcode_wrangler::MachineType;

type Handle = u64;

#[derive(Clone, Deserialize, Default, Serialize)]
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

#[derive(Deserialize, Hash, Default)]
struct Movement {
    dest: Vec2D,
    pen_down: bool,
}

#[derive(Clone, Default)]
struct AppState {
    cached_movements: Arc<Mutex<HashMap<u64, Vec<Movement>>>>,
    machine_details: MachineDetails,
}

#[derive(Clone, Default, Serialize)]
struct MachineDetails {
    dimensions: Vec2D,
    flavor: String,
    device: String,
}

impl From<HashMap<String, String>> for MachineDetails {
    fn from(fromval: HashMap<String, String>) -> Self {
        MachineDetails {
            dimensions: Vec2D {
                x: fromval
                    .get("xdim")
                    .expect("Missing config value: xdim")
                    .parse()
                    .unwrap(),
                y: fromval
                    .get("ydim")
                    .expect("Missing config value: ydim")
                    .parse()
                    .unwrap(),
            },
            flavor: {
                if let Some(maybe_match) = fromval.get("flavor") {
                    match maybe_match.as_str() {
                        "GRBL" => "GRBL",
                        "Marlin" => "Marlin",
                        unknown => {
                            panic!("Unknown gcode flavor: {unknown}")
                        }
                    }
                    .to_string()
                } else {
                    panic!("Missing config value: flavor")
                }
            },
            device: fromval
                .get("name")
                .expect("Missing config value: name")
                .to_owned(),
        }
    }
}

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("machine_settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let machine: MachineDetails = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
        .into();

    let state = AppState {
        machine_details: machine,
        ..Default::default()
    };

    let app = Router::new()
        .route("/run", get(get_run).post(post_run))
        .route("/analysis", get(get_analysis))
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

async fn get_machine(State(state): State<AppState>) -> Json<MachineDetails> {
    axum::Json(state.machine_details)
}

async fn post_movements(
    State(state): State<AppState>,
    Json(movements): Json<Vec<Movement>>,
) -> String {
    let mut s = DefaultHasher::new();
    movements.hash(&mut s);
    let hash = s.finish();
    state
        .cached_movements
        .lock()
        .unwrap()
        .insert(hash, movements);
    hash.to_string()
}

async fn post_pause(Path(handle): Path<Handle>) {}
async fn post_run(Path(handle): Path<Handle>) {}
async fn get_run(Path(handle): Path<Handle>) {}
async fn get_analysis(Path(handle): Path<Handle>) {}
