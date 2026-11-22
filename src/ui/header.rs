use egui::{vec2, Stroke, Ui};
use crate::telemetry::models::SystemInfoSnapshot;
use crate::theme::{ThemeMode, ThemePalette};

pub fn render_header(
    ui: &mut Ui,
    info: &SystemInfoSnapshot,
    current_theme: &mut ThemeMode,
    palette: &ThemePalette,
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

        // System status and Theme Selector on Right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Theme selection pills
            ui.horizontal(|ui| {
                for (idx, &mode) in ThemeMode::all().iter().enumerate() {
                    let num = idx + 1;
                    let is_active = *current_theme == mode;
                    let bg = if is_active { palette.bar_active } else { palette.card_bg };
                    let fg = if is_active { palette.background } else { palette.text_main };

                    let btn = egui::Button::new(
                        egui::RichText::new(format!("{num}"))
                            .font(egui::FontId::monospace(10.0))
                            .color(fg),
                    )
                    .fill(bg)
                    .stroke(Stroke::new(1.0_f32, palette.border))
                    .min_size(vec2(20.0, 18.0));

                    if ui.add(btn).on_hover_text(mode.name()).clicked() {
                        *current_theme = mode;
                    }
                }
            });

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
