use std::sync::Arc;

use axum::{routing::post, Router};
use domain_for_sale::{config, handler, AppState};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env().unwrap();
    let web_addr = cfg.web.address.clone();

    let state = Arc::new(AppState { cfg });

    let app = Router::new()
        .route("/api/offer", post(handler::offer))
        .fallback(handler::static_handler)
        .with_state(state);

    let tcp_listener = TcpListener::bind(&web_addr).await.unwrap();

    tracing::info!("服务监听于：{}", &web_addr);

    axum::serve(tcp_listener, app).await.unwrap();
}
