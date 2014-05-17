
#![feature(globs)]

extern crate rand;

extern crate graphics;
extern crate piston;

use piston::*;

mod aabb;
mod ball;
mod player;
mod pong_app;
mod settings;

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings::new (
            "Rust-Pong".to_owned(),
            settings::WINDOW_SIZE,
            false,
            true,
            [0.0, 0.0, 0.0, 1.0]
        )
    );

    let mut asset_store = AssetStore::from_folder(settings::ASSET_FOLDER);

    let mut app = pong_app::App::new();

    app.run(&mut game_window, &mut asset_store);
}

