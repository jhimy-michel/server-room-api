use actix_web::web::{self, Bytes};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use rand::Rng;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

#[derive(Serialize)]
struct SensorData {
    temperature: f32,
    timestamp: DateTime<Utc>,
}

struct AppState {
    is_streaming: AtomicBool,
}

#[get("/sensor-stream")]
async fn sensor_stream(data: web::Data<Arc<AppState>>) -> impl Responder {
    let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
        if !data.is_streaming.load(Ordering::Relaxed) {
            return Ok::<_, actix_web::Error>(Bytes::from("event: stop\ndata: Stream stopped\n\n"));
        }

        let mut rng = rand::thread_rng();
        let sensor_data = SensorData {
            temperature: rng.gen_range(-10.0..40.0),
            timestamp: Utc::now(),
        };
        let json = serde_json::to_string(&sensor_data).unwrap();
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
    data.is_streaming.store(true, Ordering::Relaxed);
    HttpResponse::Ok().body("Streaming started")
}

#[post("/stop")]
async fn stop_stream(data: web::Data<Arc<AppState>>) -> impl Responder {
    data.is_streaming.store(false, Ordering::Relaxed);
    HttpResponse::Ok().body("Streaming stopped")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {
        is_streaming: AtomicBool::new(false),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(sensor_stream)
            .service(start_stream)
            .service(stop_stream)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
