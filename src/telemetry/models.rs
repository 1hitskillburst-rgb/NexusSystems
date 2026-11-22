use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfoSnapshot {
    pub status: String,
    pub uptime_str: String,
    pub boot_time_str: String,
    pub power_profile: String,
}

impl Default for SystemInfoSnapshot {
    fn default() -> Self {
        Self {
            status: "Local / online".to_string(),
            uptime_str: "Up --".to_string(),
            boot_time_str: "Boot --".to_string(),
            power_profile: "Profile: Balanced".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSnapshot {
    pub model_name: String,
    pub utilization_pct: f32,
    pub package_temp_c: Option<f32>,
    pub clock_speed_ghz: f32,
    pub package_power_w: Option<f32>,
    pub core_count: usize,
    pub logical_core_count: usize,
    pub per_core_pct: Vec<f32>,
}

impl Default for CpuSnapshot {
    fn default() -> Self {
        Self {
            model_name: "CPU / Processor".to_string(),
            utilization_pct: 0.0,
            package_temp_c: None,
            clock_speed_ghz: 0.0,
            package_power_w: None,
            core_count: 0,
            logical_core_count: 0,
            per_core_pct: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSnapshot {
    pub model_name: String,
    pub utilization_pct: f32,
    pub core_temp_c: Option<f32>,
    pub core_clock_mhz: Option<f32>,
    pub vram_used_gb: f32,
    pub vram_total_gb: f32,
    pub power_w: Option<f32>,
    pub fan_speed_pct: Option<f32>,
    pub is_available: bool,
}

impl Default for GpuSnapshot {
    fn default() -> Self {
        Self {
            model_name: "GPU / Graphics".to_string(),
            utilization_pct: 0.0,
            core_temp_c: None,
            core_clock_mhz: None,
            vram_used_gb: 0.0,
            vram_total_gb: 0.0,
            power_w: None,
            fan_speed_pct: None,
            is_available: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub used_gb: f32,
    pub total_gb: f32,
    pub utilization_pct: f32,
    pub memory_speed: String,
    pub committed_gb: f32,
    pub available_gb: f32,
}

impl Default for MemorySnapshot {
    fn default() -> Self {
        Self {
            used_gb: 0.0,
            total_gb: 0.0,
            utilization_pct: 0.0,
            memory_speed: "Unknown MT/s".to_string(),
            committed_gb: 0.0,
            available_gb: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveSnapshot {
    pub name: String,
    pub mount_point: String,
    pub used_gb: f64,
    pub total_gb: f64,
    pub utilization_pct: f32,
    pub temp_c: Option<f32>,
    pub read_mb_s: f32,
    pub write_mb_s: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub cpu_pct: f32,
    pub memory_bytes: u64,
    pub memory_str: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSnapshot {
    pub download_mbps: f32,
    pub upload_mbps: f32,
    pub ping_ms: Option<u32>,
    pub adapter_name: String,
}

impl Default for NetworkSnapshot {
    fn default() -> Self {
        Self {
            download_mbps: 0.0,
            upload_mbps: 0.0,
            ping_ms: None,
            adapter_name: "Ethernet / Wi-Fi".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoolingSnapshot {
    pub cpu_fan_rpm: Option<u32>,
    pub gpu_fan_rpm: Option<u32>,
    pub case_fan_rpm: Option<u32>,
}

impl CoolingSnapshot {
    /// Returns true if at least one fan tachometer sensor returned a reading.
    pub fn has_data(&self) -> bool {
        self.cpu_fan_rpm.is_some() || self.gpu_fan_rpm.is_some() || self.case_fan_rpm.is_some()
    }
}

impl Default for CoolingSnapshot {
    fn default() -> Self {
        Self {
            cpu_fan_rpm: None,
            gpu_fan_rpm: None,
            case_fan_rpm: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPoint {
    pub timestamp_secs: f64,
    pub cpu_pct: f32,
    pub gpu_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub timestamp_epoch_ms: i64,
    pub info: SystemInfoSnapshot,
    pub cpu: CpuSnapshot,
    pub gpu: GpuSnapshot,
    pub memory: MemorySnapshot,
    pub drives: Vec<DriveSnapshot>,
    pub processes: Vec<ProcessSnapshot>,
    pub network: NetworkSnapshot,
    pub cooling: CoolingSnapshot,
}

impl Default for SystemSnapshot {
    fn default() -> Self {
        Self {
            timestamp_epoch_ms: 0,
            info: SystemInfoSnapshot::default(),
            cpu: CpuSnapshot::default(),
            gpu: GpuSnapshot::default(),
            memory: MemorySnapshot::default(),
            drives: Vec::new(),
            processes: Vec::new(),
            network: NetworkSnapshot::default(),
            cooling: CoolingSnapshot::default(),
        }
    }
}
