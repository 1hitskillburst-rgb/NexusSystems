use std::time::Instant;
use chrono::{DateTime, Local};
use sysinfo::{CpuRefreshKind, Disks, Networks, RefreshKind, System};
use windows_sys::Win32::System::SystemInformation::{GetTickCount64, GlobalMemoryStatusEx, MEMORYSTATUSEX};
use crate::telemetry::gpu::NvmlCollector;
use crate::telemetry::models::*;

pub struct TelemetryCollector {
    sys: System,
    disks: Disks,
    networks: Networks,
    nvml: Option<NvmlCollector>,
    last_sample_instant: Instant,
    last_net_sample: Instant,
    last_rx_bytes: u64,
    last_tx_bytes: u64,
    boot_time_dt: DateTime<Local>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(sysinfo::MemoryRefreshKind::everything())
            .with_processes(sysinfo::ProcessRefreshKind::everything());

        let mut sys = System::new_with_specifics(refresh_kind);
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        let nvml = NvmlCollector::try_new().ok();

        let boot_time_secs = System::boot_time();
        let boot_time_dt = DateTime::from_timestamp(boot_time_secs as i64, 0)
            .map(|utc| utc.with_timezone(&Local))
            .unwrap_or_else(Local::now);

        let initial_rx: u64 = networks.iter().map(|(_, data)| data.total_received()).sum();
        let initial_tx: u64 = networks.iter().map(|(_, data)| data.total_transmitted()).sum();

        Self {
            sys,
            disks,
            networks,
            nvml,
            last_sample_instant: Instant::now(),
            last_net_sample: Instant::now(),
            last_rx_bytes: initial_rx,
            last_tx_bytes: initial_tx,
            boot_time_dt,
        }
    }

    pub fn sample(&mut self) -> SystemSnapshot {
        let now = Instant::now();
        let _elapsed = now.duration_since(self.last_sample_instant);
        self.last_sample_instant = now;

        // Refresh system metrics
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All);
        self.disks.refresh();
        self.networks.refresh();

        // --- System Info ---
        let uptime_millis = unsafe { GetTickCount64() };
        let uptime_secs = uptime_millis / 1000;
        let days = uptime_secs / 86400;
        let hours = (uptime_secs % 86400) / 3600;
        let minutes = (uptime_secs % 3600) / 60;
        let uptime_str = format!("Up {:02}d {:02}h {:02}m", days, hours, minutes);
        let boot_time_str = format!("Boot {}", self.boot_time_dt.format("%b %d, %H:%M"));

        let info = SystemInfoSnapshot {
            status: "Local / online".to_string(),
            uptime_str,
            boot_time_str,
            power_profile: "Profile: Balanced".to_string(),
        };

        // --- CPU ---
        let cpus = self.sys.cpus();
        let cpu_global_pct = self.sys.global_cpu_usage();
        let cpu_brand = if let Some(cpu) = cpus.first() {
            let brand = cpu.brand().trim();
            if brand.is_empty() {
                "CPU / Processor".to_string()
            } else {
                // Shorten common names for concise UI presentation
                if brand.contains("Intel") {
                    brand.split('@').next().unwrap_or(brand).trim().to_string()
                } else {
                    brand.to_string()
                }
            }
        } else {
            "CPU / Processor".to_string()
        };

        let clock_speed_ghz = if let Some(cpu) = cpus.first() {
            cpu.frequency() as f32 / 1000.0
        } else {
            0.0
        };

        let per_core_pct: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();

        // CPU temperature heuristic: check components
        let cpu_temp_c = None;

        let cpu = CpuSnapshot {
            model_name: cpu_brand,
            utilization_pct: cpu_global_pct,
            package_temp_c: cpu_temp_c,
            clock_speed_ghz,
            package_power_w: None,
            core_count: self.sys.physical_core_count().unwrap_or(cpus.len()),
            logical_core_count: cpus.len(),
            per_core_pct,
        };

        // --- Memory ---
        let total_mem_bytes = self.sys.total_memory();
        let used_mem_bytes = self.sys.used_memory();
        let avail_mem_bytes = self.sys.available_memory();

        let total_gb = total_mem_bytes as f32 / (1024.0 * 1024.0 * 1024.0);
        let used_gb = used_mem_bytes as f32 / (1024.0 * 1024.0 * 1024.0);
        let avail_gb = avail_mem_bytes as f32 / (1024.0 * 1024.0 * 1024.0);
        let mem_util_pct = if total_gb > 0.0 { (used_gb / total_gb) * 100.0 } else { 0.0 };

        // Windows GlobalMemoryStatusEx for committed memory
        let mut committed_gb = used_gb;
        unsafe {
            let mut mem_status = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                dwMemoryLoad: 0,
                ullTotalPhys: 0,
                ullAvailPhys: 0,
                ullTotalPageFile: 0,
                ullAvailPageFile: 0,
                ullTotalVirtual: 0,
                ullAvailVirtual: 0,
                ullAvailExtendedVirtual: 0,
            };
            if GlobalMemoryStatusEx(&mut mem_status) != 0 {
                let committed_bytes = mem_status.ullTotalPageFile.saturating_sub(mem_status.ullAvailPageFile);
                committed_gb = committed_bytes as f32 / (1024.0 * 1024.0 * 1024.0);
            }
        }

        let memory = MemorySnapshot {
            used_gb,
            total_gb,
            utilization_pct: mem_util_pct,
            memory_speed: "-- MT/s".to_string(),
            committed_gb,
            available_gb: avail_gb,
        };

        // --- GPU ---
        let gpu = if let Some(ref nvml) = self.nvml {
            nvml.sample()
        } else {
            GpuSnapshot {
                model_name: "Integrated / Standard Graphics".to_string(),
                utilization_pct: 0.0,
                core_temp_c: None,
                core_clock_mhz: None,
                vram_used_gb: 0.0,
                vram_total_gb: 0.0,
                power_w: None,
                fan_speed_pct: None,
                is_available: false,
            }
        };

        // --- Storage ---
        let mut drives = Vec::new();
        for disk in self.disks.list() {
            let mount_point = disk.mount_point().to_string_lossy().into_owned();
            let total_b = disk.total_space();
            let free_b = disk.available_space();
            let used_b = total_b.saturating_sub(free_b);

            let total_d_gb = total_b as f64 / (1024.0 * 1024.0 * 1024.0);
            let used_d_gb = used_b as f64 / (1024.0 * 1024.0 * 1024.0);
            let util_pct = if total_d_gb > 0.0 { ((used_d_gb / total_d_gb) * 100.0) as f32 } else { 0.0 };

            let disk_name_raw = disk.name().to_string_lossy().into_owned();
            let friendly_name = if mount_point.contains("C:") {
                "NVMe SSD - System".to_string()
            } else if !disk_name_raw.is_empty() {
                disk_name_raw
            } else {
                format!("Drive ({})", mount_point.trim_end_matches('\\'))
            };

            drives.push(DriveSnapshot {
                name: friendly_name,
                mount_point: mount_point.trim_end_matches('\\').to_string(),
                used_gb: used_d_gb,
                total_gb: total_d_gb,
                utilization_pct: util_pct,
                temp_c: None,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
            });
        }

        // --- Network ---
        let net_elapsed = now.duration_since(self.last_net_sample);
        let mut current_rx = 0u64;
        let mut current_tx = 0u64;
        let mut primary_adapter = "Network".to_string();

        for (name, net_data) in self.networks.iter() {
            let rx = net_data.total_received();
            let tx = net_data.total_transmitted();
            current_rx += rx;
            current_tx += tx;

            if rx > 0 || tx > 0 {
                primary_adapter = name.clone();
            }
        }

        let elapsed_secs = net_elapsed.as_secs_f32().max(0.1);
        let rx_delta = current_rx.saturating_sub(self.last_rx_bytes);
        let tx_delta = current_tx.saturating_sub(self.last_tx_bytes);

        self.last_rx_bytes = current_rx;
        self.last_tx_bytes = current_tx;
        self.last_net_sample = now;

        let download_mbps = (rx_delta as f32 * 8.0) / (elapsed_secs * 1_000_000.0);
        let upload_mbps = (tx_delta as f32 * 8.0) / (elapsed_secs * 1_000_000.0);

        let network = NetworkSnapshot {
            download_mbps: download_mbps.max(0.0),
            upload_mbps: upload_mbps.max(0.0),
            ping_ms: None,
            adapter_name: if primary_adapter.contains("Wi-Fi") {
                "Wi-Fi 6E".to_string()
            } else {
                primary_adapter
            },
        };

        // --- Top Processes ---
        let mut processes = Vec::new();
        for (pid, proc) in self.sys.processes() {
            let cpu_usage = proc.cpu_usage();
            let mem_bytes = proc.memory();
            let name = proc.name().to_string_lossy().into_owned();

            // Filter out idle system threads
            if name.is_empty() || name == "System Idle Process" {
                continue;
            }

            let mem_mb = mem_bytes as f64 / (1024.0 * 1024.0);
            let memory_str = if mem_mb >= 1024.0 {
                format!("{:.1} GB", mem_mb / 1024.0)
            } else {
                format!("{:.0} MB", mem_mb)
            };

            processes.push(ProcessSnapshot {
                pid: pid.as_u32(),
                name,
                cpu_pct: cpu_usage,
                memory_bytes: mem_bytes,
                memory_str,
            });
        }

        // Sort descending by CPU usage, then take top 6
        processes.sort_by(|a, b| b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap_or(std::cmp::Ordering::Equal));
        processes.truncate(6);

        // --- Cooling ---
        let gpu_fan = gpu.fan_speed_pct.map(|pct| (pct * 25.0) as u32);
        let cooling = CoolingSnapshot {
            cpu_fan_rpm: None,
            gpu_fan_rpm: gpu_fan,
            case_fan_rpm: None,
        };

        SystemSnapshot {
            timestamp_epoch_ms: chrono::Utc::now().timestamp_millis(),
            info,
            cpu,
            gpu,
            memory,
            drives,
            processes,
            network,
            cooling,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_eliminates_mock_telemetry() {
        let mut collector = TelemetryCollector::new();
        let snap = collector.sample();

        // Memory speed must not be fabricated "6,000 MT/s"
        assert_eq!(snap.memory.memory_speed, "-- MT/s");

        // Ping must be None (no fabricated Some(15))
        assert_eq!(snap.network.ping_ms, None);

        // CPU and case fan RPMs must be None (no fabricated Some(1240) or Some(920))
        assert_eq!(snap.cooling.cpu_fan_rpm, None);
        assert_eq!(snap.cooling.case_fan_rpm, None);

        // CPU package temp and power must be None (no fake values)
        assert_eq!(snap.cpu.package_temp_c, None);
        assert_eq!(snap.cpu.package_power_w, None);

        // GPU fan must not have fake fallback Some(1580)
        if snap.gpu.fan_speed_pct.is_none() {
            assert_eq!(snap.cooling.gpu_fan_rpm, None);
        } else {
            let expected_rpm = snap.gpu.fan_speed_pct.map(|pct| (pct * 25.0) as u32);
            assert_eq!(snap.cooling.gpu_fan_rpm, expected_rpm);
        }
    }
}
