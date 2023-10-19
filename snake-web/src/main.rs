use once_cell::sync::Lazy;
use salvo::conn::openssl::{Keycert, OpensslConfig};

use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use salvo::websocket::WebSocketUpgrade;
use snake_web::websocket_game::WsGame;
use tracing::Level;
use tracing_subscriber;

#[derive(RustEmbed)]
#[folder = "www/"]
struct Assets;

static GAME: Lazy<WsGame> = Lazy::new(|| WsGame::new());
const PORT_BIND: &str = "80";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    GAME.start_game();
    let router = Router::new()
        .push(Router::with_path("game_data").goal(user_connected))
        .push(Router::with_path("<*path>").get(static_embed::<Assets>().fallback("index.html")));

    let port = std::env::var("PORT_BIND").unwrap_or_else(|_| PORT_BIND.to_owned());
    let bind_address = format!("0.0.0.0:{port}");
    if !port.ends_with("80") {
        let config = OpensslConfig::new(
            Keycert::new()
                .with_cert(include_bytes!("../../certs/cert.pem").as_ref())
                .with_key(include_bytes!("../../certs/privKey.pem").as_ref()),
        );
        println!("serving at https://{}", bind_address.clone());
        let acceptor = TcpListener::new(bind_address).openssl(config).bind().await;
        Server::new(acceptor).serve(router).await;
    } else {
        println!("serving at http://{}", bind_address.clone());
        let acceptor = TcpListener::new(bind_address).bind().await;
        Server::new(acceptor).serve(router).await
    }
}

#[handler]
async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    WebSocketUpgrade::new()
        .upgrade(req, res, |ws| GAME.ingress_user(ws))
        .await
}
