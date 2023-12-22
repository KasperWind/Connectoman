#![feature(let_chains)]
mod routes;

use std::sync::{Mutex, Arc};
use std::{io::Result, net::SocketAddr};

use axum::handler::HandlerWithoutStateExt;
use axum::http::{Method, Uri, StatusCode};
use axum::routing::{MethodRouter, any_service};
use axum::{Router, response::Response, middleware};

use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, Level};

use tera::Tera;

use crate::routes::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::DEBUG)
        .init();

    let t = Tera::new("templates/**/*.html").unwrap();
    let templates:Vec<&_> = t.get_template_names().collect();

    info!("{:<12} loaded templates {}", "TEMPLATE", templates.len());
    for template in templates {
        info!("{:<20} {template}", "TEMPLATE");
    }

    let tera = Arc::new(Mutex::new(t));

    let app = Router::new()
        .merge(routes::routes(AppState{tera: tera.clone()}))
        .layer(middleware::map_response(mw_reponse_map))
        .fallback_service(serve_dir());

    let addr = SocketAddr::from(([0, 0, 0, 0], 6969));
    info!("{:<12} {addr}", "LISTENING");
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Resource not found")
    }
    any_service(ServeDir::new("web_folder").not_found_service(handle_404.into_service()))
}

pub async fn mw_reponse_map(
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response { 
    info!("{:<12} {req_method} {uri}", "RES_MAPPER");

    res
}
