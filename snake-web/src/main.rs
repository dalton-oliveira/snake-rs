use std::sync::Arc;

use once_cell::sync::Lazy;

use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::Tracer};

use salvo::otel::Tracing;

use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use salvo::websocket::WebSocketUpgrade;
use snake_web::websocket_game::WsGame;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{self};

#[derive(RustEmbed)]
#[folder = "www/"]
struct Assets;

static GAME: Lazy<WsGame> = Lazy::new(WsGame::default);
const PORT_BIND: &str = "80";

fn init_tracer() -> Tracer {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = opentelemetry_sdk::trace::TracerProvider::builder().build();
    let tracer = provider.tracer("snake-web");
    global::set_tracer_provider(provider);

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer.clone());

    tracing_subscriber::registry()
        .with(sentry_tracing::layer())
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(opentelemetry)
        .init();

    tracer
}

#[tokio::main]
async fn main() {
    let tracer = init_tracer();

    let _guard = std::env::var("SENTRY_SDN").ok().map(|sdn| {
        println!("{}", sdn);
        sentry::init((
            sdn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        ))
    });

    GAME.start_game();
    let router = Router::new()
        .hoop(affix::inject(Arc::new(tracer.clone())))
        .hoop(Tracing::new(tracer))
        .push(Router::with_path("game_data").goal(user_connected))
        .push(Router::with_path("<*path>").get(static_embed::<Assets>().fallback("index.html")));

    let port = std::env::var("PORT_BIND").unwrap_or_else(|_| PORT_BIND.to_owned());
    let bind_address = format!("0.0.0.0:{port}");

    println!("serving at http://{}", bind_address.clone());
    let acceptor = TcpListener::new(bind_address).bind().await;
    Server::new(acceptor).serve(router).await
}

#[handler]
async fn user_connected(
    req: &mut Request,
    res: &mut Response,
    _depot: &mut Depot,
) -> Result<(), StatusError> {
    WebSocketUpgrade::new()
        .upgrade(req, res, |ws| GAME.ingress_user(ws))
        .await
}
