//! A small axum server, that is able to serve an index.html, that loads a wasm
//! module It explictly looks for an index.html in the current directory
//! If no port is given, it will panic - otherwise it will serve on 127.0.0.1:<port>

use axum::{
    Router, body::Body, http::{Request, StatusCode}, middleware::{self, Next}, response::{Html, IntoResponse}, routing::get
};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = env::args().nth(1).expect("No Port given");
    let port = args.parse::<u16>().expect("Cannot parse given port");
    
    // Define where to find the certificate and private key used by https
    let config: RustlsConfig = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem"
    )
    .await
    .unwrap();

    // Serve everything from the current directory (From the working directory you invoke the `server` binary from)
    let app = Router::new()
        .fallback_service(ServeDir::new("."))
        .route("/", get(index_handler))
        .layer(middleware::from_fn(add_wasm_headers));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on https://{}", addr);
    
    axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
}

async fn add_wasm_headers(
    request: Request<Body>, next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Add COOP and COEP headers required for WASM with SharedArrayBuffer
    headers.insert("cross-origin-opener-policy", "same-origin".parse().unwrap());
    headers.insert(
        "cross-origin-embedder-policy",
        "require-corp".parse().unwrap(),
    );
    headers.insert(
        "cross-origin-resource-policy",
        "cross-origin".parse().unwrap(),
    );

    Ok(response)
}

async fn index_handler() -> (StatusCode, Html<String>) {
    match std::fs::read_to_string("./index.html") {
        Ok(content) => (StatusCode::OK, Html(content)),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Html("index.html not found".to_string()),
        ),
    }
}
