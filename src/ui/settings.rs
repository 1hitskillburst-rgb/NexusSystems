use egui::{vec2, Align2, Context, Frame, RichText, Stroke, Ui, Window};
use crate::config::AppConfig;
use crate::theme::ThemePalette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsResponse {
    pub changed: bool,
    pub reset_requested: bool,
}

pub fn render_settings_window(
    ctx: &Context,
    open: &mut bool,
    config: &mut AppConfig,
    palette: &ThemePalette,
) -> SettingsResponse {
    let mut response = SettingsResponse {
        changed: false,
        reset_requested: false,
    };

    if !*open {
        return response;
    }

    let mut close_requested = false;

    Window::new("⚙ SETTINGS")
        .open(open)
        .anchor(Align2::CENTER_CENTER, [0.0_f32, 0.0_f32])
        .resizable(false)
        .collapsible(false)
        .default_width(380.0)
        .frame(
            Frame::window(&ctx.style())
                .fill(palette.card_bg)
                .stroke(Stroke::new(1.5_f32, palette.border))
                .inner_margin(16.0),
        )
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 10.0);

            // Subtitle / description
            ui.label(
                RichText::new("NEXUS SYSTEM PREFERENCES")
                    .font(egui::FontId::monospace(10.0))
                    .color(palette.text_muted),
            );
            ui.separator();

            // Section 1: Telemetry Refresh Interval
            ui.add_space(4.0);
            ui.label(
                RichText::new("TELEMETRY REFRESH RATE")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            );

            ui.horizontal(|ui| {
                let intervals = [(500, "500ms (Fast)"), (1000, "1s (Normal)"), (2000, "2s (Eco)")];
                for (val, label) in intervals {
                    let is_selected = config.refresh_interval_ms == val;
                    let fill = if is_selected { palette.bar_active } else { palette.background };
                    let text_color = if is_selected { palette.background } else { palette.text_main };

                    let btn = egui::Button::new(
                        RichText::new(label)
                            .font(egui::FontId::monospace(10.0))
                            .color(text_color)
                            .strong(),
                    )
                    .fill(fill)
                    .stroke(Stroke::new(1.0_f32, palette.border))
                    .min_size(vec2(100.0, 24.0));

                    if ui.add(btn).clicked() && !is_selected {
                        config.refresh_interval_ms = val;
                        response.changed = true;
                    }
                }
            });

            // Section 2: Secondary Cards Visibility
            ui.add_space(8.0);
            ui.label(
                RichText::new("SECONDARY DASHBOARD CARDS")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            );

            // Checkbox rows with clean retro-industrial styling
            ui.vertical(|ui| {
                if custom_toggle(ui, &mut config.show_storage, "Storage Volumes (Drives)", palette) {
                    response.changed = true;
                }
                if custom_toggle(ui, &mut config.show_processes, "Top Processes (CPU/RAM)", palette) {
                    response.changed = true;
                }
                if custom_toggle(ui, &mut config.show_network, "Network Throughput (DL/UL)", palette) {
                    response.changed = true;
                }
                if custom_toggle(ui, &mut config.show_cooling, "Cooling & Fan Instrumentation", palette) {
                    response.changed = true;
                }
                if custom_toggle(ui, &mut config.auto_hide_empty_cards, "Auto-Collapse Unpopulated Cards", palette) {
                    response.changed = true;
                }
            });

            // Section 3: Active Theme
            ui.add_space(8.0);
            ui.label(
                RichText::new("VISUAL THEME")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.horizontal(|ui| {
                let badge = egui::Button::new(
                    RichText::new("● Vintage (Retro-Industrial Ivory & Orange)")
                        .font(egui::FontId::monospace(10.0))
                        .color(palette.bar_active)
                        .strong(),
                )
                .fill(palette.background)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .min_size(vec2(ui.available_width(), 22.0));
                ui.add_enabled(false, badge);
            });

            // Bottom action row: Reset Defaults & Close
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                // Reset to Defaults on left
                let reset_btn = egui::Button::new(
                    RichText::new("↺ Reset to Defaults")
                        .font(egui::FontId::monospace(10.0))
                        .color(palette.critical),
                )
                .fill(palette.background)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .min_size(vec2(130.0, 24.0));

                if ui.add(reset_btn).clicked() {
                    config.reset_to_defaults();
                    response.changed = true;
                    response.reset_requested = true;
                }

                // Done on right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let done_btn = egui::Button::new(
                        RichText::new("Done")
                            .font(egui::FontId::monospace(11.0))
                            .color(palette.background)
                            .strong(),
                    )
                    .fill(palette.bar_active)
                    .stroke(Stroke::new(1.0_f32, palette.border))
                    .min_size(vec2(70.0, 24.0));

                    if ui.add(done_btn).clicked() {
                        close_requested = true;
                    }
                });
            });
        });

    if close_requested {
        *open = false;
    }

    response
}

fn custom_toggle(ui: &mut Ui, value: &mut bool, label: &str, palette: &ThemePalette) -> bool {
    let mut changed = false;
    let initial_val = *value;

    ui.horizontal(|ui| {
        // Toggle pill button [ON] / [OFF]
        let pill_text = if *value { "ON" } else { "OFF" };
        let pill_bg = if *value { palette.bar_active } else { palette.bar_track };
        let pill_fg = if *value { palette.background } else { palette.text_muted };

        let pill_btn = egui::Button::new(
            RichText::new(pill_text)
                .font(egui::FontId::monospace(9.0))
                .color(pill_fg)
                .strong(),
        )
        .fill(pill_bg)
        .stroke(Stroke::new(1.0_f32, palette.border))
        .min_size(vec2(36.0, 18.0));

        if ui.add(pill_btn).clicked() {
            *value = !*value;
            changed = true;
        }

        let text_btn = ui.selectable_label(
            *value,
            RichText::new(label)
                .font(egui::FontId::monospace(10.5))
                .color(palette.text_main),
        );
        if text_btn.clicked() {
            *value = !*value;
            changed = true;
        }
    });

    changed || (*value != initial_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_toggle_mutation() {
        let mut flag = true;
        let palette = ThemePalette::vintage();
        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let changed = custom_toggle(ui, &mut flag, "Test Toggle", &palette);
                assert!(!changed);
                assert!(flag);
            });
        });
    }

    #[test]
    fn test_settings_window_modal_flow() {
        let mut config = AppConfig::default();
        let palette = ThemePalette::vintage();
        let ctx = egui::Context::default();
        let mut open = false;

        let resp = render_settings_window(&ctx, &mut open, &mut config, &palette);
        assert!(!resp.changed);
        assert!(!resp.reset_requested);

        open = true;
        let _ = ctx.run(Default::default(), |ctx| {
            let resp = render_settings_window(ctx, &mut open, &mut config, &palette);
            assert!(!resp.changed);
        });
    }
}
