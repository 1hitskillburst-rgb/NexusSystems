use egui::{pos2, vec2, Align2, Context, Frame, RichText, Stroke, Ui, Window};
use crate::config::AppConfig;
use crate::theme::ThemePalette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsResponse {
    pub changed: bool,
    pub reset_requested: bool,
}

fn render_themed_separator(ui: &mut Ui, palette: &ThemePalette) {
    let w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(w, 1.0), egui::Sense::hover());
    ui.painter().line_segment(
        [pos2(rect.min.x, rect.center().y), pos2(rect.max.x, rect.center().y)],
        Stroke::new(1.0_f32, palette.bar_track),
    );
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

    // Close on Escape key press
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        close_requested = true;
    }

    // Subtle dark scrim over central canvas to enhance focus and vintage depth
    let screen_rect = ctx.screen_rect();
    ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("settings_scrim")))
        .rect_filled(screen_rect, 0.0, egui::Color32::from_rgba_unmultiplied(20, 20, 20, 80));

    Window::new("nexus_settings_window")
        .open(open)
        .anchor(Align2::CENTER_CENTER, [0.0_f32, 0.0_f32])
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .default_width(400.0)
        .frame(
            Frame::none()
                .fill(palette.card_bg)
                .stroke(Stroke::new(1.5_f32, palette.border))
                .inner_margin(egui::Margin::symmetric(18.0, 16.0))
                .rounding(0.0),
        )
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 10.0);

            // Custom Retro-Industrial Title Bar
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("⚙ SETTINGS")
                        .font(egui::FontId::monospace(13.0))
                        .strong()
                        .color(palette.text_main),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new("// PREFERENCES")
                        .font(egui::FontId::monospace(10.0))
                        .color(palette.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let close_btn = egui::Button::new(
                        RichText::new("✕")
                            .font(egui::FontId::monospace(11.0))
                            .strong()
                            .color(palette.text_main),
                    )
                    .fill(palette.card_bg)
                    .stroke(Stroke::new(1.0_f32, palette.border))
                    .min_size(vec2(22.0, 20.0))
                    .rounding(0.0);

                    if ui.add(close_btn).on_hover_text("Close Settings (Esc)").clicked() {
                        close_requested = true;
                    }
                });
            });

            render_themed_separator(ui, palette);

            // Section 1: Telemetry Refresh Interval
            ui.add_space(2.0);
            ui.label(
                RichText::new("TELEMETRY REFRESH RATE")
                    .font(egui::FontId::monospace(10.5))
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
                    .min_size(vec2(104.0, 24.0))
                    .rounding(0.0);

                    if ui.add(btn).clicked() && !is_selected {
                        config.refresh_interval_ms = val;
                        response.changed = true;
                    }
                }
            });

            // Section 2: Secondary Cards Visibility
            ui.add_space(6.0);
            ui.label(
                RichText::new("SECONDARY DASHBOARD CARDS")
                    .font(egui::FontId::monospace(10.5))
                    .strong()
                    .color(palette.text_main),
            );

            // Checkbox rows with clean retro-industrial styling (no conflicting selection rectangles)
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
            ui.add_space(6.0);
            ui.label(
                RichText::new("VISUAL THEME")
                    .font(egui::FontId::monospace(10.5))
                    .strong()
                    .color(palette.text_main),
            );
            egui::Frame::none()
                .fill(palette.background)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .rounding(0.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("●")
                                .font(egui::FontId::monospace(11.0))
                                .color(palette.bar_active),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("Vintage")
                                .font(egui::FontId::monospace(10.5))
                                .strong()
                                .color(palette.text_main),
                        );
                        ui.label(
                            RichText::new("(Retro-Industrial Ivory & Orange)")
                                .font(egui::FontId::monospace(10.0))
                                .color(palette.text_muted),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new("ACTIVE")
                                    .font(egui::FontId::monospace(9.5))
                                    .strong()
                                    .color(palette.positive),
                            );
                        });
                    });
                });

            // Bottom action row: Reset Defaults & Close
            ui.add_space(8.0);
            render_themed_separator(ui, palette);
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
                .min_size(vec2(130.0, 24.0))
                .rounding(0.0);

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
                    .min_size(vec2(70.0, 24.0))
                    .rounding(0.0);

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
        .min_size(vec2(36.0, 18.0))
        .rounding(0.0);

        if ui.add(pill_btn).clicked() {
            *value = !*value;
            changed = true;
        }

        let label_color = if *value { palette.text_main } else { palette.text_muted };
        let text_resp = ui.add(
            egui::Label::new(
                RichText::new(label)
                    .font(egui::FontId::monospace(10.5))
                    .color(label_color),
            )
            .sense(egui::Sense::click()),
        );
        if text_resp.clicked() {
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
        assert!(open);
    }

    #[test]
    fn test_settings_window_escape_key_closes_modal() {
        let mut config = AppConfig::default();
        let palette = ThemePalette::vintage();
        let ctx = egui::Context::default();
        let mut open = true;

        let _ = ctx.run(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: egui::Key::Escape,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::default(),
                }],
                ..Default::default()
            },
            |ctx| {
                let _ = render_settings_window(ctx, &mut open, &mut config, &palette);
            },
        );

        assert!(!open, "Settings window should close when Escape key is pressed");
    }
}
