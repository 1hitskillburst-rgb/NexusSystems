use egui::{pos2, vec2, Stroke, Ui};
use crate::telemetry::models::HistoryPoint;
use crate::theme::ThemePalette;

pub fn draw_history_chart(
    ui: &mut Ui,
    history: &[HistoryPoint],
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    let frame = egui::Frame::none()
        .fill(palette.card_bg)
        .stroke(Stroke::new(1.0_f32, palette.border))
        .inner_margin(12.0);

    let inner_w = (width - 24.0).max(40.0);
    let inner_h = (height - 24.0).max(40.0);

    frame.show(ui, |ui| {
        ui.set_width(inner_w);
        ui.set_height(inner_h);

        // Header: "Last 60 seconds" and Legend "CPU | GPU"
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Last 60 seconds")
                    .font(egui::FontId::monospace(12.0))
                    .strong()
                    .color(palette.text_main),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // GPU legend
                ui.label(
                    egui::RichText::new("GPU")
                        .font(egui::FontId::monospace(11.0))
                        .color(palette.text_muted),
                );
                let (gpu_rect, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(gpu_rect, 0.0, palette.chart_line2);

                ui.add_space(8.0);

                // CPU legend
                ui.label(
                    egui::RichText::new("CPU")
                        .font(egui::FontId::monospace(11.0))
                        .color(palette.text_muted),
                );
                let (cpu_rect, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(cpu_rect, 0.0, palette.chart_line1);
            });
        });

        ui.add_space(4.0);

        // Chart area allocation: header takes ~20px, bottom axis takes ~18px, spacing 8px -> ~46px overhead
        let chart_height = (inner_h - 46.0).max(30.0);
        let (response, painter) = ui.allocate_painter(vec2(inner_w, chart_height), egui::Sense::hover());
        let rect = response.rect;

        // Subtle guide lines
        let grid_stroke = Stroke::new(0.8_f32, palette.border.linear_multiply(0.4));
        for i in 1..=3 {
            let y = rect.min.y + (rect.height() * (i as f32 / 4.0));
            painter.line_segment([pos2(rect.min.x, y), pos2(rect.max.x, y)], grid_stroke);
        }

        // Plot curves
        if history.len() >= 2 {
            let max_points = 60;
            let start_idx = history.len().saturating_sub(max_points);
            let slice = &history[start_idx..];

            let mut cpu_pts = Vec::new();
            let mut gpu_pts = Vec::new();

            let x_step = rect.width() / (max_points - 1) as f32;
            let offset_x = (max_points - slice.len()) as f32 * x_step;

            for (i, pt) in slice.iter().enumerate() {
                let x = rect.min.x + offset_x + (i as f32 * x_step);
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

        // Draw axis labels dynamically aligned to the chart coordinates
        let label_font = egui::FontId::monospace(10.0);
        let label_y = rect.max.y + 4.0;
        painter.text(pos2(rect.min.x, label_y), egui::Align2::LEFT_TOP, "60s ago", label_font.clone(), palette.text_muted);
        painter.text(pos2(rect.min.x + rect.width() * 0.25, label_y), egui::Align2::CENTER_TOP, "45s", label_font.clone(), palette.text_muted);
        painter.text(pos2(rect.min.x + rect.width() * 0.50, label_y), egui::Align2::CENTER_TOP, "30s", label_font.clone(), palette.text_muted);
        painter.text(pos2(rect.min.x + rect.width() * 0.75, label_y), egui::Align2::CENTER_TOP, "15s", label_font.clone(), palette.text_muted);
        painter.text(pos2(rect.max.x, label_y), egui::Align2::RIGHT_TOP, "Now", label_font, palette.text_muted);
    });
}
