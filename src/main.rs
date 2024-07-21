use actix_cors::Cors;
use actix_web::web::{self, Bytes};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use log::info;
use rand::Rng;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

#[derive(Serialize)]
struct RackTemperature {
    id: String,
    temperature: f32,
    status: String,
}

#[derive(Serialize)]
struct ServerRoomData {
    timestamp: DateTime<Utc>,
    racks: Vec<RackTemperature>,
}

struct AppState {
    is_streaming: AtomicBool,
}

fn get_temperature_status(temp: f32) -> String {
    if temp < 15.0 {
        "Too Cold".to_string()
    } else if temp > 30.0 {
        "Too Hot".to_string()
    } else if temp >= 18.0 && temp <= 27.0 {
        "Optimal".to_string()
    } else {
        "Acceptable".to_string()
    }
}

fn generate_rack_data() -> Vec<RackTemperature> {
    let mut rng = rand::thread_rng();
    let rack_ids = vec![
        "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "S1", "S2", "S3", "S4", "S5",
        "S6", "S7", "S8", "S9", "S10",
    ];

    rack_ids
        .into_iter()
        .map(|id| {
            let temperature = rng.gen_range(15.0..35.0);
            RackTemperature {
                id: id.to_string(),
                temperature,
                status: get_temperature_status(temperature),
            }
        })
        .collect()
}

#[get("/server-room-stream")]
async fn server_room_stream(data: web::Data<Arc<AppState>>) -> impl Responder {
    let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
        if !data.is_streaming.load(Ordering::Relaxed) {
            return Ok::<_, actix_web::Error>(Bytes::from("event: stop\ndata: Stream stopped\n\n"));
        }

        let server_room_data = ServerRoomData {
            timestamp: Utc::now(),
            racks: generate_rack_data(),
        };
        
        let json: String = serde_json::to_string(&server_room_data).unwrap();
        info!("Data generated: {}", json);
        Ok::<_, actix_web::Error>(Bytes::from(format!("data: {}\n\n", json)))
    });

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/event-stream"))
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Connection", "keep-alive"))
        .streaming(stream)
}

#[post("/start")]
async fn start_stream(data: web::Data<Arc<AppState>>) -> impl Responder {
    info!("Stream started by client request.");
    data.is_streaming.store(true, Ordering::Relaxed);
    let response = serde_json::json!({
        "status": "success",
        "message": "Streaming started"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

#[post("/stop")]
async fn stop_stream(data: web::Data<Arc<AppState>>) -> impl Responder {
    info!("Stream stopped by client request.");
    data.is_streaming.store(false, Ordering::Relaxed);

    let response = serde_json::json!({
        "status": "success",
        "message": "Streaming stopped"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {
        is_streaming: AtomicBool::new(false),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .service(server_room_stream)
            .service(start_stream)
            .service(stop_stream)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
