//! Filler GUI replay app — opens a native window and plays a `game.log`
//! recorded by the engine, with play/pause, speed, scrub bar, and step
//! controls.
//!
//! Build (only when the `gui` feature is enabled):
//!     cargo build --release --features gui --bin visualizer-gui
//!
//! Run:
//!     ./target/release/visualizer-gui                # then drop a log
//!     ./target/release/visualizer-gui ./game.log     # or open one directly
//!
//! Shortcuts: Space play/pause, Left/Right step, Home/End first/last.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

use filler::log_replay::{parse_all_frames, Frame};

fn main() -> eframe::Result<()> {
    let path = std::env::args().nth(1).map(PathBuf::from);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 800.0])
            .with_min_inner_size([700.0, 480.0])
            .with_title("Filler — replay"),
        ..Default::default()
    };
    eframe::run_native(
        "Filler",
        options,
        Box::new(|_cc| Box::new(ReplayApp::new(path))),
    )
}

struct ReplayApp {
    frames: Vec<Frame>,
    current: usize,
    playing: bool,
    fps: f32,
    last_step: Instant,
    file_path: Option<PathBuf>,
    error: Option<String>,
}

impl ReplayApp {
    fn new(path: Option<PathBuf>) -> Self {
        let mut app = Self {
            frames: Vec::new(),
            current: 0,
            playing: false,
            fps: 12.0,
            last_step: Instant::now(),
            file_path: None,
            error: None,
        };
        if let Some(p) = path {
            app.load_file(p);
        }
        app
    }

    fn load_file(&mut self, path: PathBuf) {
        match File::open(&path) {
            Ok(f) => {
                self.frames = parse_all_frames(BufReader::new(f));
                self.current = 0;
                self.playing = false;
                self.error = if self.frames.is_empty() {
                    Some("No Anfield frames found in this file.".into())
                } else {
                    None
                };
                self.file_path = Some(path);
            }
            Err(e) => {
                self.error = Some(format!("Failed to open: {}", e));
            }
        }
    }

    fn count_cells(&self) -> (usize, usize) {
        if self.frames.is_empty() {
            return (0, 0);
        }
        let frame = &self.frames[self.current];
        let mut p1 = 0usize;
        let mut p2 = 0usize;
        for row in &frame.rows {
            for c in row.chars() {
                match c {
                    '@' | 'a' => p1 += 1,
                    '$' | 's' => p2 += 1,
                    _ => {}
                }
            }
        }
        (p1, p2)
    }
}

fn cell_color(c: char) -> Color32 {
    match c {
        '@' => Color32::from_rgb(220, 80, 80),
        'a' => Color32::from_rgb(255, 200, 90),
        '$' => Color32::from_rgb(80, 140, 220),
        's' => Color32::from_rgb(130, 230, 230),
        _ => Color32::from_rgb(28, 28, 36),
    }
}

impl eframe::App for ReplayApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let dropped: Option<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .next()
        });
        if let Some(p) = dropped {
            self.load_file(p);
        }

        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.playing = !self.playing;
            }
            if i.key_pressed(egui::Key::ArrowRight) && !self.frames.is_empty() {
                self.current = (self.current + 1).min(self.frames.len() - 1);
                self.playing = false;
            }
            if i.key_pressed(egui::Key::ArrowLeft) && self.current > 0 {
                self.current -= 1;
                self.playing = false;
            }
            if i.key_pressed(egui::Key::Home) {
                self.current = 0;
            }
            if i.key_pressed(egui::Key::End) && !self.frames.is_empty() {
                self.current = self.frames.len() - 1;
            }
        });

        if self.playing && !self.frames.is_empty() {
            let step = Duration::from_secs_f32(1.0 / self.fps.max(0.1));
            if self.last_step.elapsed() >= step {
                if self.current + 1 < self.frames.len() {
                    self.current += 1;
                    self.last_step = Instant::now();
                } else {
                    self.playing = false;
                }
            }
            ctx.request_repaint_after(step);
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("Open log…").clicked() {
                    if let Some(p) = rfd::FileDialog::new()
                        .add_filter("Filler log", &["log", "txt"])
                        .add_filter("All files", &["*"])
                        .pick_file()
                    {
                        self.load_file(p);
                    }
                }
                ui.separator();

                let can_play = !self.frames.is_empty();
                let play_label = if self.playing { "Pause" } else { "Play" };
                if ui
                    .add_enabled(can_play, egui::Button::new(play_label))
                    .clicked()
                {
                    self.playing = !self.playing;
                    self.last_step = Instant::now();
                }
                if ui
                    .add_enabled(can_play, egui::Button::new("|<<"))
                    .on_hover_text("First frame (Home)")
                    .clicked()
                {
                    self.current = 0;
                    self.playing = false;
                }
                if ui
                    .add_enabled(self.current > 0, egui::Button::new("<"))
                    .on_hover_text("Previous frame (Left)")
                    .clicked()
                {
                    self.current -= 1;
                    self.playing = false;
                }
                if ui
                    .add_enabled(
                        can_play && self.current + 1 < self.frames.len(),
                        egui::Button::new(">"),
                    )
                    .on_hover_text("Next frame (Right)")
                    .clicked()
                {
                    self.current += 1;
                    self.playing = false;
                }
                if ui
                    .add_enabled(can_play, egui::Button::new(">>|"))
                    .on_hover_text("Last frame (End)")
                    .clicked()
                {
                    self.current = self.frames.len().saturating_sub(1);
                    self.playing = false;
                }

                ui.separator();
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.fps, 1.0..=60.0).suffix(" fps"));

                ui.separator();
                if !self.frames.is_empty() {
                    let max = self.frames.len() - 1;
                    let mut cur = self.current;
                    let resp = ui.add(
                        egui::Slider::new(&mut cur, 0..=max).text(format!("/ {}", max)),
                    );
                    if resp.changed() {
                        self.current = cur;
                        self.playing = false;
                    }
                }
            });
            ui.add_space(4.0);
        });

        egui::SidePanel::right("info")
            .resizable(false)
            .default_width(230.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("Filler replay");
                ui.separator();
                if let Some(p) = &self.file_path {
                    let name = p
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    ui.label(format!("Log: {}", name));
                } else {
                    ui.label("Drop a game.log on this window, or click Open log….");
                }
                if let Some(err) = &self.error {
                    ui.add_space(4.0);
                    ui.colored_label(Color32::from_rgb(240, 100, 100), err);
                }
                if !self.frames.is_empty() {
                    ui.add_space(10.0);
                    ui.label(format!("Frame: {} / {}", self.current + 1, self.frames.len()));
                    let (p1, p2) = self.count_cells();
                    ui.add_space(6.0);
                    ui.colored_label(cell_color('@'), format!("Player 1: {}", p1));
                    ui.colored_label(cell_color('$'), format!("Player 2: {}", p2));
                    let winner_hint = match p1.cmp(&p2) {
                        std::cmp::Ordering::Greater => "P1 leading",
                        std::cmp::Ordering::Less => "P2 leading",
                        std::cmp::Ordering::Equal => "tied",
                    };
                    ui.add_space(2.0);
                    ui.small(winner_hint);
                }
                ui.add_space(14.0);
                ui.separator();
                ui.label("Shortcuts");
                ui.small("Space    play / pause");
                ui.small("Left/Right    step");
                ui.small("Home/End    first / last");

                ui.add_space(12.0);
                ui.separator();
                ui.label("Legend");
                ui.colored_label(cell_color('@'), "P1 own (@)");
                ui.colored_label(cell_color('a'), "P1 last move (a)");
                ui.colored_label(cell_color('$'), "P2 own ($)");
                ui.colored_label(cell_color('s'), "P2 last move (s)");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.frames.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No log loaded. Open a game.log or drop one onto this window.");
                });
                return;
            }
            let frame = &self.frames[self.current];
            let avail = ui.available_size();
            let w = frame.width as f32;
            let h = frame.height as f32;
            if w <= 0.0 || h <= 0.0 {
                return;
            }
            let cell = ((avail.x - 16.0) / w)
                .min((avail.y - 16.0) / h)
                .floor()
                .max(1.0);
            let board_w = cell * w;
            let board_h = cell * h;

            let (response, painter) = ui.allocate_painter(avail, egui::Sense::hover());
            let origin = response.rect.min
                + Vec2::new(
                    ((avail.x - board_w) * 0.5).max(0.0),
                    ((avail.y - board_h) * 0.5).max(0.0),
                );

            painter.rect_filled(
                Rect::from_min_size(origin, Vec2::new(board_w, board_h)),
                4.0,
                Color32::from_rgb(20, 20, 26),
            );

            for (y, row) in frame.rows.iter().enumerate() {
                for (x, c) in row.chars().enumerate() {
                    if x >= frame.width {
                        break;
                    }
                    if c == '.' {
                        continue;
                    }
                    let rect = Rect::from_min_size(
                        Pos2::new(origin.x + x as f32 * cell, origin.y + y as f32 * cell),
                        Vec2::splat(cell),
                    )
                    .shrink(0.5);
                    painter.rect_filled(rect, 0.0, cell_color(c));
                    if c == 'a' || c == 's' {
                        painter.rect_stroke(rect, 0.0, Stroke::new(1.5, Color32::WHITE));
                    }
                }
            }
        });
    }
}
