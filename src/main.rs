#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod theme;
mod telemetry;
mod ui;

use app::NexusApp;
use telemetry::TelemetryCollector;

#[cfg(windows)]
extern "system" {
    fn AttachConsole(dw_process_id: u32) -> i32;
    fn GetStdHandle(n_std_handle: u32) -> *mut std::ffi::c_void;
    fn SetStdHandle(n_std_handle: u32, h_handle: *mut std::ffi::c_void) -> i32;
    fn CreateFileW(
        lp_file_name: *const u16,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: *mut std::ffi::c_void,
        dw_creation_disposition: u32,
        dw_flags_and_attributes: u32,
        h_template_file: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
}

#[cfg(windows)]
const ATTACH_PARENT_PROCESS: u32 = 0xFFFF_FFFF;
#[cfg(windows)]
const STD_OUTPUT_HANDLE: u32 = 0xFFFF_FFF5;
#[cfg(windows)]
const STD_ERROR_HANDLE: u32 = 0xFFFF_FFF4;
#[cfg(windows)]
const INVALID_HANDLE_VALUE: *mut std::ffi::c_void = -1isize as *mut std::ffi::c_void;

pub const INITIAL_INNER_WIDTH: f32 = 1220.0;
pub const INITIAL_INNER_HEIGHT: f32 = 460.0;
pub const MIN_INNER_WIDTH: f32 = 960.0;
pub const MIN_INNER_HEIGHT: f32 = 440.0;

fn main() -> Result<(), eframe::Error> {
    // If CLI argument --test-telemetry is passed, run telemetry test in console
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--test-telemetry".to_string()) {
        #[cfg(windows)]
        unsafe {
            let current_out = GetStdHandle(STD_OUTPUT_HANDLE);
            if current_out.is_null() || current_out == INVALID_HANDLE_VALUE {
                if AttachConsole(ATTACH_PARENT_PROCESS) != 0 {
                    let conout_name: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
                    let handle = CreateFileW(
                        conout_name.as_ptr(),
                        0x4000_0000 | 0x8000_0000, // GENERIC_READ | GENERIC_WRITE
                        1 | 2,                     // FILE_SHARE_READ | FILE_SHARE_WRITE
                        std::ptr::null_mut(),
                        3, // OPEN_EXISTING
                        0,
                        std::ptr::null_mut(),
                    );
                    if handle != INVALID_HANDLE_VALUE && !handle.is_null() {
                        SetStdHandle(STD_OUTPUT_HANDLE, handle);
                        SetStdHandle(STD_ERROR_HANDLE, handle);
                    }
                }
            }
        }

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
            .with_inner_size([INITIAL_INNER_WIDTH, INITIAL_INNER_HEIGHT])
            .with_min_inner_size([MIN_INNER_WIDTH, MIN_INNER_HEIGHT])
            .with_resizable(true)
            .with_drag_and_drop(false),
        vsync: true,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "NEXUS / SYSTEMS --- PC Control Center",
        native_options,
        Box::new(|cc| Ok(Box::new(NexusApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_attach_parent_process_constant() {
        assert_eq!(ATTACH_PARENT_PROCESS, 0xFFFF_FFFF);
    }

    #[test]
    #[cfg(windows)]
    fn test_attach_console_invocation() {
        // Calling AttachConsole when already attached or without parent console returns non-zero (success)
        // or zero (failure), but does not panic or violate memory safety.
        unsafe {
            let _ = AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    #[test]
    fn test_minimum_window_size_configuration_bounds() {
        assert!(MIN_INNER_WIDTH >= 960.0, "Minimum window width must be at least 960px");
        assert!(MIN_INNER_HEIGHT >= 440.0, "Minimum window height must be at least 440px to fit layout budget");
        assert!(INITIAL_INNER_WIDTH >= MIN_INNER_WIDTH);
        assert!(INITIAL_INNER_HEIGHT >= MIN_INNER_HEIGHT);
    }
}

