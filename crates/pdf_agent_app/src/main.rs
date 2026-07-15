mod app;
mod message;
mod screens;
mod state;
mod theme;
mod commands;
mod components;
mod panes;
mod subscriptions;
mod update_handler;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    App::run(Settings::default())
}
