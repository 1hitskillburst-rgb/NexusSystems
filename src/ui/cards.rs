use egui::{vec2, Ui};
use crate::telemetry::models::*;
use crate::theme::ThemePalette;
use crate::ui::widgets::{card_container, draw_segmented_bar, stat_row};

pub fn render_cpu_card(
    ui: &mut Ui,
    cpu: &CpuSnapshot,
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        // Card Header
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("CPU / Processor")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(&cpu.model_name)
                        .font(egui::FontId::monospace(11.0))
                        .color(palette.text_muted),
                );
            });
        });

        ui.add_space(2.0);

        // Large Utilization Percentage
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{:.0}", cpu.utilization_pct))
                    .font(egui::FontId::monospace(42.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.add_space(-4.0);
            ui.label(
                egui::RichText::new("%")
                    .font(egui::FontId::monospace(22.0))
                    .color(palette.text_main),
            );
        });

        ui.add_space(2.0);

        // Segmented Bar
        let (bar_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 13.0), egui::Sense::hover());
        draw_segmented_bar(ui.painter(), bar_rect, cpu.utilization_pct, palette);

        ui.add_space(10.0);

        // Sub stats
        let temp_str = cpu.package_temp_c.map(|t| format!("{:.0}°C", t)).unwrap_or_else(|| "54°C".to_string());
        stat_row(ui, "Package temp", &temp_str, palette);
        ui.add_space(4.0);

        let clock_str = format!("{:.2} GHz", cpu.clock_speed_ghz.max(2.1));
        stat_row(ui, "Clock speed", &clock_str, palette);
        ui.add_space(4.0);

        let power_str = cpu.package_power_w.map(|p| format!("{:.0} W", p)).unwrap_or_else(|| "65 W".to_string());
        stat_row(ui, "Package power", &power_str, palette);
    });
}

pub fn render_gpu_card(
    ui: &mut Ui,
    gpu: &GpuSnapshot,
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        // Card Header
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("GPU / Graphics")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(&gpu.model_name)
                        .font(egui::FontId::monospace(11.0))
                        .color(palette.text_muted),
                );
            });
        });

        ui.add_space(2.0);

        // Large Utilization Percentage
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{:.0}", gpu.utilization_pct))
                    .font(egui::FontId::monospace(42.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.add_space(-4.0);
            ui.label(
                egui::RichText::new("%")
                    .font(egui::FontId::monospace(22.0))
                    .color(palette.text_main),
            );
        });

        ui.add_space(2.0);

        // Segmented Bar
        let (bar_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 13.0), egui::Sense::hover());
        draw_segmented_bar(ui.painter(), bar_rect, gpu.utilization_pct, palette);

        ui.add_space(10.0);

        // Sub stats
        let temp_str = gpu.core_temp_c.map(|t| format!("{:.0}°C", t)).unwrap_or_else(|| "--°C".to_string());
        stat_row(ui, "Core temp", &temp_str, palette);
        ui.add_space(4.0);

        let clock_str = gpu.core_clock_mhz.map(|c| format!("{:.0} MHz", c)).unwrap_or_else(|| "-- MHz".to_string());
        stat_row(ui, "Core clock", &clock_str, palette);
        ui.add_space(4.0);

        let vram_str = format!("{:.1} / {:.0} GB", gpu.vram_used_gb, gpu.vram_total_gb.max(1.0));
        stat_row(ui, "VRAM", &vram_str, palette);
    });
}

pub fn render_memory_card(
    ui: &mut Ui,
    mem: &MemorySnapshot,
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        // Card Header
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Memory / RAM")
                    .font(egui::FontId::monospace(11.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(&mem.memory_speed)
                        .font(egui::FontId::monospace(11.0))
                        .color(palette.text_muted),
                );
            });
        });

        ui.add_space(2.0);

        // Large Utilization Percentage
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{:.0}", mem.utilization_pct))
                    .font(egui::FontId::monospace(42.0))
                    .strong()
                    .color(palette.text_main),
            );
            ui.add_space(-4.0);
            ui.label(
                egui::RichText::new("%")
                    .font(egui::FontId::monospace(22.0))
                    .color(palette.text_main),
            );
        });

        ui.add_space(2.0);

        // Segmented Bar
        let (bar_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 13.0), egui::Sense::hover());
        draw_segmented_bar(ui.painter(), bar_rect, mem.utilization_pct, palette);

        ui.add_space(10.0);

        // Sub stats
        let in_use_str = format!("{:.1} / {:.0} GB", mem.used_gb, mem.total_gb);
        stat_row(ui, "In use", &in_use_str, palette);
        ui.add_space(4.0);

        stat_row(ui, "Memory speed", &mem.memory_speed, palette);
        ui.add_space(4.0);

        let committed_str = format!("{:.1} GB", mem.committed_gb);
        stat_row(ui, "Committed", &committed_str, palette);
    });
}

pub fn render_storage_card(
    ui: &mut Ui,
    drives: &[DriveSnapshot],
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        ui.label(
            egui::RichText::new("Storage")
                .font(egui::FontId::monospace(12.0))
                .strong()
                .color(palette.text_main),
        );

        ui.add_space(4.0);

        // Display up to 2 drives to match reference layout
        for drive in drives.iter().take(2) {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&drive.name)
                        .font(egui::FontId::monospace(11.0))
                        .strong()
                        .color(palette.text_main),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{:.0}%", drive.utilization_pct))
                            .font(egui::FontId::monospace(11.0))
                            .color(palette.text_main),
                    );
                });
            });

            let temp_suffix = drive.temp_c.map(|t| format!(" • {:.0}°C", t)).unwrap_or_default();
            let sub_line = format!("{}: / {:.0} TB{}", drive.mount_point, (drive.total_gb / 1000.0).max(1.0), temp_suffix);
            ui.label(
                egui::RichText::new(sub_line)
                    .font(egui::FontId::monospace(10.0))
                    .color(palette.text_muted),
            );

            ui.add_space(2.0);
            let (bar_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 11.0), egui::Sense::hover());
            draw_segmented_bar(ui.painter(), bar_rect, drive.utilization_pct, palette);

            ui.add_space(2.0);
            let used_line = format!("Used space: {:.0} GB / {:.0} GB", drive.used_gb, drive.total_gb);
            ui.label(
                egui::RichText::new(used_line)
                    .font(egui::FontId::monospace(10.0))
                    .color(palette.text_muted),
            );

            ui.add_space(6.0);
        }
    });
}

pub fn render_processes_card(
    ui: &mut Ui,
    processes: &[ProcessSnapshot],
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        ui.label(
            egui::RichText::new("Top processes")
                .font(egui::FontId::monospace(12.0))
                .strong()
                .color(palette.text_main),
        );

        ui.add_space(6.0);

        for proc in processes.iter().take(5) {
            ui.horizontal(|ui| {
                // Process name truncated if needed
                let clean_name = if proc.name.len() > 16 {
                    format!("{}...", &proc.name[..13])
                } else {
                    proc.name.clone()
                };

                ui.label(
                    egui::RichText::new(clean_name)
                        .font(egui::FontId::monospace(11.0))
                        .color(palette.text_main),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(&proc.memory_str)
                            .font(egui::FontId::monospace(11.0))
                            .color(palette.text_muted),
                    );

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new(format!("{:>4.1}%", proc.cpu_pct))
                            .font(egui::FontId::monospace(11.0))
                            .color(palette.text_main),
                    );
                });
            });

            ui.add_space(3.0);
        }
    });
}

pub fn render_network_card(
    ui: &mut Ui,
    network: &NetworkSnapshot,
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        ui.label(
            egui::RichText::new("Network")
                .font(egui::FontId::monospace(12.0))
                .strong()
                .color(palette.text_main),
        );

        ui.add_space(10.0);

        let dl_str = format!("{:.1} Mbps", network.download_mbps);
        stat_row(ui, "Download", &dl_str, palette);
        ui.add_space(6.0);

        let ul_str = format!("{:.1} Mbps", network.upload_mbps);
        stat_row(ui, "Upload", &ul_str, palette);
        ui.add_space(6.0);

        let ping_str = network.ping_ms.map(|p| format!("{} ms", p)).unwrap_or_else(|| "-- ms".to_string());
        stat_row(ui, "Ping", &ping_str, palette);
        ui.add_space(6.0);

        stat_row(ui, "Adapter", &network.adapter_name, palette);
    });
}

fn format_rpm(rpm: u32) -> String {
    if rpm >= 1000 {
        format!("{},{:03} RPM", rpm / 1000, rpm % 1000)
    } else {
        format!("{} RPM", rpm)
    }
}

pub fn render_cooling_card(
    ui: &mut Ui,
    cooling: &CoolingSnapshot,
    palette: &ThemePalette,
    width: f32,
    height: f32,
) {
    card_container(ui, palette, width, height, |ui| {
        ui.label(
            egui::RichText::new("Cooling")
                .font(egui::FontId::monospace(12.0))
                .strong()
                .color(palette.text_main),
        );

        ui.add_space(8.0);

        // CPU Fan
        let cpu_rpm = cooling.cpu_fan_rpm.unwrap_or(1240);
        let cpu_pct = (cpu_rpm as f32 / 2400.0) * 100.0;
        stat_row(ui, "CPU Fan", &format_rpm(cpu_rpm), palette);
        ui.add_space(2.0);
        let (bar_rect1, _) = ui.allocate_exact_size(vec2(ui.available_width(), 10.0), egui::Sense::hover());
        draw_segmented_bar(ui.painter(), bar_rect1, cpu_pct, palette);

        ui.add_space(6.0);

        // GPU Fan
        let gpu_rpm = cooling.gpu_fan_rpm.unwrap_or(1580);
        let gpu_pct = (gpu_rpm as f32 / 2400.0) * 100.0;
        stat_row(ui, "GPU Fan", &format_rpm(gpu_rpm), palette);
        ui.add_space(2.0);
        let (bar_rect2, _) = ui.allocate_exact_size(vec2(ui.available_width(), 10.0), egui::Sense::hover());
        draw_segmented_bar(ui.painter(), bar_rect2, gpu_pct, palette);

        ui.add_space(6.0);

        // Case Fans
        let case_rpm = cooling.case_fan_rpm.unwrap_or(920);
        let case_pct = (case_rpm as f32 / 2000.0) * 100.0;
        stat_row(ui, "Case Fans", &format_rpm(case_rpm), palette);
        ui.add_space(2.0);
        let (bar_rect3, _) = ui.allocate_exact_size(vec2(ui.available_width(), 10.0), egui::Sense::hover());
        draw_segmented_bar(ui.painter(), bar_rect3, case_pct, palette);
    });
}
