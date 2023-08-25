use axum::http::StatusCode;
use axum::{extract::Json, extract::Path, extract::State, routing::get, routing::post, Router};

use config::Config;
use gcode_wrangler::models::{MachineDetails, Movement};
use gcode_wrangler::{to_gcode, to_program, GCode, PortCmd, Position, SerialChannel};
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;

type Handle = u64;

#[derive(Clone)]
pub struct AppState {
    movements: Arc<Mutex<HashMap<Handle, Vec<Movement>>>>,
    cached_gcode: Arc<Mutex<HashMap<Handle, Vec<GCode>>>>,
    machine_details: MachineDetails,
    progress: Receiver<usize>,
    cmd_channel: Sender<PortCmd>,
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

    let maybe_channel: Result<
        (
            tokio::sync::watch::Receiver<usize>,
            Sender<PortCmd>,
            SerialChannel,
        ),
        serialport::Error,
    > = SerialChannel::new(&machine);

    let (progress, cmd, mut channel) = maybe_channel.expect("failed to open serial port");

    let state = AppState {
        machine_details: machine,
        movements: Default::default(),
        cached_gcode: Default::default(),
        progress: progress,
        cmd_channel: cmd,
    };

    thread::spawn(move || channel.run());

    let app = Router::new()
        .route("/run", get(get_run).post(post_run))
        .route("/analysis", get(get_analysis))
        .route("/movements", post(post_movements))
        .route("/pause", post(post_pause))
        .route("/resume", post(post_resume))
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
        .cached_gcode
        .lock()
        .unwrap()
        .insert(hash, to_gcode(&movements, Position::Relative));
    state.movements.lock().unwrap().insert(hash, movements);

    hash.to_string()
}

async fn post_pause(State(state): State<AppState>) {
    state.cmd_channel.send(PortCmd::PAUSE).await.unwrap();
}

async fn post_resume(State(state): State<AppState>) {
    state.cmd_channel.send(PortCmd::RUN).await.unwrap();
}

async fn post_run(State(state): State<AppState>, Path(handle): Path<Handle>) -> StatusCode {
    let flavor = state.machine_details.flavor;
    let program = match state.cached_gcode.lock().unwrap().get(&handle) {
        Some(gcode) => Some(to_program(gcode, flavor)),
        None => None,
    };

    match program {
        Some(program) => match state.cmd_channel.send(PortCmd::SEND(program)).await {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        None => StatusCode::NOT_FOUND,
    }
}

async fn get_run(State(state): State<AppState>) -> Json<usize> {
    axum::Json(*state.progress.borrow())
}

async fn get_analysis(State(state): State<AppState>, Path(handle): Path<Handle>) {
    let rendered: Vec<GCode> = match state.movements.lock().unwrap().get(&handle) {
        Some(movements) => todo!(),
        None => todo!(),
    };
}
