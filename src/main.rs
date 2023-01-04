use std::{env, net::SocketAddr, path::PathBuf};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router, TypedHeader,
};
use headers::UserAgent;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "echo=debug,tower_http=error".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend/build");

    let app = Router::new()
        .fallback_service(
            get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        )
        .route("/ws", get(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 9700));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(msg) => {
                    debug!("get msg, {}", msg);
                    if socket.send(Message::Text(msg)).await.is_err() {
                        info!("connect disconnected, bye");
                        return;
                    }
                }
                Message::Binary(data) => {
                    debug!("get binary, len: {}", data.len());
                    if let Ok(msg) = std::str::from_utf8(&data) {
                        debug!("msg: {msg}");
                    }
                    if socket.send(Message::Binary(data)).await.is_err() {
                        info!("connect disconnected, bye");
                        return;
                    }
                }
                Message::Ping(_) => debug!("ping"),
                Message::Pong(_) => debug!("pong"),
                Message::Close(_) => debug!("close"),
            }
        } else {
            println!("client disconnected");
            return;
        }
    }
}
