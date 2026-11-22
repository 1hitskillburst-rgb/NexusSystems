use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use egui::Context;
use crate::config::AppConfig;
use crate::telemetry::models::{HistoryPoint, SystemSnapshot};
use crate::telemetry::recording::PerformanceRecordingSession;
use crate::telemetry::TelemetryCollector;
use crate::theme::ThemePalette;
use crate::ui::cards::*;
use crate::ui::header::render_header;
use crate::ui::recording_chart::{render_chart_panel, ChartViewMode};
use crate::ui::settings::render_settings_window;


pub struct NexusApp {
    config: AppConfig,
    snapshot: SystemSnapshot,
    history: Vec<HistoryPoint>,
    rx: Receiver<SystemSnapshot>,
    running: Arc<AtomicBool>,
    refresh_interval: Arc<AtomicU64>,
    worker_handle: Option<JoinHandle<()>>,
    settings_open: bool,
    recording: PerformanceRecordingSession,
    chart_view_mode: ChartViewMode,
}


fn apply_theme_to_ctx(ctx: &Context) {
    let palette = ThemePalette::vintage();
    ctx.style_mut(|style| {
        style.visuals.panel_fill = palette.background;
        style.visuals.window_fill = palette.background;
        style.visuals.extreme_bg_color = palette.card_bg;
    });
}

impl NexusApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::with_context(cc.egui_ctx.clone())
    }

    pub fn with_context(egui_ctx: egui::Context) -> Self {
        let config = AppConfig::load();
        let (tx, rx) = channel::<SystemSnapshot>();

        // Apply theme to context once at creation time
        apply_theme_to_ctx(&egui_ctx);

        // Initial sample
        let mut initial_collector = TelemetryCollector::new();
        let initial_snapshot = initial_collector.sample();

        let running = Arc::new(AtomicBool::new(true));
        let is_running = Arc::clone(&running);
        let refresh_interval = Arc::new(AtomicU64::new(config.refresh_interval_ms));
        let interval_clone = Arc::clone(&refresh_interval);

        let ctx = egui_ctx.clone();
        let worker_handle = thread::spawn(move || {
            let mut collector = initial_collector;
            while is_running.load(Ordering::Relaxed) {
                // Sleep in dynamic 50ms slices to allow prompt termination and responsive rate changes
                let target_ms = interval_clone.load(Ordering::Relaxed).max(100);
                let slices = (target_ms / 50).max(1);
                for _ in 0..slices {
                    if !is_running.load(Ordering::Relaxed) {
                        return;
                    }
                    thread::sleep(Duration::from_millis(50));
                }
                if !is_running.load(Ordering::Relaxed) {
                    break;
                }
                let snap = collector.sample();
                if tx.send(snap).is_ok() {
                    ctx.request_repaint();
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
            running,
            refresh_interval,
            worker_handle: Some(worker_handle),
            settings_open: false,
            recording: PerformanceRecordingSession::new(),
            chart_view_mode: ChartViewMode::Live,
        }

    }
}

impl eframe::App for NexusApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let palette = ThemePalette::vintage();
        palette.background.to_normalized_gamma_f32()
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
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

            // Capture telemetry snapshot into active recording session
            self.recording.add_sample(&self.snapshot);
        }

        // Request prompt repaint if actively recording so live duration counter is smooth
        if self.recording.is_recording() {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        let palette = ThemePalette::vintage();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(palette.background).inner_margin(16.0))
            .show(ctx, |ui| {
                // Skip rendering if window is minimized or collapsed during rapid drag
                if ui.available_width() < 50.0 || ui.available_height() < 50.0 {
                    return;
                }

                // Header with Settings Cog Button and Performance Recording Controls
                render_header(
                    ui,
                    &self.snapshot.info,
                    &palette,
                    &mut self.settings_open,
                    &mut self.recording,
                    Some(&mut self.chart_view_mode),
                );

                let col_gap = 10.0;
                let row_gap = 12.0;
                ui.add_space(row_gap);

                let available_w = ui.available_width();
                let available_h = ui.available_height();

                let show_storage = self.config.show_storage;
                let show_processes = self.config.show_processes;
                let show_network = self.config.show_network;
                let cooling_has_data = self.snapshot.cooling.has_data();
                let show_cooling = self.config.show_cooling && (!self.config.auto_hide_empty_cards || cooling_has_data);
                let has_row2 = show_storage || show_processes || show_network || show_cooling;

                // Dynamic row height: allocate half height if secondary row exists, else full height
                let row1_h = if has_row2 {
                    ((available_h - row_gap) * 0.5).max(150.0)
                } else {
                    available_h.max(150.0)
                };
                let (col_w, chart_w) = calculate_grid_columns(available_w, col_gap);

                // Row 1: Primary Metrics
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    render_cpu_card(ui, &self.snapshot.cpu, &palette, col_w, row1_h);
                    ui.add_space(col_gap);

                    render_gpu_card(ui, &self.snapshot.gpu, &palette, col_w, row1_h);
                    ui.add_space(col_gap);

                    render_memory_card(ui, &self.snapshot.memory, &palette, col_w, row1_h);
                    ui.add_space(col_gap);

                    render_chart_panel(
                        ui,
                        &self.history,
                        &self.recording,
                        &palette,
                        &mut self.chart_view_mode,
                        chart_w,
                        row1_h,
                    );
                });


                // Row 2: Secondary Metrics (Configurable Visibility & Smart Auto-Collapse)
                if has_row2 {
                    ui.add_space(row_gap);
                    let row2_h = ui.available_height().max(150.0);

                    let visible_count = (show_storage as usize)
                        + (show_processes as usize)
                        + (show_network as usize)
                        + (show_cooling as usize);

                    if visible_count == 4 {
                        // 4 cards visible: maintain aligned 4-column geometry with Row 1
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            render_storage_card(ui, &self.snapshot.drives, &palette, col_w, row2_h);
                            ui.add_space(col_gap);

                            render_processes_card(ui, &self.snapshot.processes, &palette, col_w, row2_h);
                            ui.add_space(col_gap);

                            render_network_card(ui, &self.snapshot.network, &palette, col_w, row2_h);
                            ui.add_space(col_gap);

                            render_cooling_card(ui, &self.snapshot.cooling, &palette, chart_w, row2_h);
                        });
                    } else if visible_count == 3 && show_storage && show_processes && show_network {
                        // 3 primary cards visible (Storage, Processes, Network - Cooling collapsed):
                        // Storage in Col 1 (col_w), Top Processes spans Col 2+3 (col_w * 2.0 + col_gap), Network in Col 4 (chart_w)
                        // Every vertical column edge matches Row 1 exactly with ZERO blank whitespace squares!
                        let procs_w = col_w * 2.0 + col_gap;
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            render_storage_card(ui, &self.snapshot.drives, &palette, col_w, row2_h);
                            ui.add_space(col_gap);

                            render_processes_card(ui, &self.snapshot.processes, &palette, procs_w, row2_h);
                            ui.add_space(col_gap);

                            render_network_card(ui, &self.snapshot.network, &palette, chart_w, row2_h);
                        });
                    } else {
                        // Dynamic distribution across visible secondary cards
                        let total_gaps = col_gap * ((visible_count - 1) as f32);
                        let dynamic_w = ((available_w - total_gaps) / (visible_count as f32)).max(100.0);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            let mut first = true;

                            if show_storage {
                                if !first { ui.add_space(col_gap); }
                                first = false;
                                render_storage_card(ui, &self.snapshot.drives, &palette, dynamic_w, row2_h);
                            }
                            if show_processes {
                                if !first { ui.add_space(col_gap); }
                                first = false;
                                render_processes_card(ui, &self.snapshot.processes, &palette, dynamic_w, row2_h);
                            }
                            if show_network {
                                if !first { ui.add_space(col_gap); }
                                first = false;
                                render_network_card(ui, &self.snapshot.network, &palette, dynamic_w, row2_h);
                            }
                            if show_cooling {
                                if !first { ui.add_space(col_gap); }
                                render_cooling_card(ui, &self.snapshot.cooling, &palette, dynamic_w, row2_h);
                            }
                        });
                    }
                }
            });

        // Settings Modal Component
        let settings_res = render_settings_window(ctx, &mut self.settings_open, &mut self.config, &palette);
        if settings_res.changed {
            self.refresh_interval.store(self.config.refresh_interval_ms, Ordering::Relaxed);
            self.config.save();
        }

        // Repaint periodically for telemetry and time updates
        ctx.request_repaint_after(Duration::from_millis(self.config.refresh_interval_ms.min(1000)));
    }
}

impl Drop for NexusApp {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}

/// Calculates standardized 4-column layout widths for a given available width and column gap.
/// Returns `(col_w, col4_w)` where `col_w` applies to columns 1-3 and `col4_w` applies to column 4.
pub fn calculate_grid_columns(available_w: f32, col_gap: f32) -> (f32, f32) {
    let total_gaps = col_gap * 3.0;
    let net_w = (available_w - total_gaps).max(400.0);
    let col_w = (net_w / 4.0).max(100.0);
    let col4_w = (available_w - (col_w * 3.0) - total_gaps).max(100.0);
    (col_w, col4_w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::App;

    fn create_test_app() -> NexusApp {
        NexusApp::with_context(egui::Context::default())
    }

    #[test]
    fn test_worker_thread_stops_promptly_on_cancellation_flag() {
        let mut app = create_test_app();
        assert!(app.running.load(Ordering::SeqCst));
        assert!(app.worker_handle.is_some());

        let handle = app.worker_handle.take().unwrap();
        let running = Arc::clone(&app.running);

        let start = std::time::Instant::now();
        running.store(false, Ordering::SeqCst);
        let join_res = handle.join();
        let elapsed = start.elapsed();

        assert!(join_res.is_ok(), "Worker thread panicked");
        assert!(
            elapsed < Duration::from_millis(250),
            "Worker thread took too long to exit: {:?}",
            elapsed
        );
    }

    #[test]
    fn test_app_on_exit_terminates_and_joins_thread() {
        let mut app = create_test_app();
        assert!(app.worker_handle.is_some());

        let start = std::time::Instant::now();
        app.on_exit(None);
        let elapsed = start.elapsed();

        assert!(!app.running.load(Ordering::SeqCst));
        assert!(app.worker_handle.is_none());
        assert!(
            elapsed < Duration::from_millis(250),
            "on_exit took too long to join worker thread: {:?}",
            elapsed
        );

        // Calling on_exit again is safe and idempotent
        app.on_exit(None);
        assert!(app.worker_handle.is_none());
    }

    #[test]
    fn test_app_drop_terminates_and_joins_thread() {
        let app = create_test_app();
        let running = Arc::clone(&app.running);
        assert!(running.load(Ordering::SeqCst));

        let start = std::time::Instant::now();
        drop(app);
        let elapsed = start.elapsed();

        assert!(!running.load(Ordering::SeqCst));
        assert!(
            elapsed < Duration::from_millis(250),
            "Drop took too long to join worker thread: {:?}",
            elapsed
        );
    }

    #[test]
    fn test_app_on_exit_then_drop_safety() {
        let mut app = create_test_app();
        app.on_exit(None);
        assert!(app.worker_handle.is_none());
        // Dropping after on_exit must be completely safe without double join or panic
        drop(app);
    }

    #[test]
    fn test_app_settings_initial_state() {
        let app = create_test_app();
        assert!(!app.settings_open);
    }

    #[test]
    fn test_grid_column_widths_alignment_across_rows() {
        let col_gap = 10.0;
        let test_widths = [400.0, 800.0, 960.0, 1188.0, 1220.0, 1920.0];

        for &available_w in &test_widths {
            let total_gaps = col_gap * 3.0;
            let (col_w, col4_w) = calculate_grid_columns(available_w, col_gap);

            // Row 1 columns: CPU (col_w), GPU (col_w), RAM (col_w), Chart (col4_w)
            let row1_cols = [col_w, col_w, col_w, col4_w];
            // Row 2 columns: Storage (col_w), Processes (col_w), Network (col_w), Cooling (col4_w)
            let row2_cols = [col_w, col_w, col_w, col4_w];

            // Verify columns 1-3 have identical, aligned widths across Row 1 and Row 2
            assert_eq!(row1_cols[0], row2_cols[0], "Column 1 mismatch at available_w={}", available_w);
            assert_eq!(row1_cols[1], row2_cols[1], "Column 2 mismatch at available_w={}", available_w);
            assert_eq!(row1_cols[2], row2_cols[2], "Column 3 mismatch at available_w={}", available_w);
            assert_eq!(row1_cols[3], row2_cols[3], "Column 4 mismatch at available_w={}", available_w);

            // Verify each column is at least 100.0 px wide (no negative or collapsed geometry)
            for (idx, &w) in row1_cols.iter().enumerate() {
                assert!(w >= 100.0, "Column {} width ({}) < 100.0 at available_w={}", idx + 1, w, available_w);
            }

            // Verify that for standard width (available_w >= 430.0), total width matches available_w
            if available_w >= 430.0 {
                let total_rendered_w = col_w * 3.0 + col4_w + total_gaps;
                assert!(
                    (total_rendered_w - available_w).abs() < 1e-4,
                    "Total grid width ({}) must span available width ({})",
                    total_rendered_w,
                    available_w
                );
            }
        }
    }

    #[test]
    fn test_grid_column_widths_under_extreme_constraint() {
        let col_gap = 10.0;
        // Even with extremely small or negative available width, columns are clamped to >= 100.0
        let (col_w, col4_w) = calculate_grid_columns(50.0, col_gap);
        assert_eq!(col_w, 100.0);
        assert_eq!(col4_w, 100.0);

        let (col_w_neg, col4_w_neg) = calculate_grid_columns(-100.0, col_gap);
        assert_eq!(col_w_neg, 100.0);
        assert_eq!(col4_w_neg, 100.0);
    }

    #[test]
    fn test_app_recording_initial_state() {
        let app = create_test_app();
        assert!(app.recording.is_idle());
        assert!(app.recording.is_empty());
        assert_eq!(app.chart_view_mode, ChartViewMode::Live);
    }

    #[test]
    fn test_app_recording_start_capture_stop_and_discard() {
        let mut app = create_test_app();

        // 1. Start recording
        app.recording.start();
        app.chart_view_mode = ChartViewMode::Recording;
        assert!(app.recording.is_recording());
        assert_eq!(app.chart_view_mode, ChartViewMode::Recording);

        // 2. Add sample
        app.recording.add_sample(&app.snapshot);
        assert_eq!(app.recording.sample_count(), 1);

        // 3. Stop recording
        app.recording.stop();
        assert!(app.recording.is_stopped());
        assert_eq!(app.recording.sample_count(), 1);

        // 4. Discard immediately purges data
        app.recording.discard();
        app.chart_view_mode = ChartViewMode::Live;
        assert!(app.recording.is_idle());
        assert!(app.recording.is_empty());
        assert_eq!(app.chart_view_mode, ChartViewMode::Live);
    }

    #[test]
    fn test_app_update_frame_renders_cleanly() {
        let mut app = create_test_app();
        let palette = ThemePalette::vintage();
        let ctx = egui::Context::default();

        // Render header and chart panel in Live mode
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(
                    ui,
                    &app.snapshot.info,
                    &palette,
                    &mut app.settings_open,
                    &mut app.recording,
                    Some(&mut app.chart_view_mode),
                );
                render_chart_panel(
                    ui,
                    &app.history,
                    &app.recording,
                    &palette,
                    &mut app.chart_view_mode,
                    350.0,
                    180.0,
                );
            });
        });

        // Start recording and switch to recording graph
        app.recording.start();
        app.recording.add_sample(&app.snapshot);
        app.chart_view_mode = ChartViewMode::Recording;

        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(
                    ui,
                    &app.snapshot.info,
                    &palette,
                    &mut app.settings_open,
                    &mut app.recording,
                    Some(&mut app.chart_view_mode),
                );
                render_chart_panel(
                    ui,
                    &app.history,
                    &app.recording,
                    &palette,
                    &mut app.chart_view_mode,
                    350.0,
                    180.0,
                );
            });
        });

        // Stop and render
        app.recording.stop();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(
                    ui,
                    &app.snapshot.info,
                    &palette,
                    &mut app.settings_open,
                    &mut app.recording,
                    Some(&mut app.chart_view_mode),
                );
                render_chart_panel(
                    ui,
                    &app.history,
                    &app.recording,
                    &palette,
                    &mut app.chart_view_mode,
                    350.0,
                    180.0,
                );
            });
        });

        // Discard and verify clean purge
        app.recording.discard();
        assert!(app.recording.is_idle());
        assert!(app.recording.is_empty());
    }

}

