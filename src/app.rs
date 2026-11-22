use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;
use egui::{Context, Key};
use crate::config::AppConfig;
use crate::telemetry::models::{HistoryPoint, SystemSnapshot};
use crate::telemetry::TelemetryCollector;
use crate::theme::{ThemeMode, ThemePalette};
use crate::ui::cards::*;
use crate::ui::chart::draw_history_chart;
use crate::ui::header::render_header;

pub struct NexusApp {
    config: AppConfig,
    snapshot: SystemSnapshot,
    history: Vec<HistoryPoint>,
    rx: Receiver<SystemSnapshot>,
}

fn apply_theme_to_ctx(ctx: &Context, mode: ThemeMode) {
    let palette = ThemePalette::for_mode(mode);
    ctx.style_mut(|style| {
        style.visuals.panel_fill = palette.background;
        style.visuals.window_fill = palette.background;
        style.visuals.extreme_bg_color = palette.card_bg;
    });
}

impl NexusApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        let (tx, rx) = channel::<SystemSnapshot>();

        // Apply theme to context once at creation time
        apply_theme_to_ctx(&cc.egui_ctx, config.theme);

        // Initial sample
        let mut initial_collector = TelemetryCollector::new();
        let initial_snapshot = initial_collector.sample();

        // Spawn background telemetry sampler thread and request repaint on each new sample
        let egui_ctx = cc.egui_ctx.clone();
        thread::spawn(move || {
            let mut collector = initial_collector;
            loop {
                thread::sleep(Duration::from_millis(1000));
                let snap = collector.sample();
                if tx.send(snap).is_ok() {
                    egui_ctx.request_repaint();
                } else {
                    break;
                }
            }
        });

        // Seed 60 history points so the graph is immediately populated with realistic history
        let now_epoch = chrono::Utc::now().timestamp_millis() as f64 / 1000.0;
        let mut history = Vec::new();
        for i in (0..60).rev() {
            let t = now_epoch - (i as f64);
            history.push(HistoryPoint {
                timestamp_secs: t,
                cpu_pct: initial_snapshot.cpu.utilization_pct,
                gpu_pct: initial_snapshot.gpu.utilization_pct,
            });
        }

        Self {
            config,
            snapshot: initial_snapshot,
            history,
            rx,
        }
    }
}

impl eframe::App for NexusApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let palette = ThemePalette::for_mode(self.config.theme);
        palette.background.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle theme switching keyboard shortcuts 1..6
        let mut theme_changed = false;
        ctx.input(|i| {
            if i.key_pressed(Key::Num1) {
                self.config.theme = ThemeMode::Vintage;
                theme_changed = true;
            } else if i.key_pressed(Key::Num2) {
                self.config.theme = ThemeMode::Nebula;
                theme_changed = true;
            } else if i.key_pressed(Key::Num3) {
                self.config.theme = ThemeMode::Clearspace;
                theme_changed = true;
            } else if i.key_pressed(Key::Num4) {
                self.config.theme = ThemeMode::Forge;
                theme_changed = true;
            } else if i.key_pressed(Key::Num5) {
                self.config.theme = ThemeMode::Terminal;
                theme_changed = true;
            } else if i.key_pressed(Key::Num6) {
                self.config.theme = ThemeMode::Aura;
                theme_changed = true;
            }
        });

        // Drain telemetry updates from worker thread
        let mut received_new = false;
        while let Ok(new_snap) = self.rx.try_recv() {
            self.snapshot = new_snap;
            received_new = true;
        }

        if received_new {
            let now_epoch = chrono::Utc::now().timestamp_millis() as f64 / 1000.0;
            self.history.push(HistoryPoint {
                timestamp_secs: now_epoch,
                cpu_pct: self.snapshot.cpu.utilization_pct,
                gpu_pct: self.snapshot.gpu.utilization_pct,
            });

            // Keep bounded 60 seconds
            if self.history.len() > 60 {
                self.history.remove(0);
            }
        }

        let palette = ThemePalette::for_mode(self.config.theme);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(palette.background).inner_margin(16.0))
            .show(ctx, |ui| {
                let prev_theme = self.config.theme;

                // Header
                render_header(ui, &self.snapshot.info, &mut self.config.theme, &palette);

                if self.config.theme != prev_theme {
                    theme_changed = true;
                }

                if theme_changed {
                    self.config.save();
                    apply_theme_to_ctx(ctx, self.config.theme);
                }

                let col_gap = 12.0;
                let row_gap = 12.0;
                ui.add_space(row_gap);

                let available_w = ui.available_width();
                let available_h = ui.available_height();

                // Dynamic row height: allocate half the available height to row 1
                let row1_h = ((available_h - row_gap) * 0.5).max(150.0);

                // Top row: 3 metric cards + 1 wide chart
                // Metric cards take ~18.5% width each, chart takes remaining space
                let card_w = ((available_w - (col_gap * 3.0)) * 0.185).max(180.0);

                ui.horizontal(|ui| {
                    render_cpu_card(ui, &self.snapshot.cpu, &palette, card_w, row1_h);
                    ui.add_space(col_gap);

                    render_gpu_card(ui, &self.snapshot.gpu, &palette, card_w, row1_h);
                    ui.add_space(col_gap);

                    render_memory_card(ui, &self.snapshot.memory, &palette, card_w, row1_h);
                    ui.add_space(col_gap);

                    let chart_w = ui.available_width().max(280.0);
                    draw_history_chart(ui, &self.history, &palette, chart_w, row1_h);
                });

                ui.add_space(row_gap);

                // Bottom row: 4 cards (Storage, Top processes, Network, Cooling)
                // Dynamically take all remaining height to fully span to the bottom inner margin
                let row2_h = ui.available_height().max(150.0);
                let row2_col_w = ((available_w - (col_gap * 3.0)) / 4.0).max(180.0);

                ui.horizontal(|ui| {
                    render_storage_card(ui, &self.snapshot.drives, &palette, row2_col_w, row2_h);
                    ui.add_space(col_gap);

                    render_processes_card(ui, &self.snapshot.processes, &palette, row2_col_w, row2_h);
                    ui.add_space(col_gap);

                    render_network_card(ui, &self.snapshot.network, &palette, row2_col_w, row2_h);
                    ui.add_space(col_gap);

                    let cooling_w = ui.available_width().max(180.0);
                    render_cooling_card(ui, &self.snapshot.cooling, &palette, cooling_w, row2_h);
                });
            });

        // Repaint periodically for telemetry and time updates (every 1 second)
        ctx.request_repaint_after(Duration::from_millis(1000));
    }
}
