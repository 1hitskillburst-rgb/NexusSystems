use egui::{vec2, Stroke, Ui};
use crate::telemetry::models::SystemInfoSnapshot;
use crate::theme::ThemePalette;

pub fn render_header(
    ui: &mut Ui,
    info: &SystemInfoSnapshot,
    palette: &ThemePalette,
    settings_open: &mut bool,
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

        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_header(ui, &info, &palette, &mut settings_open);
            });
        });
        assert!(!settings_open);
    }
}
