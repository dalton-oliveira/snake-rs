use snake_multiplayer::websocket_game::WsGame;

use once_cell::sync::Lazy;

use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use salvo::websocket::WebSocketUpgrade;

#[derive(RustEmbed)]
#[folder = "../wasm-render/www/"]
struct Assets;

static GAME: Lazy<WsGame> = Lazy::new(|| WsGame::new());
const BIND_ADDRESS: &str = "0.0.0.0:4000";
#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("game_data").goal(user_connected))
        .push(Router::with_path("<*path>").get(static_embed::<Assets>().fallback("index.html")));

    let acceptor = TcpListener::new(BIND_ADDRESS).bind().await;
    println!("serving at http://{BIND_ADDRESS}");
    GAME.start_game();

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    WebSocketUpgrade::new()
        .upgrade(req, res, |ws| GAME.add_user(ws))
        .await
}
