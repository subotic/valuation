use core::error::Error;
use core::time::Duration;

use async_stream::stream;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use datastar::prelude::{MergeFragments, ReadSignals};
use datastar::Sse;
use serde::Deserialize;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/hello-world", get(hello_world))
        .route("/calculator", get(calculator_page))
        .route("/calculator/styles.css", get(calculator_style_css));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3333")
        .await
        .unwrap();

    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn index() -> Html<String> {
    Html(pages::index())
}

const MESSAGE: &str = "Hello, world!";

#[derive(Deserialize)]
pub struct Signals {
    pub delay: u64,
}

async fn hello_world(ReadSignals(signals): ReadSignals<Signals>) -> impl IntoResponse {
    Sse(stream! {
        for i in 0..MESSAGE.len() {
            yield MergeFragments::new(format!("<div id='message'>{}</div>", &MESSAGE[0..i + 1])).into();
            tokio::time::sleep(Duration::from_millis(signals.delay)).await;
        }
    })
}

async fn calculator_page() -> Html<String> {
    Html(pages::calculator::index())
}

async fn calculator_style_css() -> Response {
    (
        [("Content-Type", "text/css")],
        pages::calculator::style(), // this is the &'static str from the other crate
    )
        .into_response()
}
