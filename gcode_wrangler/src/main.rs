use axum::body::{Bytes, Full};
use axum::http::{header, Response, StatusCode};
use axum::{extract::Json, extract::Path, extract::State, routing::get, routing::post, Router};

use config::Config;
use gcode_wrangler::models::{MachineDetails, Movement};
use gcode_wrangler::{
    clamp_movements, to_gcode, to_program, GCode, PortCmd, Position, SerialChannel,
};
use image::{ImageOutputFormat, Rgba, RgbaImage};
use imageproc::drawing::{draw_line_segment_mut, Blend};
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

type Handle = u64;

const IMAGE_SCALE: f32 = 4.0;
const DRAW_COLOR: Rgba<u8> = Rgba([0u8, 0u8, 0u8, 255u8]);
const MOVE_COLOR: Rgba<u8> = Rgba([235u8, 197u8, 103u8, 255u8]);

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
        progress,
        cmd_channel: cmd,
    };

    thread::spawn(move || channel.run());

    tracing_subscriber::fmt()
        .init();

    let app = Router::new()
        .route("/run/:handle", get(get_run).post(post_run))
        .route("/rendered/:handle", get(get_analysis))
        .route("/movements", post(post_movements))
        .route("/pause", post(post_pause))
        .route("/resume", post(post_resume))
        .route("/machine", get(get_machine))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    println!("Starting Server...");
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

    let clamped_movements = clamp_movements(
        movements,
        state.machine_details.dimensions,
        Position::Absolute,
    );

    state
        .cached_gcode
        .lock()
        .unwrap()
        .insert(hash, to_gcode(&clamped_movements, Position::Absolute));

    state
        .movements
        .lock()
        .unwrap()
        .insert(hash, clamped_movements);

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
    let program = state
        .cached_gcode
        .lock()
        .unwrap()
        .get(&handle)
        .map(|gcode| to_program(gcode, flavor));

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

async fn get_analysis(
    State(state): State<AppState>,
    Path(handle): Path<Handle>,
) -> Response<Full<Bytes>> {
    let dimensions = state.machine_details.dimensions;
    match state.movements.lock().unwrap().get(&handle) {
        Some(movements) => {
            let image = RgbaImage::new(
                (dimensions.x * IMAGE_SCALE) as u32,
                (dimensions.y * IMAGE_SCALE) as u32,
            );
            let mut canvas = Blend(image);

            let mut start_position = (0.0, 0.0);

            for movement in movements.iter() {
                let dest = (
                    // using absolute coordinates always for now
                    (movement.dest.x * IMAGE_SCALE),
                    (movement.dest.y * IMAGE_SCALE),
                );

                draw_line_segment_mut(
                    &mut canvas,
                    start_position,
                    dest,
                    if movement.pen_down {
                        DRAW_COLOR
                    } else {
                        MOVE_COLOR
                    },
                );

                start_position = dest;
            }

            let buf: Vec<u8> = Vec::new();
            let mut bytes: Cursor<Vec<u8>> = Cursor::new(buf);
            canvas
                .0
                .write_to(&mut bytes, ImageOutputFormat::Png)
                .expect("Failed to save canvas bytes");

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/png")
                .body(Full::from(bytes.into_inner()))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::from(vec![]))
            .unwrap(),
    }
}
