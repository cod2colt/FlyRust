// model
use crate::config::{FlyMode, WorldConfig};
use rand::{Rng, rng};

/// FlyModel: Encapsulates fly position logic
pub struct FlyModel {
    pub config: WorldConfig, // config setting
    pub fly_speed: f32,      // fly speed factor
    pub fly_hunted: u32,     // fly hunting number
    pub fly_time: f32,       // game total time in second
    pub counter: f32,        // play time counter
}

impl FlyModel {
    /// Initialize FlyModel in the middle of window
    pub fn new(config: WorldConfig) -> Self {
        // Return the struct directly
        Self {
            config, // This uses "Field Init Shorthand" since name and variable match
            fly_speed: 20.0,
            fly_hunted: 0,
            fly_time: 10.0,
            counter: 10.0, // init set to fly_time
        }
    }

    // reset the game
    pub fn fly_reset(&mut self) {
        self.counter = self.fly_time;
    }

    // start the game
    pub fn fly_start(&mut self) {
        self.counter = self.fly_time;
        self.fly_hunted = 0;
    }

    // update counter
    // return false: counter less or equal 0
    pub fn fly_counter_update(&mut self, dt: f32) -> bool {
        self.counter -= dt;
        self.counter > 0.0 // return bool
    }

    // update fly hunted number
    pub fn fly_hunted_update(&mut self) {
        self.fly_hunted += 1;
    }

    // generate new fly position
    pub fn fly_position(&mut self, fly_x: f32, fly_y: f32, mode: FlyMode) -> (f32, f32) {
        let win_x = self.config.width;
        let win_y = self.config.height;
        let win_margin = self.config.margin;
        let speed_factor = self.fly_speed;

        let mut rand_num = rng();
        let x_rand = rand_num.random_range(-1.0..1.0);
        let y_rand = rand_num.random_range(-1.0..1.0);
        let x_margin = win_x - win_margin * 2.0;
        let y_margin = win_y - win_margin * 2.0;
        let mut x_new = fly_x - win_margin;
        let mut y_new = fly_y - win_margin;
        // the fly hunted, generate a new position
        match mode {
            FlyMode::Wander => {
                x_new = (x_new + x_rand * speed_factor).clamp(0.0, x_margin);
                y_new = (y_new + y_rand * speed_factor).clamp(0.0, y_margin);
            }
            FlyMode::Reborn => {
                x_new = x_rand.abs() * x_margin;
                y_new = y_rand.abs() * y_margin;
            }
            _ => {
                // freeze
                x_new += 0.0;
                y_new += 0.0;
            }
        }
        x_new += win_margin;
        y_new += win_margin;

        //return
        (x_new, y_new)
    }
}
