use axum::response::IntoResponse;
use axum::{http::StatusCode, routing::post, Json, Router};
use irq::{compute_eip, Line};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

const ADR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    // allow requests from any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application
    let app = Router::new()
        .route("/eip", post(handle))
        .nest_service("/", ServeDir::new("./dist"))
        .layer(cors);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(ADR).await.unwrap();
    println!("Started server at {ADR}");
    println!("Visit http://{ADR}/index.html");

    let serve =axum::serve(listener, app);

    open::that(format!("http://{ADR}/index.html")).unwrap();
    serve.await.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SLine {
    m: f64,
    b: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    lines: Vec<SLine>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    result: Vec<Vec<f64>>
}

async fn handle(Json(data): Json<Data>) -> impl IntoResponse {
    println!("{:?}", data);

    let mut d: Vec<Line> = Vec::new();
    for (i, line) in data.lines.iter().enumerate() {
        d.push(Line::new(line.m, line.b, i));
    }
    let eip = compute_eip(&mut d);

    let mut out: Vec<Vec<f64>> = Vec::new();

    for i in 0..d.len() {
        let idx = d[i].idx;

        let x = eip.0[idx];
        let mut a: Vec<f64> = vec![x];
        a.push(d[i].y_at(x));
        out.push(a);

        let x = eip.1[idx];
        let mut b: Vec<f64> = vec![x];
        b.push(d[i].y_at(x));
        out.push(b);
    }

    let out = Result {result: out};

    (StatusCode::OK, Json(out))
}
