
use graphics::*;

pub struct AABB {
    pub center: Vec2d,
    pub size: Vec2d,
}

impl AABB {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> AABB {
        AABB {
            center: Vec2d([x, y]),
            size: Vec2d([w, h]),
        }
    }

    pub fn trans(&self, offset: Vec2d) -> AABB {
        let Vec2d(center) = self.center;
        let Vec2d(offset) = offset;
        AABB {
            center: Vec2d([center[0] + offset[0], center[1] + offset[1]]),
            size: self.size,
        }
    }

    pub fn is_collided_with(&self, other: &AABB) -> bool {
        self.right() >= other.left() &&
        self.left() <= other.right() &&
        self.top() <= other.bottom() &&
        self.bottom() >= other.top()
    }

    pub fn left(&self) -> f64 {
        let Vec2d(center) = self.center;
        let Vec2d(size) = self.size;
        center[0] - size[0] / 2.0
    }

    pub fn right(&self) -> f64 {
        let Vec2d(center) = self.center;
        let Vec2d(size) = self.size;
        center[0] + size[0] / 2.0
    }

    pub fn top(&self) -> f64 {
        let Vec2d(center) = self.center;
        let Vec2d(size) = self.size;
        center[1] - size[1] / 2.0
    }

    pub fn bottom(&self) -> f64 {
        let Vec2d(center) = self.center;
        let Vec2d(size) = self.size;
        center[1] + size[1] / 2.0
    }

    pub fn size(&self) -> Vec2d {
        self.size
    }
}
