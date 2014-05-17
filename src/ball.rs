
use rand;

use graphics::*;
use piston::*;

use aabb::AABB;
use settings;
use player::Player;

pub struct Ball {
    pos: Vec2d,
    v: Vec2d,

    aabb: AABB,

    image: Image,
}

impl Ball {
    pub fn new(image: Image) -> Ball {
        Ball {
            pos: Vec2d([0.0, 0.0]),
            v: Vec2d([0.0, 0.0]),

            aabb: AABB::new(0.0, 0.0, image.texture_width as f64, image.texture_height as f64),

            image: image,
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb.trans(self.pos)
    }

    pub fn position(&self) -> Vec2d {
        self.pos
    }

    pub fn update(
        &mut self, dt: f64,
        top_wall_aabb: &AABB,
        bottom_wall_aabb: &AABB,
        left_wall_aabb: &AABB,
        right_wall_aabb: &AABB,
        player1: &mut Player,
        player2: &mut Player
    ) {
        let Vec2d(size) = self.aabb.size;
        let Vec2d(mut pos) = self.pos;
        let Vec2d(v) = self.v;
        let dp = [v[0] * dt, v[1] * dt];

        // check collision of x direction
        let aabb = self.aabb.trans(Vec2d([pos[0] + dp[0], pos[1]]));
        if dp[0] < 0.0 {
            if aabb.is_collided_with(left_wall_aabb) {
                player2.win();
                self.reset();
                println!("{} : {}", player1.score(), player2.score());
                return;
            } else if aabb.is_collided_with(&player1.aabb()) {
                let dy = self.reflection(player1);
                self.set_direction(1.0, dy);
            } else {
                pos[0] = pos[0] + dp[0];
            }
        } else {
            if aabb.is_collided_with(right_wall_aabb) {
                player1.win();
                self.reset();
                println!("{} : {}", player1.score(), player2.score());
                return;
            } else if aabb.is_collided_with(&player2.aabb()) {
                let dy = self.reflection(player2);
                self.set_direction(-1.0, dy);
            } else {
                pos[0] = pos[0] + dp[0];
            }
        }

        // check collision of y direction
        let aabb = self.aabb.trans(Vec2d([pos[0], pos[1] + dp[1]]));
        if dp[1] < 0.0 {
            if aabb.is_collided_with(top_wall_aabb) {
                pos[1] = top_wall_aabb.bottom() + size[1] / 2.0;
                self.v = Vec2d([v[0], -v[1]]);
            } else {
                pos[1] = pos[1] + dp[1];
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                pos[1] = bottom_wall_aabb.top() - size[1] / 2.0;
                self.v = Vec2d([v[0], -v[1]]);
            } else {
                pos[1] = pos[1] + dp[1];
            }
        }

        self.pos = Vec2d(pos);
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        let Vec2d(pos) = self.pos;
        c.view().trans_local(pos[0], pos[1]).rect_centered(0.0, 0.0, self.image.texture_width as f64 / 2.0, self.image.texture_height as f64 / 2.0).image(self.image).draw(gl);
    }

    pub fn emit(&mut self) {
        let Vec2d(v) = self.v;
        if v[0] == 0.0 && v[1] == 0.0 {
            let dx = if rand::random::<f64>() > 0.5 {
                0.5
            } else {
                -0.5
            };
            self.set_direction(dx, rand::random::<f64>() * 2.0 - 1.0);
        }
    }

    pub fn reset(&mut self) {
        self.v = Vec2d([0.0, 0.0]);
        self.pos = Vec2d([settings::WINDOW_SIZE[0] as f64 / 2.0, settings::WINDOW_SIZE[1] as f64 / 2.0]);
    }

    fn set_direction(&mut self, dx: f64, dy: f64) {
        let len = (dx * dx + dy * dy).sqrt();
        let dx = dx / len;
        let dy = dy / len;
        self.v = Vec2d([dx * settings::BALL_MOVE_SPEED, dy * settings::BALL_MOVE_SPEED]);
    }

    fn reflection(&self, player: &Player) -> f64 {
        let Vec2d(pos) = self.pos;
        let Vec2d(player_pos) = player.position();
        let mut y = pos[1] - player_pos[1];

        y = y * rand::random::<f64>() * 0.4 + 0.8;

        let Vec2d(size) =  player.aabb().size;
        if y > size[1] / 2.0 {
            y = size[1] / 2.0;
        } else if y < -size[1] / 2.0 {
            y = -size[1] / 2.0;
        }
        y * 4.0 / size[1]
    }
}

