use axum::response::IntoResponse;
use axum::{http::StatusCode, routing::post, Json, Router};
use irq::querry::line_halfplane;
use irq::Line;
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
        .route("/irq", post(handle))
        .nest_service("/", ServeDir::new("./dist"))
        .layer(cors);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(ADR).await.unwrap();
    println!("Started server: http://{ADR}/");
    println!("crtl + c to close the server");

    let serve = axum::serve(listener, app);

    open::that(format!("http://{ADR}/")).unwrap();
    serve.await.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SLine {
    m: f64,
    b: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    lines: Vec<SLine>,
    halfplane: SLine,
    bounds_above: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    result: Vec<Vec<f64>>,
}

async fn handle(Json(data): Json<Data>) -> impl IntoResponse {
    // println!("{:?}", data);

    let mut d: Vec<Line> = Vec::new();
    for (i, line) in data.lines.iter().enumerate() {
        d.push(Line::new(line.m, line.b, i));
    }
    let boundary = Line::new(data.halfplane.m, data.halfplane.b, 0);
    let hp = irq::HalfPlane::new(&boundary, data.bounds_above);
    let res = line_halfplane(&mut d, &hp);

    let mut out: Vec<Vec<f64>> = Vec::new();

    for (i, w) in res.iter().enumerate() {
        let l_i = &d[i];
        for j in w {
            let l_j = &d[*j];
            let x = l_i.intersection_with_line(l_j).unwrap();
            let y = l_i.y_at(x);
            out.push(vec![x, y]);
        }
    }

    let out = Result { result: out };

    (StatusCode::OK, Json(out))
}
