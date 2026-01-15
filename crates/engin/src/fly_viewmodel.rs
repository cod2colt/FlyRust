// view model
use crate::config::{Difficulty, FlyMode, GameState, WorldConfig};
use crate::fly_model::FlyModel;
use util::utilities;

pub struct FlyViewModel {
    pub timer_tick: f32,
    pub state: GameState, // fly state
    pub fly: FlyModel,
    pub fly_x: f32, // fly at x
    pub fly_y: f32, // fly at y
    pub difficulty: Difficulty,
}

impl FlyViewModel {
    pub fn new(config: WorldConfig) -> Self {
        let fly = FlyModel::new(config);
        let x = config.width / 2.0;
        let y = config.height / 2.0;

        Self {
            timer_tick: 0.1, // timer tick 0.1
            state: GameState::Stopped,
            fly,
            fly_x: x,
            fly_y: y,
            difficulty: Difficulty::Easy,
        }
    }

    // update the running status
    pub fn toggle_run(&mut self) {
        self.state = match self.state {
            GameState::Stopped => {
                self.fly.fly_start();
                GameState::Running
            }
            GameState::Running => GameState::Paused,
            GameState::Paused => GameState::Running,
        };
    }

    // stop button procedure
    // return to close the app or not
    pub fn stop(&mut self) -> bool {
        if self.state == GameState::Stopped {
            // game stop to set close app
            true
        } else {
            // stop flying
            self.state = GameState::Stopped;
            // reset the game
            self.fly.fly_reset();
            // get new position
            (self.fly_x, self.fly_y) =
                self.fly
                    .fly_position(self.fly_x, self.fly_y, FlyMode::Reborn);
            false
        }
    }

    // timer tick
    // return to pop message box or not or not
    pub fn tick(&mut self) -> bool {
        // check running
        if !self.is_running() {
            return false;
        }
        // update counter
        if !self.fly.fly_counter_update(self.timer_tick) {
            self.stop();
            // pop message box
            return true;
        }
        // get the fly fly trace
        (self.fly_x, self.fly_y) = self
            .fly
            .fly_position(self.fly_x, self.fly_y, FlyMode::Wander);
        false
    }

    // fly clicked, fly hunted
    pub fn fly_click(&mut self) {
        if !self.is_running() {
            return;
        }
        // update fly hunted number
        self.fly.fly_hunted_update();

        // get a new position
        (self.fly_x, self.fly_y) = self
            .fly
            .fly_position(self.fly_x, self.fly_y, FlyMode::Reborn);
    }

    pub fn apply_difficulty(&mut self) {
        match self.difficulty {
            Difficulty::Easy => self.fly.fly_speed = 10.0,
            Difficulty::Medium => self.fly.fly_speed = 30.0,
            Difficulty::Hard => self.fly.fly_speed = 60.0,
        }
    }

    // set game result message
    pub fn get_game_result_message(&self) -> u32 {
        self.fly.fly_hunted
    }
    // private functions
    // check game state
    fn is_running(&self) -> bool {
        matches!(self.state, GameState::Running)
    }

    pub fn dash_board_info(&self) -> (String, String) {
        //     // counter display
        let str_sec = utilities::time_format_to_s_m(self.fly.counter);
        let str_score: String = format!("{}", self.fly.fly_hunted);
        (str_sec, str_score)
    }
}
