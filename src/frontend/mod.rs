pub mod http;
pub mod websocket;

pub use self::websocket::Responder;

use std::sync::mpsc::{channel, Receiver};
use std::thread;
use log::LogLevel;

use config::Config;

#[derive(Debug, Deserialize)]
pub enum Request {
    UpdateConfig(Config),
    SendMessage { addr: u32, data: String },
    GetConfig,
    GetVersion,
    Shutdown,
    Restart
}

#[derive(Debug, Serialize)]
pub enum Response {
    Config(Config),
    Version(String),
    Log(u8, String),
    Ok,
    Error(String)
}

pub fn run() -> (Responder, Receiver<Request>) {
    thread::spawn(http::run);

    let (tx, rx) = channel();
    let responder = websocket::create(tx);

    (responder, rx)
}
