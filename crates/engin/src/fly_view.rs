// view
use chrono::Local;
use eframe::egui::{self, Pos2, RichText};
use egui::ViewportCommand;
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};

use crate::config::{Difficulty, IconType, Popup, WorldConfig};
use crate::fly_viewmodel::FlyViewModel;

use util::assets::{I18NUIJSON, I18NUIZHCNJSON, I18NUIZHTWJSON, LanguageItem, MyAssets, UiConfig};
use util::setup_custom_fonts;
use util::{MyScore, get_resource_path_str};

// ---------- MyApp ----------
pub struct MyApp {
    vm: FlyViewModel,
    rx: Receiver<()>,
    assets: MyAssets,
    language: LanguageItem,
    ui_config: UiConfig,
    now_time_display: String,
    dash_board_display: String,
    popup: Popup,
    check_exit: bool,
    should_exit: bool,
    score: MyScore,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set custom font
        setup_custom_fonts(&cc.egui_ctx);

        // load image assets
        let assets = MyAssets::load_from_json(&cc.egui_ctx);

        // load i18n
        let path = &get_resource_path_str(I18NUIJSON);
        let ui_config = UiConfig::load(path);
        let language: LanguageItem = ui_config.languages[0].clone(); // default English // crash when package to macOS app

        // timer thread
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            // interval 0.1 seconds
            let interval = Duration::from_secs_f64(0.1);
            let mut next_tick = Instant::now();
            loop {
                // make sure the thread interval is fixed, not effected via inner program
                next_tick += interval;
                let _ = tx.send(());
                // request repaint when wake up
                let now = Instant::now();
                if next_tick > now {
                    std::thread::sleep(next_tick - now);
                } else if now - next_tick > Duration::from_secs(1) {
                    next_tick = now;
                }
            }
        });

        // score sqlite
        let score = MyScore::new().expect("Failed to open db file");

        Self {
            vm: FlyViewModel::new(WorldConfig::default()),
            rx,
            assets,
            language,
            ui_config,
            now_time_display: String::from(" "),
            dash_board_display: String::from(" "),
            popup: Popup::None,
            check_exit: false,
            should_exit: false,
            score,
        }
    }

    // ---------- pop-up ----------
    pub fn show_popup(
        &mut self,
        ctx: &egui::Context,
        title: &str,
        message: &str,
        icon: IconType,
        ok_text: &str,
    ) {
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_sized(
                        egui::vec2(480.0, 100.0),
                        egui::Label::new(
                            RichText::new(format!("{} {}", icon.to_emoji(), message))
                                .size(28.0)
                                .strong(),
                        ),
                    );
                    ui.add_space(12.0);

                    if ui
                        .add(
                            egui::Button::new(RichText::new(ok_text).size(28.0).strong())
                                .min_size(egui::vec2(80.0, 30.0)),
                        )
                        .clicked()
                    {
                        self.popup = Popup::None;
                        if self.check_exit {
                            self.should_exit = true;
                        }
                        // add player and score
                        self.score.score = self.vm.get_game_result_message() as i32;
                        self.score.add();
                    }
                });
                ui.add_space(25.0);
                // player name input
                if self.popup == Popup::GameOver {
                    ui.vertical_centered(|ui| {
                        let data_label = ui.label(
                            egui::RichText::new(&self.ui_config.gameover.player).monospace(),
                        );
                        egui::Frame::NONE.show(ui, |ui| {
                            ui.text_edit_singleline(&mut self.score.name)
                                .labelled_by(data_label.id)
                        });
                    });
                }
                // show players' score
                ui.horizontal(|ui| {
                    // print list
                    self.score.output.clear();
                    self.score.output.push_str(&self.ui_config.gameover.ranking);
                    self.score.list();
                    ui.label(egui::RichText::new(&self.score.output).monospace());
                });
            });
    }

    // ---------- difficulty radio ----------
    pub fn show_difficulty_radio_on_canvas(&mut self, ctx: &egui::Context) {
        egui::Area::new("difficulty_radio".into())
            .fixed_pos(Pos2::new(8.0, 570.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🛠").size(20.0).strong());

                    ui.radio_value(
                        &mut self.vm.difficulty,
                        Difficulty::Easy,
                        RichText::new("⭐").size(20.0).strong(),
                    );
                    ui.radio_value(
                        &mut self.vm.difficulty,
                        Difficulty::Medium,
                        RichText::new("⭐⭐").size(20.0).strong(),
                    );
                    ui.radio_value(
                        &mut self.vm.difficulty,
                        Difficulty::Hard,
                        RichText::new("⭐⭐⭐").size(20.0).strong(),
                    );

                    self.vm.apply_difficulty();
                });
            });
    }

    // ---------- draw fly ----------
    fn draw_rustacean(&mut self, ctx: &eframe::egui::Context) {
        egui::Area::new("fly_rustacean".into())
            .fixed_pos(egui::pos2(self.vm.fly_x, self.vm.fly_y))
            .show(ctx, |ui| {
                if let Some(tex) = self.assets.get("Rustacean") {
                    let img = egui::Image::new(tex).fit_to_exact_size(egui::vec2(60.0, 60.0));

                    if ui.add(egui::Button::image(img).frame(false)).clicked() {
                        self.vm.fly_click();
                    }
                }
            });
    }

    // ---------- draw fly hammer ----------
    fn draw_fly_hand(&self, ctx: &eframe::egui::Context) {
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            if let Some(tex) = self.assets.get("Hand") {
                let painter = ctx.layer_painter(egui::LayerId::new(
                    egui::Order::Foreground,
                    egui::Id::new("hand_layer"),
                ));

                painter.image(
                    tex.id(),
                    egui::Rect::from_center_size(pos, egui::vec2(50.0, 50.0)),
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
    }

    fn language_selector(&mut self, ctx: &egui::Context) {
        let mut next_lang_code: Option<String> = None;

        egui::Area::new("language_selector".into())
            .fixed_pos(egui::Pos2::new(680.0, 560.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🌐").size(20.0));

                    egui::ComboBox::from_id_salt("language_combo")
                        .selected_text(egui::RichText::new(&self.language.name).size(14.0))
                        .width(80.0)
                        .show_ui(ui, |ui| {
                            for lang in &self.ui_config.languages {
                                if ui
                                    .selectable_value(
                                        &mut self.language,
                                        lang.clone(),
                                        egui::RichText::new(&lang.name).size(14.0),
                                    )
                                    .clicked()
                                {
                                    next_lang_code = Some(lang.code.clone());
                                }
                            }
                        });
                });
            });

        if let Some(code) = next_lang_code {
            let path_lang = match code.as_str() {
                "en" => I18NUIJSON,
                "zh-TW" => I18NUIZHTWJSON,
                "zh-CN" => I18NUIZHCNJSON,
                _ => I18NUIJSON,
            };
            let path = &get_resource_path_str(path_lang);
            self.ui_config = UiConfig::load(path);
        }

        // update windows title
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
            self.ui_config.app_name.clone(),
        ));
    }
}

// ---------- App trait ----------
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // check exit
        if self.check_exit {
            self.popup = Popup::Exit;
        }
        if self.should_exit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        // timer tick procedure
        if let Ok(_) = self.rx.try_recv() {
            if self.vm.tick() {
                self.popup = Popup::GameOver;
            }

            // update time
            let now_time = Local::now();
            self.now_time_display = now_time.format("%Y-%m-%d %H:%M:%S\n").to_string();

            // update dash board
            let (str_sec, str_score) = self.vm.dash_board_info();

            let str_disp = self
                .ui_config
                .labels
                .dashboard
                .replace("{time}", &str_sec)
                .replace("{score}", &str_score);

            self.dash_board_display = str_disp;
            // request repaint per tick
            ctx.request_repaint();
        }

        // central panel
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.inner_margin(0.0))
            .show(ctx, |ui| {
                if let Some(tex) = self.assets.get("Beach") {
                    ui.add(egui::Image::new(tex).fit_to_exact_size(ui.available_size()));
                }

                let content_rect = ui.max_rect();
                ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    let mut draw_label = |pos: Pos2, size: f32, text: &str, width: f32| {
                        let rect = egui::Rect::from_min_size(pos, egui::vec2(width, 50.0));
                        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| ui.label(RichText::new(text).size(size).monospace()),
                            );
                        });
                    };
                    // display time
                    draw_label(Pos2::new(650.0, 0.0), 12.0, &self.now_time_display, 600.0);
                    // display count score
                    draw_label(Pos2::new(10.0, 0.0), 28.0, &self.dash_board_display, 300.0);

                    // control buttons
                    // button: start / pause
                    let start_pause_pos = Pos2::new(700.0, 40.0);
                    let start_pause_rect =
                        egui::Rect::from_min_size(start_pause_pos, egui::vec2(40.0, 40.0));
                    if ui
                        .put(
                            start_pause_rect,
                            egui::Button::new(RichText::new(self.vm.state.icon()).size(28.0)),
                        )
                        .clicked()
                    {
                        if self.popup == Popup::None {
                            self.vm.toggle_run();
                        }
                    }

                    // button stop
                    let stop_pos = Pos2::new(750.0, 40.0);
                    let stop_rect = egui::Rect::from_min_size(stop_pos, egui::vec2(40.0, 40.0));
                    if ui
                        .put(stop_rect, egui::Button::new(RichText::new("⏹").size(28.0)))
                        .clicked()
                    {
                        if self.popup == Popup::None {
                            self.check_exit = self.vm.stop();
                        }
                    }
                });
            });

        // difficulty radio
        self.show_difficulty_radio_on_canvas(ctx);

        // language select
        self.language_selector(ctx);

        // draw fly
        self.draw_rustacean(ctx);

        // popup message box
        match self.popup {
            Popup::Exit => {
                // pop exit message
                let title = &self.ui_config.gameover.close_title.clone();
                let ok = &self.ui_config.gameover.close_ok.clone();
                let message = &self.ui_config.gameover.close_bye.clone();
                let icon = IconType::Info;
                self.show_popup(ctx, title, &message, icon, ok);
            }
            Popup::GameOver => {
                // report score
                let score = self.vm.get_game_result_message();
                let title = &self.ui_config.gameover.title.clone();
                let ok = &self.ui_config.gameover.ok.clone();
                let (message, icon) = match score {
                    0 => (self.ui_config.gameover.error.clone(), IconType::Error),
                    1 => (self.ui_config.gameover.warning.clone(), IconType::Warning),
                    _ => (
                        self.ui_config
                            .gameover
                            .info
                            .replace("{score}", &score.to_string()),
                        IconType::Custom("🏆"),
                    ),
                };
                self.show_popup(ctx, title, &message, icon, ok);
            }
            Popup::None => {}
        }
        // draw fly hammer
        self.draw_fly_hand(ctx);

        // hidden mouse cursor
        ctx.set_cursor_icon(egui::CursorIcon::None);

        // request repaint per frame
        ctx.request_repaint();
    }
}
