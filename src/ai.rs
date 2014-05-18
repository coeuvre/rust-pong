
use ball::Ball;
use player::Player;
use settings;

pub struct AI {
    sleep_time: f64,
    awake_time: f64,
}

impl AI {
    pub fn new() -> AI {
        AI {
            sleep_time: 0.0,
            awake_time: settings::AI_AWAKE_TIME,
        }
    }

    pub fn update(&mut self, dt: f64, player: &mut Player, ball: &mut Ball) {
        let d = ball.position()[1] - player.position()[1];
        let size = ball.aabb().size;

        if self.awake_time > 0.0 {
            self.awake_time -= dt;

            if d < -size[1] / 4.0 {
                player.start_moving_up();
            } else if d > size[1] / 4.0 {
                player.start_moving_down();
            } else {
                player.stop_move();
            }
        } else {
            self.sleep_time -= dt;

            if d.abs() < size[1] / 2.0 {
                player.stop_move();
            }

            if self.sleep_time < 0.0 {
                self.awake_time = settings::AI_AWAKE_TIME;
                self.sleep_time = settings::AI_SLEEP_TIME;
            }
        }
    }
}

