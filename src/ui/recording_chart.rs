use egui::{pos2, vec2, Align2, FontId, Stroke, Ui};
use crate::telemetry::models::HistoryPoint;
use crate::telemetry::recording::{format_duration_short, PerformanceRecordingSession, RecordingPoint, RecordingState};
use crate::theme::ThemePalette;
use crate::ui::chart::clamp_chart_width;
use crate::ui::widgets::card_container;

/// View mode toggle between 60-second rolling live telemetry and performance recording graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartViewMode {
    #[default]
    Live,
    Recording,
}

impl ChartViewMode {
    pub fn is_recording(&self) -> bool {
        matches!(self, ChartViewMode::Recording)
    }

    #[allow(dead_code)]
    pub fn is_live(&self) -> bool {
        matches!(self, ChartViewMode::Live)
    }
}


/// Renders the retro vintage tooltip card displaying detailed telemetry metrics
/// for the inspected point.
pub fn render_recording_tooltip(ui: &mut Ui, pt: &RecordingPoint, palette: &ThemePalette) {
    egui::Frame::none()
        .fill(palette.card_bg)
        .stroke(Stroke::new(1.5_f32, palette.border))
        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
        .show(ui, |ui| {
            ui.set_max_width(280.0);

            // Header: Point inspection label & elapsed time
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("POINT INSPECT")
                        .font(FontId::monospace(10.5))
                        .strong()
                        .color(palette.bar_active),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("T +{}", pt.format_elapsed()))
                            .font(FontId::monospace(11.0))
                            .strong()
                            .color(palette.text_main),
                    );
                });
            });

            ui.add_space(3.0);

            // Subtle divider
            let (sep_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), egui::Sense::hover());
            ui.painter().rect_filled(sep_rect, 0.0, palette.bar_track);

            ui.add_space(4.0);

            // CPU Row (palette.chart_line1)
            ui.horizontal(|ui| {
                let (swatch, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(swatch, 0.0, palette.chart_line1);
                ui.label(
                    egui::RichText::new(format!("CPU {:>5.1}%", pt.cpu_pct))
                        .font(FontId::monospace(11.0))
                        .strong()
                        .color(palette.text_main),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}  {}", pt.format_cpu_temp(), pt.format_cpu_clock()))
                            .font(FontId::monospace(10.5))
                            .color(palette.text_muted),
                    );
                });
            });

            ui.add_space(3.0);

            // GPU Row (palette.chart_line2)
            ui.horizontal(|ui| {
                let (swatch, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(swatch, 0.0, palette.chart_line2);
                ui.label(
                    egui::RichText::new(format!("GPU {:>5.1}%", pt.gpu_pct))
                        .font(FontId::monospace(11.0))
                        .strong()
                        .color(palette.text_main),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}  {}", pt.format_gpu_temp(), pt.format_gpu_clock()))
                            .font(FontId::monospace(10.5))
                            .color(palette.text_muted),
                    );
                });
            });

            ui.add_space(3.0);

            // RAM Row (palette.positive)
            ui.horizontal(|ui| {
                let (swatch, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(swatch, 0.0, palette.positive);
                ui.label(
                    egui::RichText::new(format!("RAM {:>5.1}%", pt.ram_pct))
                        .font(FontId::monospace(11.0))
                        .strong()
                        .color(palette.text_main),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}  {}", pt.format_ram_usage(), pt.ram_speed))
                            .font(FontId::monospace(10.5))
                            .color(palette.text_muted),
                    );
                });
            });
        });
}

/// Draws the interactive performance recording graph plotting three curves:
/// - CPU: `palette.chart_line1`
/// - GPU: `palette.chart_line2`
/// - RAM: `palette.positive`
///
/// Features interactive mouse hover pinpointing the nearest recording point, drawing
/// a vertical time marker, and displaying a vintage telemetry tooltip card.
#[allow(dead_code)]
pub fn draw_recording_chart(
    ui: &mut Ui,
    session: &PerformanceRecordingSession,
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {

    draw_recording_chart_internal(ui, session, palette, None, width, height);
}

/// Draws the performance recording graph with optional mode tab switcher.
pub fn draw_recording_chart_with_tabs(
    ui: &mut Ui,
    session: &PerformanceRecordingSession,
    palette: &ThemePalette,
    mode: &mut ChartViewMode,
    width: f32,
    height: f32,
) {
    draw_recording_chart_internal(ui, session, palette, Some(mode), width, height);
}

fn draw_recording_chart_internal(
    ui: &mut Ui,
    session: &PerformanceRecordingSession,
    palette: &ThemePalette,
    mut mode: Option<&mut ChartViewMode>,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        let inner_w = ui.available_width();
        let inner_h = ui.available_height();

        // Header: Tab / Title on Left & 3-Curve Legend on Right
        ui.horizontal(|ui| {
            if let Some(ref mut m) = mode {
                // Tab switcher between Live and Recording
                let is_rec = m.is_recording();
                let live_bg = if !is_rec { palette.bar_active } else { palette.card_bg };
                let live_fg = if !is_rec { palette.background } else { palette.text_muted };
                let rec_bg = if is_rec { palette.bar_active } else { palette.card_bg };
                let rec_fg = if is_rec { palette.background } else { palette.text_main };

                let live_btn = egui::Button::new(
                    egui::RichText::new("LIVE")
                        .font(FontId::monospace(10.5))
                        .color(live_fg),
                )
                .fill(live_bg)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .min_size(vec2(44.0, 18.0));

                if ui.add(live_btn).clicked() {
                    **m = ChartViewMode::Live;
                }

                ui.add_space(2.0);

                let rec_label = match session.state() {
                    RecordingState::Recording => "● REC GRAPH",
                    _ => "REC GRAPH",
                };
                let rec_btn = egui::Button::new(
                    egui::RichText::new(rec_label)
                        .font(FontId::monospace(10.5))
                        .strong()
                        .color(rec_fg),
                )
                .fill(rec_bg)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .min_size(vec2(74.0, 18.0));

                if ui.add(rec_btn).clicked() {
                    **m = ChartViewMode::Recording;
                }
            } else {
                let title = match session.state() {
                    RecordingState::Recording => "REC // Recording...",
                    RecordingState::Stopped => "REC // Stopped Review",
                    RecordingState::Idle => "REC // Performance Graph",
                };
                ui.label(
                    egui::RichText::new(title)
                        .font(FontId::monospace(11.5))
                        .strong()
                        .color(palette.text_main),
                );
            }

            ui.add_space(6.0);

            // Session duration / points badge
            if !session.is_idle() {
                let dur_str = format_duration_short(session.duration_secs());
                let pts_cnt = session.sample_count();
                ui.label(
                    egui::RichText::new(format!("{} ({}p)", dur_str, pts_cnt))
                        .font(FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
            }

            // Legend on the Right: RAM (green) | GPU (orange) | CPU (black)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // RAM legend
                ui.label(
                    egui::RichText::new("RAM")
                        .font(FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
                let (ram_rect, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                ui.painter().rect_filled(ram_rect, 0.0, palette.positive);

                ui.add_space(6.0);

                // GPU legend
                ui.label(
                    egui::RichText::new("GPU")
                        .font(FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
                let (gpu_rect, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                ui.painter().rect_filled(gpu_rect, 0.0, palette.chart_line2);

                ui.add_space(6.0);

                // CPU legend
                ui.label(
                    egui::RichText::new("CPU")
                        .font(FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
                let (cpu_rect, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                ui.painter().rect_filled(cpu_rect, 0.0, palette.chart_line1);
            });
        });

        ui.add_space(4.0);

        // Allocate chart painter
        let chart_height = (inner_h - 48.0).max(30.0);
        let chart_w = clamp_chart_width(inner_w);
        let (response, painter) = ui.allocate_painter(vec2(chart_w, chart_height), egui::Sense::hover());
        let rect = response.rect;

        // Visible horizontal grid lines (25%, 50%, 75%)
        let grid_stroke = Stroke::new(1.0_f32, palette.bar_track);
        for i in 1..=3 {
            let y = rect.min.y + (rect.height() * (i as f32 / 4.0));
            painter.line_segment([pos2(rect.min.x, y), pos2(rect.max.x, y)], grid_stroke);
        }

        let samples = session.samples();

        // Empty state placeholder
        if samples.is_empty() {
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                "IDLE // PRESS [START RECORDING] TO CAPTURE CPU / GPU / RAM",
                FontId::monospace(11.0),
                palette.text_muted,
            );
        } else if samples.len() == 1 {
            let pt = &samples[0];
            let x = rect.center().x;
            let chart_h = rect.height() - 8.0;

            let cpu_norm = (pt.cpu_pct / 100.0).clamp(0.0, 1.0);
            let gpu_norm = (pt.gpu_pct / 100.0).clamp(0.0, 1.0);
            let ram_norm = (pt.ram_pct / 100.0).clamp(0.0, 1.0);

            let cpu_y = rect.max.y - 4.0 - (cpu_norm * chart_h);
            let gpu_y = rect.max.y - 4.0 - (gpu_norm * chart_h);
            let ram_y = rect.max.y - 4.0 - (ram_norm * chart_h);

            painter.circle_filled(pos2(x, ram_y), 4.0, palette.positive);
            painter.circle_filled(pos2(x, gpu_y), 4.0, palette.chart_line2);
            painter.circle_filled(pos2(x, cpu_y), 4.0, palette.chart_line1);
        } else {
            // Plot 3 distinct curves
            let t_min = samples.first().map(|p| p.elapsed_secs).unwrap_or(0.0);
            let t_max = samples.last().map(|p| p.elapsed_secs).unwrap_or(0.0);
            let duration = (t_max - t_min).max(0.0001);
            let chart_h = rect.height() - 8.0;

            let mut cpu_pts = Vec::with_capacity(samples.len());
            let mut gpu_pts = Vec::with_capacity(samples.len());
            let mut ram_pts = Vec::with_capacity(samples.len());

            for pt in samples {
                let frac_x = ((pt.elapsed_secs - t_min) / duration).clamp(0.0, 1.0) as f32;
                let x = (rect.min.x + frac_x * rect.width()).clamp(rect.min.x, rect.max.x);

                let cpu_norm = (pt.cpu_pct / 100.0).clamp(0.0, 1.0);
                let gpu_norm = (pt.gpu_pct / 100.0).clamp(0.0, 1.0);
                let ram_norm = (pt.ram_pct / 100.0).clamp(0.0, 1.0);

                let cpu_y = rect.max.y - 4.0 - (cpu_norm * chart_h);
                let gpu_y = rect.max.y - 4.0 - (gpu_norm * chart_h);
                let ram_y = rect.max.y - 4.0 - (ram_norm * chart_h);

                cpu_pts.push(pos2(x, cpu_y));
                gpu_pts.push(pos2(x, gpu_y));
                ram_pts.push(pos2(x, ram_y));
            }

            // Draw RAM curve (palette.positive)
            let ram_stroke = Stroke::new(1.8_f32, palette.positive);
            for w in ram_pts.windows(2) {
                painter.line_segment([w[0], w[1]], ram_stroke);
            }

            // Draw GPU curve (palette.chart_line2)
            let gpu_stroke = Stroke::new(2.0_f32, palette.chart_line2);
            for w in gpu_pts.windows(2) {
                painter.line_segment([w[0], w[1]], gpu_stroke);
            }

            // Draw CPU curve (palette.chart_line1)
            let cpu_stroke = Stroke::new(2.0_f32, palette.chart_line1);
            for w in cpu_pts.windows(2) {
                painter.line_segment([w[0], w[1]], cpu_stroke);
            }

            // Interactive mouse hover inspection
            if let Some(mouse_pos) = response.hover_pos() {
                if rect.contains(mouse_pos) {
                    let hover_frac = ((mouse_pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0) as f64;
                    let hover_time = t_min + (hover_frac * duration);

                    if let Some(nearest_pt) = session.find_nearest_point(hover_time) {
                        let marker_frac = ((nearest_pt.elapsed_secs - t_min) / duration).clamp(0.0, 1.0) as f32;
                        let marker_x = (rect.min.x + marker_frac * rect.width()).clamp(rect.min.x, rect.max.x);

                        // Draw vertical time marker line
                        let marker_stroke = Stroke::new(1.2_f32, palette.bar_active);
                        painter.line_segment([pos2(marker_x, rect.min.y), pos2(marker_x, rect.max.y)], marker_stroke);

                        // Draw indicator circles on the 3 curves at target point
                        let pt_cpu_norm = (nearest_pt.cpu_pct / 100.0).clamp(0.0, 1.0);
                        let pt_gpu_norm = (nearest_pt.gpu_pct / 100.0).clamp(0.0, 1.0);
                        let pt_ram_norm = (nearest_pt.ram_pct / 100.0).clamp(0.0, 1.0);

                        let pt_cpu_y = rect.max.y - 4.0 - (pt_cpu_norm * chart_h);
                        let pt_gpu_y = rect.max.y - 4.0 - (pt_gpu_norm * chart_h);
                        let pt_ram_y = rect.max.y - 4.0 - (pt_ram_norm * chart_h);

                        // Highlight dots
                        painter.circle_filled(pos2(marker_x, pt_ram_y), 3.5, palette.positive);
                        painter.circle_stroke(pos2(marker_x, pt_ram_y), 4.5, Stroke::new(1.0_f32, palette.background));

                        painter.circle_filled(pos2(marker_x, pt_gpu_y), 3.5, palette.chart_line2);
                        painter.circle_stroke(pos2(marker_x, pt_gpu_y), 4.5, Stroke::new(1.0_f32, palette.background));

                        painter.circle_filled(pos2(marker_x, pt_cpu_y), 3.5, palette.chart_line1);
                        painter.circle_stroke(pos2(marker_x, pt_cpu_y), 4.5, Stroke::new(1.0_f32, palette.background));

                        // Marker time pill at the top of the line
                        let time_label = format_duration_short(nearest_pt.elapsed_secs);
                        let text_pos = pos2((marker_x + 3.0).min(rect.max.x - 30.0), rect.min.y + 2.0);
                        painter.text(
                            text_pos,
                            Align2::LEFT_TOP,
                            time_label,
                            FontId::monospace(9.5),
                            palette.bar_active,
                        );

                        // Show retro vintage tooltip card
                        let pt_clone = nearest_pt.clone();
                        let pal = palette.clone();
                        response.on_hover_ui_at_pointer(move |ui| {
                            render_recording_tooltip(ui, &pt_clone, &pal);
                        });
                    }
                }
            }
        }

        ui.add_space(4.0);

        // Bottom time axis labels
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let font = FontId::monospace(10.0);
            let row_w = ui.available_width();
            let total_dur = session.duration_secs();

            let t0_str = "00:00";
            let t_end_str = format_duration_short(total_dur);

            if total_dur > 0.0 {
                let step = (row_w - 70.0) / 4.0;
                let t1_str = format_duration_short(total_dur * 0.25);
                let t2_str = format_duration_short(total_dur * 0.50);
                let t3_str = format_duration_short(total_dur * 0.75);

                ui.label(egui::RichText::new(t0_str).font(font.clone()).color(palette.text_muted));
                ui.add_space((step - 16.0).max(2.0));
                ui.label(egui::RichText::new(t1_str).font(font.clone()).color(palette.text_muted));
                ui.add_space((step - 16.0).max(2.0));
                ui.label(egui::RichText::new(t2_str).font(font.clone()).color(palette.text_muted));
                ui.add_space((step - 16.0).max(2.0));
                ui.label(egui::RichText::new(t3_str).font(font.clone()).color(palette.text_muted));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(t_end_str).font(font).color(palette.text_muted));
                });
            } else {
                ui.label(egui::RichText::new(t0_str).font(font.clone()).color(palette.text_muted));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(t0_str).font(font).color(palette.text_muted));
                });
            }
        });
    });
}

/// Draws the 60-second live rolling telemetry chart with the mode tab bar.
pub fn draw_live_chart_with_tabs(
    ui: &mut Ui,
    history: &[HistoryPoint],
    session: &PerformanceRecordingSession,
    palette: &ThemePalette,
    mode: &mut ChartViewMode,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        let inner_w = ui.available_width();
        let inner_h = ui.available_height();

        // Header: Tabs on Left, Legend on Right
        ui.horizontal(|ui| {
            let is_rec = mode.is_recording();
            let live_bg = if !is_rec { palette.bar_active } else { palette.card_bg };
            let live_fg = if !is_rec { palette.background } else { palette.text_muted };
            let rec_bg = if is_rec { palette.bar_active } else { palette.card_bg };
            let rec_fg = if is_rec { palette.background } else { palette.text_main };

            let live_btn = egui::Button::new(
                egui::RichText::new("LIVE")
                    .font(FontId::monospace(10.5))
                    .strong()
                    .color(live_fg),
            )
            .fill(live_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(44.0, 18.0));

            if ui.add(live_btn).clicked() {
                *mode = ChartViewMode::Live;
            }

            ui.add_space(2.0);

            let rec_label = match session.state() {
                RecordingState::Recording => "● REC GRAPH",
                RecordingState::Stopped => "REC GRAPH (●)",
                RecordingState::Idle => "REC GRAPH",
            };
            let rec_btn = egui::Button::new(
                egui::RichText::new(rec_label)
                    .font(FontId::monospace(10.5))
                    .color(rec_fg),
            )
            .fill(rec_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(74.0, 18.0));

            if ui.add(rec_btn).clicked() {
                *mode = ChartViewMode::Recording;
            }

            ui.add_space(6.0);

            ui.label(
                egui::RichText::new("Last 60s")
                    .font(FontId::monospace(10.0))
                    .color(palette.text_muted),
            );

            // Legend on Right: GPU | CPU
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("GPU")
                        .font(FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
                let (gpu_rect, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                ui.painter().rect_filled(gpu_rect, 0.0, palette.chart_line2);

                ui.add_space(6.0);

                ui.label(
                    egui::RichText::new("CPU")
                        .font(FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
                let (cpu_rect, _) = ui.allocate_exact_size(vec2(7.0, 7.0), egui::Sense::hover());
                ui.painter().rect_filled(cpu_rect, 0.0, palette.chart_line1);
            });
        });

        ui.add_space(4.0);

        // Painter
        let chart_height = (inner_h - 48.0).max(30.0);
        let chart_w = clamp_chart_width(inner_w);
        let (response, painter) = ui.allocate_painter(vec2(chart_w, chart_height), egui::Sense::hover());
        let rect = response.rect;

        // Grid lines
        let grid_stroke = Stroke::new(1.0_f32, palette.bar_track);
        for i in 1..=3 {
            let y = rect.min.y + (rect.height() * (i as f32 / 4.0));
            painter.line_segment([pos2(rect.min.x, y), pos2(rect.max.x, y)], grid_stroke);
        }

        // Draw curves
        if history.len() >= 2 {
            let max_points = 60;
            let start_idx = history.len().saturating_sub(max_points);
            let slice = &history[start_idx..];

            let mut cpu_pts = Vec::new();
            let mut gpu_pts = Vec::new();

            let x_step = rect.width() / (max_points - 1) as f32;
            let offset_x = (max_points - slice.len()) as f32 * x_step;

            for (i, pt) in slice.iter().enumerate() {
                let x = (rect.min.x + offset_x + (i as f32 * x_step)).clamp(rect.min.x, rect.max.x);
                let cpu_norm = (pt.cpu_pct / 100.0).clamp(0.0, 1.0);
                let gpu_norm = (pt.gpu_pct / 100.0).clamp(0.0, 1.0);

                let cpu_y = rect.max.y - (cpu_norm * (rect.height() - 8.0)) - 4.0;
                let gpu_y = rect.max.y - (gpu_norm * (rect.height() - 8.0)) - 4.0;

                cpu_pts.push(pos2(x, cpu_y));
                gpu_pts.push(pos2(x, gpu_y));
            }

            let gpu_stroke = Stroke::new(2.0_f32, palette.chart_line2);
            for w in gpu_pts.windows(2) {
                painter.line_segment([w[0], w[1]], gpu_stroke);
            }

            let cpu_stroke = Stroke::new(2.0_f32, palette.chart_line1);
            for w in cpu_pts.windows(2) {
                painter.line_segment([w[0], w[1]], cpu_stroke);
            }
        }

        ui.add_space(4.0);

        // Bottom labels
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let font = FontId::monospace(10.0);
            let row_w = ui.available_width();
            let step = (row_w - 70.0) / 4.0;

            ui.label(egui::RichText::new("60s ago").font(font.clone()).color(palette.text_muted));
            ui.add_space((step - 16.0).max(2.0));
            ui.label(egui::RichText::new("45s").font(font.clone()).color(palette.text_muted));
            ui.add_space((step - 16.0).max(2.0));
            ui.label(egui::RichText::new("30s").font(font.clone()).color(palette.text_muted));
            ui.add_space((step - 16.0).max(2.0));
            ui.label(egui::RichText::new("15s").font(font.clone()).color(palette.text_muted));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("Now").font(font).color(palette.text_muted));
            });
        });
    });
}

/// Unified chart panel that switches between Live History and Recording Graph.
pub fn render_chart_panel(
    ui: &mut Ui,
    history: &[HistoryPoint],
    session: &PerformanceRecordingSession,
    palette: &ThemePalette,
    mode: &mut ChartViewMode,
    width: f32,
    height: f32,
) {
    match mode {
        ChartViewMode::Live => {
            draw_live_chart_with_tabs(ui, history, session, palette, mode, width, height);
        }
        ChartViewMode::Recording => {
            draw_recording_chart_with_tabs(ui, session, palette, mode, width, height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::models::{CpuSnapshot, GpuSnapshot, MemorySnapshot, SystemSnapshot};

    fn make_test_snapshot(cpu: f32, gpu: f32, ram: f32, epoch_ms: i64) -> SystemSnapshot {
        let mut snap = SystemSnapshot::default();
        snap.timestamp_epoch_ms = epoch_ms;
        snap.cpu = CpuSnapshot {
            utilization_pct: cpu,
            package_temp_c: Some(62.0),
            clock_speed_ghz: 4.20,
            ..Default::default()
        };
        snap.gpu = GpuSnapshot {
            utilization_pct: gpu,
            core_temp_c: Some(71.0),
            core_clock_mhz: Some(2150.0),
            ..Default::default()
        };
        snap.memory = MemorySnapshot {
            utilization_pct: ram,
            used_gb: 14.2,
            total_gb: 31.9,
            memory_speed: "5600 MT/s".into(),
            ..Default::default()
        };
        snap
    }

    #[test]
    fn test_chart_view_mode_defaults_and_helpers() {
        let mode = ChartViewMode::default();
        assert_eq!(mode, ChartViewMode::Live);
        assert!(mode.is_live());
        assert!(!mode.is_recording());

        let rec_mode = ChartViewMode::Recording;
        assert!(rec_mode.is_recording());
        assert!(!rec_mode.is_live());
    }

    #[test]
    fn test_render_recording_tooltip() {
        let palette = ThemePalette::vintage();
        let pt = RecordingPoint {
            timestamp_epoch_ms: 1700000000000,
            elapsed_secs: 14.25,
            cpu_pct: 45.2,
            cpu_temp_c: Some(65.0),
            cpu_clock_ghz: 4.25,
            gpu_pct: 78.5,
            gpu_temp_c: Some(72.0),
            gpu_clock_mhz: Some(2100.0),
            ram_pct: 44.5,
            ram_used_gb: 14.2,
            ram_total_gb: 31.9,
            ram_speed: "5600 MT/s".into(),
        };

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_recording_tooltip(ui, &pt, &palette);
            });
        });
    }

    #[test]
    fn test_draw_recording_chart_idle_empty() {
        let palette = ThemePalette::vintage();
        let session = PerformanceRecordingSession::new();
        assert!(session.is_empty());

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_recording_chart(ui, &session, &palette, 350.0, 180.0);
            });
        });
    }

    #[test]
    fn test_draw_recording_chart_single_sample() {
        let palette = ThemePalette::vintage();
        let mut session = PerformanceRecordingSession::new();
        session.start();
        session.add_sample(&make_test_snapshot(30.0, 50.0, 40.0, 1000));
        assert_eq!(session.sample_count(), 1);

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_recording_chart(ui, &session, &palette, 350.0, 180.0);
            });
        });
    }

    #[test]
    fn test_draw_recording_chart_multiple_samples_and_tabs() {
        let palette = ThemePalette::vintage();
        let mut session = PerformanceRecordingSession::new();
        session.start();
        session.add_sample(&make_test_snapshot(20.0, 30.0, 40.0, 1000));
        session.add_sample(&make_test_snapshot(55.0, 75.0, 45.0, 2000));
        session.add_sample(&make_test_snapshot(85.0, 90.0, 50.0, 3000));
        session.stop();

        assert_eq!(session.sample_count(), 3);
        assert!(session.is_stopped());

        let mut mode = ChartViewMode::Recording;
        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_recording_chart_with_tabs(ui, &session, &palette, &mut mode, 400.0, 200.0);
            });
        });
    }

    #[test]
    fn test_draw_live_chart_with_tabs_and_render_chart_panel() {
        let palette = ThemePalette::vintage();
        let session = PerformanceRecordingSession::new();
        let history = vec![
            HistoryPoint { timestamp_secs: 1.0, cpu_pct: 25.0, gpu_pct: 40.0 },
            HistoryPoint { timestamp_secs: 2.0, cpu_pct: 35.0, gpu_pct: 50.0 },
        ];
        let mut mode = ChartViewMode::Live;

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_chart_panel(ui, &history, &session, &palette, &mut mode, 400.0, 200.0);
            });
        });
    }

    #[test]
    fn test_discard_purges_samples_and_memory_immediately() {
        let mut session = PerformanceRecordingSession::new();
        session.start();
        for i in 0..100 {
            session.add_sample(&make_test_snapshot(20.0 + (i as f32 % 50.0), 30.0, 40.0, 1000 + i * 500));
        }
        assert_eq!(session.sample_count(), 100);

        // Immediate discard
        session.discard();
        assert!(session.is_empty());
        assert_eq!(session.sample_count(), 0);
        assert_eq!(session.state(), RecordingState::Idle);
    }
}
