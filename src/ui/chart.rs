use egui::{pos2, vec2, Stroke, Ui};
use crate::telemetry::models::HistoryPoint;
use crate::theme::ThemePalette;
use crate::ui::widgets::card_container;

pub fn draw_history_chart(
    ui: &mut Ui,
    history: &[HistoryPoint],
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        let inner_w = ui.available_width();
        let inner_h = ui.available_height();

        // Header: "Last 60 seconds" and Legend "CPU | GPU"
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Last 60 seconds")
                    .font(egui::FontId::monospace(11.5))
                    .strong()
                    .color(palette.text_main),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // GPU legend
                ui.label(
                    egui::RichText::new("GPU")
                        .font(egui::FontId::monospace(10.5))
                        .color(palette.text_muted),
                );
                let (gpu_rect, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(gpu_rect, 0.0, palette.chart_line2);

                ui.add_space(8.0);

                // CPU legend
                ui.label(
                    egui::RichText::new("CPU")
                        .font(egui::FontId::monospace(10.5))
                        .color(palette.text_muted),
                );
                let (cpu_rect, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(cpu_rect, 0.0, palette.chart_line1);
            });
        });

        ui.add_space(4.0);

        // Allocate the chart painter within inner_h - 48.0
        let chart_height = (inner_h - 48.0).max(30.0);
        let (response, painter) = ui.allocate_painter(vec2(inner_w, chart_height), egui::Sense::hover());
        let rect = response.rect;

        // Visible grid lines using palette.bar_track
        let grid_stroke = Stroke::new(1.0_f32, palette.bar_track);
        for i in 1..=3 {
            let y = rect.min.y + (rect.height() * (i as f32 / 4.0));
            painter.line_segment([pos2(rect.min.x, y), pos2(rect.max.x, y)], grid_stroke);
        }

        // Plot curves across full width of the chart
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

            // Draw GPU line segments
            let gpu_stroke = Stroke::new(2.0_f32, palette.chart_line2);
            for w in gpu_pts.windows(2) {
                painter.line_segment([w[0], w[1]], gpu_stroke);
            }

            // Draw CPU line segments
            let cpu_stroke = Stroke::new(2.0_f32, palette.chart_line1);
            for w in cpu_pts.windows(2) {
                painter.line_segment([w[0], w[1]], cpu_stroke);
            }
        }

        ui.add_space(4.0);

        // Allocate bottom time labels in their own horizontal row so they are fully visible
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let font = egui::FontId::monospace(10.0);
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
