

use graphics::*;
use piston::*;

use aabb::AABB;
use settings;

pub struct Player {
    score: int,
    pos: Vec2d,
    vy: f64,
    aabb: AABB,

    image: Image,
}

impl Player {
    pub fn new(image: Image) -> Player {
        Player {
            score: 0,
            pos: Vec2d([0.0, 0.0]),
            vy: 0.0,
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

    pub fn score(&self) -> int {
        self.score
    }

    pub fn offset(&mut self, offset: Vec2d) {
        let Vec2d(pos) = self.pos;
        let Vec2d(offset) = offset;
        self.pos = Vec2d([pos[0] + offset[0], pos[1] + offset[1]]);
    }

    pub fn is_moving_up(&self) -> bool {
        self.vy < 0.0
    }

    pub fn start_moving_up(&mut self) {
        self.vy = -settings::PLAYER_MOVE_SPEED;
    }

    pub fn is_moving_down(&self) -> bool {
        self.vy > 0.0
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
        let Vec2d(pos) = self.pos;
        c.view().trans_local(pos[0], pos[1]).rect_centered(0.0, 0.0, self.image.texture_width as f64 / 2.0, self.image.texture_height as f64 / 2.0).image(self.image).draw(gl);
    }

    pub fn update(
        &mut self,
        dt: f64,
        top_wall_aabb: &AABB,
        bottom_wall_aabb: &AABB
    ) {
        let Vec2d(mut pos) = self.pos;
        let Vec2d(size) = self.aabb.size();
        let dy = self.vy * dt;
        let aabb = self.aabb.trans(Vec2d([pos[0], pos[1] + dy]));

        if dy < 0.0 {
            if aabb.is_collided_with(top_wall_aabb) {
                pos[1] = top_wall_aabb.bottom() + size[1] / 2.0;
            } else {
                pos[1] = pos[1] + dy;
            }
        } else {
            if aabb.is_collided_with(bottom_wall_aabb) {
                pos[1] = bottom_wall_aabb.top() - size[1] / 2.0;
            } else {
                pos[1] = pos[1] + dy;
            }
        }

        self.pos = Vec2d(pos);
    }
}
