
use graphics::*;
use piston::*;

use aabb::AABB;
use settings;
use player::Player;

pub struct App {
    background_image: Option<Image>,
    ball_image: Option<Image>,
    player_image: Option<Image>,

    player1: Option<Player>,

    top_wall_aabb: AABB,
    bottom_wall_aabb: AABB,

    is_up_holding: bool,
    is_down_holding: bool,
}

impl App {
    pub fn new() -> App {
        App {
            background_image: None,
            ball_image: None,
            player_image: None,

            player1: None,

            top_wall_aabb: AABB::new(240.0, 8.0, 480.0, 16.0),
            bottom_wall_aabb: AABB::new(240.0, 320.0 - 8.0, 480.0, 16.0),

            is_up_holding: false,
            is_down_holding: false,
        }
    }
}

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        c.view().image(self.background_image.unwrap()).draw(gl);

        self.player1.get_ref().render(c, gl);
    }

    fn update(&mut self, dt: f64, asset_store: &mut AssetStore) {
        if self.is_up_holding && self.is_down_holding {
            self.player1.get_mut_ref().stop_move();
        } else if self.is_up_holding {
            self.player1.get_mut_ref().start_moving_up();
        } else if self.is_down_holding {
            self.player1.get_mut_ref().start_moving_down();
        } else {
            self.player1.get_mut_ref().stop_move();
        }

        self.player1.get_mut_ref().update(dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
    }

    fn load(&mut self, asset_store: &mut AssetStore) {
        self.background_image = Some(asset_store.load_image(settings::BACKGROUND_IMAGE));
        self.ball_image = Some(asset_store.load_image(settings::BALL_IMAGE));
        self.player_image = Some(asset_store.load_image(settings::PLAYER_IMAGE));

        self.player1 = Some(Player::new(self.player_image.unwrap()));
        self.player1.get_mut_ref().offset(Vec2d([settings::PLAYER_PADDING, settings::WINDOW_SIZE[1] as f64 / 2.0]));
    }

    fn key_press(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        if key == keyboard::Up {
            self.is_up_holding = true;
        }

        if key == keyboard::Down {
            self.is_down_holding = true;
        }
    }

    fn key_release(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        if key == keyboard::Up {
            self.is_up_holding = false;
        }

        if key == keyboard::Down {
            self.is_down_holding = false;
        }
    }

}

