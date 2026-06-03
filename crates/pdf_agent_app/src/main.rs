mod app;
mod message;
mod screens;
mod state;
mod theme;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    App::run(Settings::default())
}
