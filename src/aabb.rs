
use graphics::*;

pub struct AABB {
    pub center: [f64, ..2],
    pub size: [f64, ..2],
}

impl AABB {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> AABB {
        AABB {
            center: [x, y],
            size: [w, h],
        }
    }

    pub fn trans(&self, offset: [f64, ..2]) -> AABB {
        let &center = &self.center;
        let &offset = &offset;
        AABB {
            center: [center[0] + offset[0], center[1] + offset[1]],
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
        self.center[0] - self.size[0] / 2.0
    }

    pub fn right(&self) -> f64 {
        self.center[0] + self.size[0] / 2.0
    }

    pub fn top(&self) -> f64 {
        self.center[1] - self.size[1] / 2.0
    }

    pub fn bottom(&self) -> f64 {
        self.center[1] + self.size[1] / 2.0
    }

    pub fn size(&self) -> [f64, ..2] {
        self.size
    }
}

