use bevy::{
    ecs::event::{Event, EventReader},
    prelude::*,
};
use tracing::error;

#[derive(Event)]
pub struct ErrorEvent(pub GameError);

pub fn log_error(mut errors: EventReader<ErrorEvent>) {
    for event in errors.read() {
        let clean_error = strip_ansi(&event.0.to_string());
        error!("{}", clean_error);
    }
}

pub trait ResultExt<T> {
    fn log_err(self) -> Option<T>;
    fn log_err_with(self, context: &str) -> Option<T>;
}

impl<T, E: Into<GameError>> ResultExt<T> for Result<T, E> {
    fn log_err(self) -> Option<T> {
        self.map_err(|e| {
            let game_error = e.into();
            error!("{}", strip_ansi(&game_error.to_string()));
            game_error
        })
        .ok()
    }

    fn log_err_with(self, context: &str) -> Option<T> {
        self.map_err(|e| {
            let game_error = e.into();
            error!("{}: {}", context, strip_ansi(&game_error.to_string()));
            game_error
        })
        .ok()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Parser(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(InvalidData),
    #[error("Unsupported: {0}")]
    Unsupported(String),
    #[error("Client error: {0}")]
    Client(#[from] ClientError),
    #[error("Server error: {0}")]
    Server(#[from] ServerError),
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidData {
    #[error("Invalid block identifier: {0}")]
    BlockIdentifier(String),
    #[error("Invalid block data: {0}")]
    BlockData(String),
    #[error("Invalid block position: {0}")]
    BlockPosition(String),
    #[error("Invalid block state: {0}")]
    BlockState(String),
    #[error("Invalid block metadata: {0}")]
    BlockMetadata(String),
    #[error("Invalid block properties: {0}")]
    BlockProperties(String),
    #[error("Invalid block texture: {0}")]
    BlockTexture(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Rendering error: {0}")]
    RenderingError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Asset error: {0}")]
    AssetError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

fn strip_ansi(input: &str) -> String {
    input
        .replace('\x1b', " ")
        .replace("[0m", "\"")
        .replace("[2m", "\"")
        .replace("[3m", "\"")
}
