
#![feature(globs)]

extern crate graphics;
extern crate piston;

use piston::*;

mod aabb;
mod ai;
mod ball;
mod player;
mod pong_app;
mod settings;

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings {
            title: "Rust-Pong".to_string(),
            size: settings::WINDOW_SIZE,
            fullscreen: false,
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0]
        }
    );

    let mut asset_store = AssetStore::from_folder(settings::ASSET_FOLDER);

    let mut app = pong_app::App::new();

    app.run(&mut game_window, &mut asset_store);
}

