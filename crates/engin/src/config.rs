// config.rs

// game state
#[derive(PartialEq)]
pub enum GameState {
    Stopped,
    Running,
    Paused,
}
impl GameState {
    pub fn icon(&self) -> &'static str {
        match self {
            GameState::Running => "⏸",
            _ => "▶",
        }
    }
}

// fly mode
pub enum FlyMode {
    Wander,
    Reborn,
    Frozen,
}
// message box
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Popup {
    None,
    PopUp,
}
/// WorldConfig: Game world configuration parameters
#[derive(Clone, Copy)]
pub struct WorldConfig {
    /// Window width
    pub width: f32,
    /// Window height
    pub height: f32,
    /// Margin from the window border
    pub margin: f32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            margin: 80.0,
        }
    }
}

//icon
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconType {
    Error,
    Warning,
    Info,
    Custom(&'static str), // 可以傳入任意 emoji 或字元
}

impl IconType {
    pub fn to_emoji(&self) -> &'static str {
        match self {
            IconType::Error => "❌",
            IconType::Warning => "❗",
            IconType::Info => "🔔",
            IconType::Custom(e) => e,
        }
    }
}

// difficulty
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
