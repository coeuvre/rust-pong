
use std::rc::Rc;
use std::rand::random;

use graphics::*;
use piston::*;

use aabb::AABB;
use settings;
use player::Player;

pub struct Ball {
    pos: [f64, ..2],
    v: [f64, ..2],

    aabb: AABB,

    image: Rc<Texture>,
}

impl Ball {
    pub fn new(image: Rc<Texture>) -> Ball {
        let (w, h) = image.get_size();
        Ball {
            pos: [0.0, 0.0],
            v: [0.0, 0.0],

            aabb: AABB::new(0.0, 0.0, w as f64, h as f64),

            image: image,
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb.trans(self.pos)
    }

    pub fn position(&self) -> [f64, ..2] {
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
        let size = self.aabb.size;
        let dp = [self.v[0] * dt, self.v[1] * dt];

        // check collision of x direction
        let aabb = self.aabb.trans([self.pos[0] + dp[0], self.pos[1]]);
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
                self.pos[0] += dp[0];
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
                self.pos[0] += dp[0];
            }
        }

        // check collision of y direction
        let aabb = self.aabb.trans([self.pos[0], self.pos[1] + dp[1]]);
        if dp[1] < 0.0 {
            if aabb.is_collided_with(top_wall_aabb) {
                self.pos[1] = top_wall_aabb.bottom() + size[1] / 2.0;
                self.v[1] = -self.v[1];
            } else {
                self.pos[1] += dp[1];
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                self.pos[1] = bottom_wall_aabb.top() - size[1] / 2.0;
                self.v[1] = -self.v[1];
            } else {
                self.pos[1] += dp[1];
            }
        }
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        let (w, h) = self.image.get_size();
        c.rect_centered(self.pos[0], self.pos[1], w as f64 / 2.0, h as f64 / 2.0)
         .image(&*self.image).draw(gl);
    }

    pub fn emit(&mut self) {
        if self.v[0] == 0.0 && self.v[1] == 0.0 {
            let dx = if random::<f64>() > 0.5 {
                0.5
            } else {
                -0.5
            };
            self.set_direction(dx, 0.0);
        }
    }

    pub fn reset(&mut self) {
        self.v = [0.0, 0.0];
        self.pos = [settings::WINDOW_SIZE[0] as f64 / 2.0, settings::WINDOW_SIZE[1] as f64 / 2.0];
    }

    fn set_direction(&mut self, dx: f64, dy: f64) {
        let len = (dx * dx + dy * dy).sqrt();
        let dx = dx / len;
        let dy = dy / len;
        self.v = [dx * settings::BALL_MOVE_SPEED, dy * settings::BALL_MOVE_SPEED];
    }

    fn reflection(&self, player: &Player) -> f64 {
        let player_pos = player.position();
        let mut y = self.pos[1] - player_pos[1];

        y = y * random::<f64>() * 0.4 + 0.8;

        let size =  player.aabb().size;
        if y > size[1] / 2.0 {
            y = size[1] / 2.0;
        } else if y < -size[1] / 2.0 {
            y = -size[1] / 2.0;
        }
        y * 4.0 / size[1]
    }
}

