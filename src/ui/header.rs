use egui::{vec2, Stroke, Ui};
use crate::telemetry::models::SystemInfoSnapshot;
use crate::telemetry::recording::{format_duration, PerformanceRecordingSession, RecordingState};
use crate::theme::ThemePalette;
use crate::ui::recording_chart::ChartViewMode;

/// Renders the performance recording controls matching the Vintage industrial theme:
/// Start Recording / REC indicator / Stop Recording / Discard & Done.
pub fn render_recording_controls(
    ui: &mut Ui,
    recording: &mut PerformanceRecordingSession,
    mut chart_mode: Option<&mut ChartViewMode>,
    palette: &ThemePalette,
) {
    match recording.state() {
        RecordingState::Idle => {
            // Idle state: "START RECORDING" button
            let start_btn = egui::Button::new(
                egui::RichText::new("⏺ START RECORDING")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            )
            .fill(palette.card_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(130.0, 22.0));

            if ui
                .add(start_btn)
                .on_hover_text("Start telemetry performance recording session")
                .clicked()
            {
                recording.start();
                if let Some(ref mut m) = chart_mode {
                    **m = ChartViewMode::Recording;
                }
            }
        }
        RecordingState::Recording => {
            // Active Recording state: REC indicator badge + STOP + DISCARD
            let duration_str = format_duration(recording.live_duration_secs());
            let sample_cnt = recording.sample_count();

            // Distinct Vintage REC indicator badge
            let rec_frame = egui::Frame::none()
                .fill(palette.bar_active)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .inner_margin(egui::Margin::symmetric(6.0, 3.0));

            rec_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("● REC")
                            .font(egui::FontId::monospace(11.0))
                            .strong()
                            .color(palette.background),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!("{} ({}p)", duration_str, sample_cnt))
                            .font(egui::FontId::monospace(11.0))
                            .color(palette.background),
                    );
                });
            });

            ui.add_space(6.0);

            // STOP button
            let stop_btn = egui::Button::new(
                egui::RichText::new("⏹ STOP")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            )
            .fill(palette.card_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(60.0, 22.0));

            if ui
                .add(stop_btn)
                .on_hover_text("Stop recording and freeze telemetry data for inspection")
                .clicked()
            {
                recording.stop();
            }

            ui.add_space(6.0);

            // DISCARD button
            let discard_btn = egui::Button::new(
                egui::RichText::new("🗑 DISCARD")
                    .font(egui::FontId::monospace(11.0))
                    .color(palette.critical),
            )
            .fill(palette.card_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(72.0, 22.0));

            if ui
                .add(discard_btn)
                .on_hover_text("Purge and free all recorded data immediately")
                .clicked()
            {
                recording.discard();
                if let Some(ref mut m) = chart_mode {
                    **m = ChartViewMode::Live;
                }
            }
        }
        RecordingState::Stopped => {
            // Stopped state: RECORDED badge + NEW REC + DISCARD & DONE
            let duration_str = format_duration(recording.duration_secs());
            let sample_cnt = recording.sample_count();

            let badge_frame = egui::Frame::none()
                .fill(palette.bar_track)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .inner_margin(egui::Margin::symmetric(6.0, 3.0));

            badge_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("RECORDED")
                            .font(egui::FontId::monospace(10.5))
                            .strong()
                            .color(palette.text_main),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!("{} ({}p)", duration_str, sample_cnt))
                            .font(egui::FontId::monospace(10.5))
                            .color(palette.text_muted),
                    );
                });
            });

            ui.add_space(6.0);

            // NEW REC button
            let new_btn = egui::Button::new(
                egui::RichText::new("⏺ NEW REC")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            )
            .fill(palette.card_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(76.0, 22.0));

            if ui
                .add(new_btn)
                .on_hover_text("Start a fresh recording session")
                .clicked()
            {
                recording.start();
                if let Some(ref mut m) = chart_mode {
                    **m = ChartViewMode::Recording;
                }
            }

            ui.add_space(6.0);

            // DISCARD & DONE button
            let discard_done_btn = egui::Button::new(
                egui::RichText::new("🗑 DISCARD & DONE")
                    .font(egui::FontId::monospace(11.0))
                    .color(palette.critical),
            )
            .fill(palette.card_bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(120.0, 22.0));

            if ui
                .add(discard_done_btn)
                .on_hover_text("Purge recorded data immediately and return to Idle")
                .clicked()
            {
                recording.discard();
                if let Some(ref mut m) = chart_mode {
                    **m = ChartViewMode::Live;
                }
            }
        }
    }
}

/// Renders a dedicated standalone performance recording toolbar matching the Vintage theme.
#[allow(dead_code)]
pub fn render_recording_toolbar(

    ui: &mut Ui,
    recording: &mut PerformanceRecordingSession,
    chart_mode: Option<&mut ChartViewMode>,
    palette: &ThemePalette,
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("RECORDER:")
                .font(egui::FontId::monospace(11.0))
                .strong()
                .color(palette.text_muted),
        );
        ui.add_space(8.0);
        render_recording_controls(ui, recording, chart_mode, palette);
    });
}

pub fn render_header(
    ui: &mut Ui,
    info: &SystemInfoSnapshot,
    palette: &ThemePalette,
    settings_open: &mut bool,
    recording: &mut PerformanceRecordingSession,
    chart_mode: Option<&mut ChartViewMode>,
) {
    ui.horizontal(|ui| {
        // Branding on Left
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("NEXUS")
                    .font(egui::FontId::monospace(18.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("PC Control Center")
                    .font(egui::FontId::monospace(12.0))
                    .color(palette.text_muted),
            );
        });

        ui.add_space(16.0);

        // Recording Controls in Header
        render_recording_controls(ui, recording, chart_mode, palette);

        // System status and Settings Button on Right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Settings Cog Button on far right
            let is_open = *settings_open;
            let bg = if is_open { palette.bar_active } else { palette.card_bg };
            let fg = if is_open { palette.background } else { palette.text_main };

            let cog_btn = egui::Button::new(
                egui::RichText::new("⚙")
                    .font(egui::FontId::monospace(13.0))
                    .color(fg),
            )
            .fill(bg)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .min_size(vec2(26.0, 22.0));

            if ui.add(cog_btn).on_hover_text("Settings (⚙)").clicked() {
                *settings_open = !*settings_open;
            }

            ui.add_space(16.0);

            // Power Profile
            ui.label(
                egui::RichText::new(&info.power_profile)
                    .font(egui::FontId::monospace(11.0))
                    .color(palette.text_muted),
            );

            ui.add_space(12.0);

            // Boot time
            ui.label(
                egui::RichText::new(&info.boot_time_str)
                    .font(egui::FontId::monospace(11.0))
                    .color(palette.text_muted),
            );

            ui.add_space(12.0);

            // Uptime
            ui.label(
                egui::RichText::new(&info.uptime_str)
                    .font(egui::FontId::monospace(11.0))
                    .color(palette.text_muted),
            );

            ui.add_space(12.0);

            // Status indicator
            ui.label(
                egui::RichText::new(&info.status)
                    .font(egui::FontId::monospace(11.0))
                    .color(palette.text_muted),
            );
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_header_toggle_cog() {
        let palette = ThemePalette::vintage();
        let info = SystemInfoSnapshot {
            status: "OPTIMAL".into(),
            uptime_str: "1h 23m".into(),
            boot_time_str: "09:00:00".into(),
            power_profile: "BALANCED".into(),
        };
        let mut settings_open = false;
        let mut recording = PerformanceRecordingSession::new();
        let mut chart_mode = ChartViewMode::Live;

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(ui, &info, &palette, &mut settings_open, &mut recording, Some(&mut chart_mode));
            });
        });
        assert!(!settings_open);
        assert!(recording.is_idle());
        assert_eq!(chart_mode, ChartViewMode::Live);
    }

    #[test]
    fn test_render_header_recording_states_ui() {
        let palette = ThemePalette::vintage();
        let info = SystemInfoSnapshot::default();
        let mut settings_open = false;
        let mut recording = PerformanceRecordingSession::new();
        let mut chart_mode = ChartViewMode::Live;

        let ctx = egui::Context::default();

        // 1. Idle state rendering
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(ui, &info, &palette, &mut settings_open, &mut recording, Some(&mut chart_mode));
            });
        });
        assert!(recording.is_idle());

        // 2. Active recording state rendering
        recording.start();
        assert!(recording.is_recording());
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(ui, &info, &palette, &mut settings_open, &mut recording, Some(&mut chart_mode));
            });
        });

        // 3. Stopped state rendering
        recording.stop();
        assert!(recording.is_stopped());
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(ui, &info, &palette, &mut settings_open, &mut recording, Some(&mut chart_mode));
            });
        });

        // 4. Discard purges data immediately
        recording.discard();
        assert!(recording.is_idle());
        assert!(recording.is_empty());
    }

    #[test]
    fn test_render_recording_toolbar_standalone() {
        let palette = ThemePalette::vintage();
        let mut recording = PerformanceRecordingSession::new();
        let mut chart_mode = ChartViewMode::Recording;

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_recording_toolbar(ui, &mut recording, Some(&mut chart_mode), &palette);
            });
        });
    }
}
