use std::sync::Arc;

use once_cell::sync::Lazy;

use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::Tracer};

use rust_embed::RustEmbed;
use salvo::otel::Tracing;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use salvo::websocket::WebSocketUpgrade;
use shuttle_secrets::SecretStore;
use snake_web::websocket_game::WsGame;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{self};

#[derive(RustEmbed)]
#[folder = "www/"]
struct Assets;

static GAME: Lazy<WsGame> = Lazy::new(WsGame::default);

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

// #[shuttle_runtime::main]
// async fn main() -> shuttle_salvo::ShuttleSalvo {
//     // let tracer = init_tracer();

//     // let _guard = std::env::var("SENTRY_SDN").ok().map(|sdn| {
//     //     println!("{}", sdn);
//     //     sentry::init((
//     //         sdn,
//     //         sentry::ClientOptions {
//     //             release: sentry::release_name!(),
//     //             traces_sample_rate: 1.0,
//     //             ..Default::default()
//     //         },
//     //     ))
//     // });

//     GAME.start_game();
//     let router = Router::new()
//         // .hoop(affix::inject(Arc::new(tracer.clone())))
//         // .hoop(Tracing::new(tracer))
//         .push(Router::with_path("game_data").goal(user_connected))
//         .push(Router::with_path("<*path>").get(static_embed::<Assets>().fallback("index.html")));

//     Ok(router.into())
// }

// #[tokio::main]

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_salvo::ShuttleSalvo {
    let tracer = init_tracer();

    let _guard = secret_store.get("SENTRY_SDN").map(|sdn| {
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

    Ok(router.into())
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
