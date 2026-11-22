#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod theme;
mod telemetry;
mod ui;

use app::NexusApp;
use telemetry::TelemetryCollector;

fn main() -> Result<(), eframe::Error> {
    // If CLI argument --test-telemetry is passed, run telemetry test in console
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--test-telemetry".to_string()) {
        println!("=== NEXUS SYSTEMS TELEMETRY TEST ===");
        let mut collector = TelemetryCollector::new();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let snap = collector.sample();
        println!("System Status: {}", snap.info.status);
        println!("Uptime: {}", snap.info.uptime_str);
        println!("CPU: {} | Util: {:.1}% | Speed: {:.2} GHz", snap.cpu.model_name, snap.cpu.utilization_pct, snap.cpu.clock_speed_ghz);
        println!("GPU: {} | Util: {:.1}% | Temp: {:?} | VRAM: {:.1}/{:.1} GB", snap.gpu.model_name, snap.gpu.utilization_pct, snap.gpu.core_temp_c, snap.gpu.vram_used_gb, snap.gpu.vram_total_gb);
        println!("RAM: {:.1}/{:.1} GB ({:.1}%) | Speed: {}", snap.memory.used_gb, snap.memory.total_gb, snap.memory.utilization_pct, snap.memory.memory_speed);
        println!("Drives ({}):", snap.drives.len());
        for d in &snap.drives {
            println!("  - {} [{}] {:.1}/{:.1} GB ({:.1}%)", d.name, d.mount_point, d.used_gb, d.total_gb, d.utilization_pct);
        }
        println!("Top Processes ({}):", snap.processes.len());
        for p in &snap.processes {
            println!("  - {:<18} CPU: {:>4.1}% | RAM: {}", p.name, p.cpu_pct, p.memory_str);
        }
        println!("Network: DL {:.1} Mbps | UL {:.1} Mbps | Ping: {:?} ms", snap.network.download_mbps, snap.network.upload_mbps, snap.network.ping_ms);
        println!("=====================================");
        return Ok(());
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("NEXUS / SYSTEMS --- PC Control Center")
            .with_inner_size([1220.0, 420.0])
            .with_min_inner_size([960.0, 380.0])
            .with_resizable(true)
            .with_drag_and_drop(false),
        vsync: true,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            present_mode: eframe::wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: Some(2),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "NEXUS / SYSTEMS --- PC Control Center",
        native_options,
        Box::new(|cc| Ok(Box::new(NexusApp::new(cc)))),
    )
}
