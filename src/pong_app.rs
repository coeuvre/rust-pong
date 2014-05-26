
use graphics::*;
use piston::*;

use aabb::AABB;
use ai::AI;
use ball::Ball;
use settings;
use player::Player;

pub struct App {
    background_image: Option<Image>,
    ball_image: Option<Image>,
    player_image: Option<Image>,

    player1: Option<Player>,
    player2: Option<Player>,
    ball: Option<Ball>,
    ai: Option<AI>,

    top_wall_aabb: AABB,
    bottom_wall_aabb: AABB,
    left_wall_aabb: AABB,
    right_wall_aabb: AABB,

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
            player2: None,
            ball: None,
            ai: None,

            top_wall_aabb: AABB::new(
                settings::WINDOW_SIZE[0] as f64 / 2.0,
                settings::WALL_RADIUS,
                settings::WINDOW_SIZE[0] as f64,
                settings::WALL_RADIUS * 2.0
            ),
            bottom_wall_aabb: AABB::new(
                settings::WINDOW_SIZE[0] as f64 / 2.0,
                settings::WINDOW_SIZE[1] as f64 - settings::WALL_RADIUS,
                settings::WINDOW_SIZE[0] as f64,
                settings::WALL_RADIUS * 2.0
            ),
            left_wall_aabb: AABB::new(
                -settings::WALL_RADIUS,
                settings::WINDOW_SIZE[1] as f64 / 2.0,
                settings::WALL_RADIUS * 2.0,
                settings::WINDOW_SIZE[1] as f64
            ),
            right_wall_aabb: AABB::new(
                settings::WINDOW_SIZE[0] as f64 + settings::WALL_RADIUS,
                settings::WINDOW_SIZE[1] as f64 / 2.0,
                settings::WALL_RADIUS * 2.0,
                settings::WINDOW_SIZE[1] as f64
            ),

            is_up_holding: false,
            is_down_holding: false,
        }
    }
}

impl Game for App {
    fn render(&self, _ext_dt: f64, c: &Context, gl: &mut Gl) {
        c.view().image(self.background_image.unwrap()).draw(gl);

        self.player1.get_ref().render(c, gl);
        self.player2.get_ref().render(c, gl);
        self.ball.get_ref().render(c, gl);
    }

    fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
        if self.is_up_holding && self.is_down_holding {
            self.player1.get_mut_ref().stop_move();
        } else if self.is_up_holding {
            self.player1.get_mut_ref().start_moving_up();
        } else if self.is_down_holding {
            self.player1.get_mut_ref().start_moving_down();
        } else {
            self.player1.get_mut_ref().stop_move();
        }

        self.ai.get_mut_ref().update(dt, self.player2.get_mut_ref(), self.ball.get_mut_ref());

        self.player1.get_mut_ref().update(dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
        self.player2.get_mut_ref().update(dt, &self.top_wall_aabb, &self.bottom_wall_aabb);
        self.ball.get_mut_ref().update(dt, &self.top_wall_aabb, &self.bottom_wall_aabb, &self.left_wall_aabb, &self.right_wall_aabb, self.player1.get_mut_ref(), self.player2.get_mut_ref());
    }

    fn load(&mut self, asset_store: &mut AssetStore) {
        self.background_image = Some(asset_store.load_image(settings::BACKGROUND_IMAGE).unwrap());
        self.ball_image = Some(asset_store.load_image(settings::BALL_IMAGE).unwrap());
        self.player_image = Some(asset_store.load_image(settings::PLAYER_IMAGE).unwrap());

        self.player1 = Some(Player::new(self.player_image.unwrap()));
        self.player1.get_mut_ref().set_pos([settings::PLAYER_PADDING, settings::WINDOW_SIZE[1] as f64 / 2.0]);

        self.player2 = Some(Player::new(self.player_image.unwrap()));
        self.player2.get_mut_ref().set_pos([settings::WINDOW_SIZE[0] as f64 - settings::PLAYER_PADDING, settings::WINDOW_SIZE[1] as f64 / 2.0]);

        self.ball = Some(Ball::new(self.ball_image.unwrap()));
        self.ball.get_mut_ref().reset();

        self.ai = Some(AI::new());
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

        if key == keyboard::Space {
            self.ball.get_mut_ref().emit();
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

