use log::{self, Log, LogRecord, LogLevel, LogLevelFilter, LogMetadata};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use frontend:: {Responder, Response};

struct Logger {
    responder: Responder
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                LogLevel::Error => "\x1B[31m",
                LogLevel::Warn => "\x1B[33m",
                LogLevel::Info => "\x1B[32m",
                _ => ""
            };
            println!("\r{}{}\x1B[39m - {}", color, record.level(), record.args())
        }

        self.responder.send(Response::Log(record.level() as u8, record.args().to_string()));
    }
}

pub fn init(responder: Responder) -> Receiver<(u8, String)> {
    let (tx, rx) = channel();

    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(Logger { responder: responder })
    }).expect("Unable to setup logger");

    rx
}
