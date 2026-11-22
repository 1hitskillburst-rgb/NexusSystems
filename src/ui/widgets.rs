use egui::{pos2, vec2, Painter, Rect, Stroke, Ui};
use crate::theme::ThemePalette;

pub fn draw_segmented_bar(
    painter: &Painter,
    rect: Rect,
    percentage: f32,
    palette: &ThemePalette,
) {
    let pct = percentage.clamp(0.0, 100.0);
    let border_stroke = Stroke::new(1.0_f32, palette.border);

    // Draw container border
    painter.rect(
        rect,
        0.0,
        palette.bar_track,
        border_stroke,
    );

    let inner_rect = rect.shrink2(vec2(2.0, 2.0));
    let num_segments = 36;
    let gap = 1.5;
    let total_width = inner_rect.width();
    let total_gaps = (num_segments - 1) as f32 * gap;
    let segment_width = ((total_width - total_gaps) / num_segments as f32).max(1.0);

    let active_segments = ((pct / 100.0) * num_segments as f32).round() as usize;

    for i in 0..num_segments {
        let x0 = inner_rect.min.x + i as f32 * (segment_width + gap);
        let x1 = (x0 + segment_width).min(inner_rect.max.x);
        let seg_rect = Rect::from_min_max(
            pos2(x0, inner_rect.min.y),
            pos2(x1, inner_rect.max.y),
        );

        if i < active_segments {
            painter.rect_filled(seg_rect, 0.0, palette.bar_active);
        }
    }
}

pub fn card_container<R>(
    ui: &mut Ui,
    palette: &ThemePalette,
    width: f32,
    height: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let frame = egui::Frame::none()
        .fill(palette.card_bg)
        .stroke(Stroke::new(1.0_f32, palette.border))
        .inner_margin(12.0);

    let inner_w = (width - 24.0).max(40.0);
    let inner_h = (height - 24.0).max(40.0);

    ui.allocate_ui_with_layout(
        vec2(width, height),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            frame.show(ui, |ui| {
                ui.set_width(inner_w);
                ui.set_height(inner_h);
                add_contents(ui)
            }).inner
        },
    ).inner
}

pub fn stat_row(ui: &mut Ui, label: &str, value: &str, palette: &ThemePalette) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .font(egui::FontId::monospace(11.0))
                .color(palette.text_muted),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .font(egui::FontId::monospace(11.5))
                    .strong()
                    .color(palette.text_main),
            );
        });
    });
}
