
use graphics::*;
use piston::*;

use aabb::AABB;
use settings;

pub struct Player {
    score: int,
    pos: [f64, ..2],
    vy: f64,
    aabb: AABB,

    image: Image,
}

impl Player {
    pub fn new(image: Image) -> Player {
        Player {
            score: 0,
            pos: [0.0, 0.0],
            vy: 0.0,
            aabb: AABB::new(0.0, 0.0, image.texture_width as f64, image.texture_height as f64),
            image: image,
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb.trans(self.pos)
    }

    pub fn position(&self) -> [f64, ..2] {
        self.pos
    }

    pub fn score(&self) -> int {
        self.score
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        self.pos = [pos[0], pos[1]];
    }

    pub fn start_moving_up(&mut self) {
        self.vy = -settings::PLAYER_MOVE_SPEED;
    }

    pub fn start_moving_down(&mut self) {
        self.vy = settings::PLAYER_MOVE_SPEED;
    }

    pub fn stop_move(&mut self) {
        self.vy = 0.0;
    }

    pub fn win(&mut self) {
        self.score += 1;
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        c.view().rect_centered(self.pos[0], self.pos[1], self.image.texture_width as f64 / 2.0, self.image.texture_height as f64 / 2.0).image(self.image).draw(gl);
    }

    pub fn update(
        &mut self,
        dt: f64,
        top_wall_aabb: &AABB,
        bottom_wall_aabb: &AABB
    ) {
        let size = self.aabb.size();
        let dy = self.vy * dt;
        let aabb = self.aabb.trans([self.pos[0], self.pos[1] + dy]);

        if dy < 0.0 {
            if aabb.is_collided_with(top_wall_aabb) {
                self.pos[1] = top_wall_aabb.bottom() + size[1] / 2.0;
            } else {
                self.pos[1] += dy;
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                self.pos[1] = bottom_wall_aabb.top() - size[1] / 2.0;
            } else {
                self.pos[1] += dy;
            }
        }
    }
}

