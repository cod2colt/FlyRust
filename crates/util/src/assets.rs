//! assets.rs
//! Handle assets json file
//! read json file and parser to its data structure.
use eframe::egui;
use egui::{ColorImage, Context, TextureHandle, TextureOptions};
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

// assets.json path
pub const I18NUIJSON: &str = "assets/i18n/ui.json";
pub const I18NUIZHTWJSON: &str = "assets/i18n/ui_zh-TW.json";
pub const I18NUIZHCNJSON: &str = "assets/i18n/ui_zh-CN.json";

pub static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets");
pub static IMAGE_LIST: &[(&str, &str)] = &[
    // id, path
    ("Rustacean", "image/rustacean-flat-happy-48x48.png"),
    ("Hand", "image/hand.png"),
    ("Beach", "image/wave-sand-beach-800x600.png"),
];

// Image information
#[derive(Debug, Deserialize, Clone)]
pub struct ImageInfo {
    pub id: String,
    pub path: String,
}

// My assets structure
pub struct MyAssets {
    images: HashMap<String, TextureHandle>,
}

impl MyAssets {
    /// get texture
    pub fn get(&self, id: &str) -> Option<&TextureHandle> {
        self.images.get(id)
    }

    /// load all image from
    pub fn load_from_json(ctx: &Context) -> Self {
        // install image loader
        egui_extras::install_image_loaders(ctx);
        // load assets inside the code

        let mut images = HashMap::new();

        // for img in image_list {
        for (id, path) in IMAGE_LIST {
            let tex = Self::load_texture(ctx, id, path);
            images.insert(id.to_string(), tex);
        }

        Self { images }
    }

    /// load texture
    fn load_texture(ctx: &Context, id: &str, path: &str) -> TextureHandle {
        let data = ASSETS
            .get_file(path)
            .unwrap_or_else(|| panic!("Asset not found: {}", path))
            .contents();
        let image = image::load_from_memory(data)
            .expect(&format!("Failed to open image: {}", path))
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();
        let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);

        ctx.load_texture(id, color_image, TextureOptions::LINEAR)
    }
}

// language
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LanguageItem {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UiLabels {
    pub dashboard: String,
    pub start: String,
    pub pause: String,
    pub stop: String,
    pub difficulty: String,
}

#[derive(Debug, Deserialize)]
pub struct DifficultyLabels {
    pub easy: String,
    pub medium: String,
    pub hard: String,
}

#[derive(Debug, Deserialize)]
pub struct PopupLabels {
    pub game_over: String,
    pub ok: String,
}

#[derive(Debug, Deserialize)]
pub struct GameOver {
    pub title: String,
    pub ok: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    pub close_title: String,
    pub close_ok: String,
    pub close_bye: String,
}

#[derive(Debug, Deserialize)]
pub struct UiConfig {
    pub languages: Vec<LanguageItem>,
    pub app_name: String,
    pub labels: UiLabels,
    pub difficulty: DifficultyLabels,
    pub popup: PopupLabels,
    pub gameover: GameOver,
}

impl UiConfig {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| UiConfig {
            languages: vec![LanguageItem {
                code: "en".to_string(),
                name: "English".to_string(),
            }],
            app_name: "APPName".to_string(),
            labels: UiLabels {
                dashboard: "Dashboard".to_string(),
                start: "Start".to_string(),
                pause: "Pause".to_string(),
                stop: "Stop".to_string(),
                difficulty: "Difficulty".to_string(),
            },
            difficulty: DifficultyLabels {
                easy: "⭐".to_string(),
                medium: "⭐⭐".to_string(),
                hard: "⭐⭐⭐".to_string(),
            },
            popup: PopupLabels {
                game_over: "Game Over".to_string(),
                ok: "OK".to_string(),
            },
            gameover: GameOver {
                title: "Game Over".to_string(),
                ok: "OK".to_string(),
                error: "BAD! YOU GOT NOTHING!".to_string(),
                warning: "OOPS, you got ONLY 1.".to_string(),
                info: "You got {} Rustaceans.".to_string(),
                close_title: "ByeBye".to_string(),
                close_ok: "ByeBye".to_string(),
                close_bye: "Have a good day.\nSee you soon.".to_string(),
            },
        })
    }
}
