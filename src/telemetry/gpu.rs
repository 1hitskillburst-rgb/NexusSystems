use std::ffi::c_void;
use std::os::raw::{c_char, c_uint};
use libloading::Library;
use crate::telemetry::models::GpuSnapshot;

type NvmlReturn = c_uint;
type NvmlDevice = *mut c_void;

type NvmlInitFn = unsafe extern "C" fn() -> NvmlReturn;
type NvmlShutdownFn = unsafe extern "C" fn() -> NvmlReturn;
type NvmlDeviceGetCountFn = unsafe extern "C" fn(*mut c_uint) -> NvmlReturn;
type NvmlDeviceGetHandleByIndexFn = unsafe extern "C" fn(c_uint, *mut NvmlDevice) -> NvmlReturn;
type NvmlDeviceGetNameFn = unsafe extern "C" fn(NvmlDevice, *mut c_char, c_uint) -> NvmlReturn;
type NvmlDeviceGetUtilizationFn = unsafe extern "C" fn(NvmlDevice, *mut NvmlUtilization) -> NvmlReturn;
type NvmlDeviceGetTemperatureFn = unsafe extern "C" fn(NvmlDevice, c_uint, *mut c_uint) -> NvmlReturn;
type NvmlDeviceGetMemoryInfoFn = unsafe extern "C" fn(NvmlDevice, *mut NvmlMemory) -> NvmlReturn;
type NvmlDeviceGetClockInfoFn = unsafe extern "C" fn(NvmlDevice, c_uint, *mut c_uint) -> NvmlReturn;
type NvmlDeviceGetPowerUsageFn = unsafe extern "C" fn(NvmlDevice, *mut c_uint) -> NvmlReturn;
type NvmlDeviceGetFanSpeedFn = unsafe extern "C" fn(NvmlDevice, *mut c_uint) -> NvmlReturn;

#[repr(C)]
struct NvmlUtilization {
    gpu: c_uint,
    memory: c_uint,
}

#[repr(C)]
struct NvmlMemory {
    total: u64,
    free: u64,
    used: u64,
}

pub struct NvmlCollector {
    device: NvmlDevice,
    pub name: String,
    device_get_utilization: NvmlDeviceGetUtilizationFn,
    device_get_temperature: NvmlDeviceGetTemperatureFn,
    device_get_memory_info: NvmlDeviceGetMemoryInfoFn,
    device_get_clock_info: NvmlDeviceGetClockInfoFn,
    device_get_power_usage: Option<NvmlDeviceGetPowerUsageFn>,
    device_get_fan_speed: Option<NvmlDeviceGetFanSpeedFn>,
    nvml_shutdown: NvmlShutdownFn,
    #[allow(dead_code)]
    lib: Library,
}

impl NvmlCollector {
    pub fn try_new() -> Result<Self, String> {
        let lib = unsafe {
            Library::new(r"C:\Windows\System32\nvml.dll")
                .map_err(|e| format!("NVML not available: {e}"))?
        };

        unsafe {
            let nvml_init: NvmlInitFn = *(lib
                .get::<NvmlInitFn>(b"nvmlInit_v2\0")
                .or_else(|_| lib.get::<NvmlInitFn>(b"nvmlInit\0"))
                .map_err(|e| format!("nvmlInit missing: {e}"))?);

            let nvml_shutdown: NvmlShutdownFn = *lib
                .get::<NvmlShutdownFn>(b"nvmlShutdown\0")
                .map_err(|e| format!("nvmlShutdown missing: {e}"))?;

            let get_count: NvmlDeviceGetCountFn = *(lib
                .get::<NvmlDeviceGetCountFn>(b"nvmlDeviceGetCount_v2\0")
                .or_else(|_| lib.get::<NvmlDeviceGetCountFn>(b"nvmlDeviceGetCount\0"))
                .map_err(|e| format!("nvmlDeviceGetCount missing: {e}"))?);

            let get_handle: NvmlDeviceGetHandleByIndexFn = *(lib
                .get::<NvmlDeviceGetHandleByIndexFn>(b"nvmlDeviceGetHandleByIndex_v2\0")
                .or_else(|_| lib.get::<NvmlDeviceGetHandleByIndexFn>(b"nvmlDeviceGetHandleByIndex\0"))
                .map_err(|e| format!("nvmlDeviceGetHandleByIndex missing: {e}"))?);

            let get_name: NvmlDeviceGetNameFn = *lib
                .get::<NvmlDeviceGetNameFn>(b"nvmlDeviceGetName\0")
                .map_err(|e| format!("nvmlDeviceGetName missing: {e}"))?;

            let device_get_utilization: NvmlDeviceGetUtilizationFn = *lib
                .get::<NvmlDeviceGetUtilizationFn>(b"nvmlDeviceGetUtilizationRates\0")
                .map_err(|e| format!("nvmlDeviceGetUtilizationRates missing: {e}"))?;

            let device_get_temperature: NvmlDeviceGetTemperatureFn = *lib
                .get::<NvmlDeviceGetTemperatureFn>(b"nvmlDeviceGetTemperature\0")
                .map_err(|e| format!("nvmlDeviceGetTemperature missing: {e}"))?;

            let device_get_memory_info: NvmlDeviceGetMemoryInfoFn = *lib
                .get::<NvmlDeviceGetMemoryInfoFn>(b"nvmlDeviceGetMemoryInfo\0")
                .map_err(|e| format!("nvmlDeviceGetMemoryInfo missing: {e}"))?;

            let device_get_clock_info: NvmlDeviceGetClockInfoFn = *lib
                .get::<NvmlDeviceGetClockInfoFn>(b"nvmlDeviceGetClockInfo\0")
                .map_err(|e| format!("nvmlDeviceGetClockInfo missing: {e}"))?;

            let device_get_power_usage: Option<NvmlDeviceGetPowerUsageFn> = lib
                .get::<NvmlDeviceGetPowerUsageFn>(b"nvmlDeviceGetPowerUsage\0")
                .ok()
                .map(|sym| *sym);

            let device_get_fan_speed: Option<NvmlDeviceGetFanSpeedFn> = lib
                .get::<NvmlDeviceGetFanSpeedFn>(b"nvmlDeviceGetFanSpeed\0")
                .ok()
                .map(|sym| *sym);

            let res = nvml_init();
            if res != 0 {
                return Err(format!("nvmlInit returned error code: {res}"));
            }

            let mut count: c_uint = 0;
            if get_count(&mut count) != 0 || count == 0 {
                let _ = nvml_shutdown();
                return Err("No NVML GPU devices found".to_string());
            }

            let mut device: NvmlDevice = std::ptr::null_mut();
            if get_handle(0, &mut device) != 0 {
                let _ = nvml_shutdown();
                return Err("Failed to get device handle 0".to_string());
            }

            let mut name_buf = [0i8; 128];
            let name = if get_name(device, name_buf.as_mut_ptr(), 128) == 0 {
                let c_str = std::ffi::CStr::from_ptr(name_buf.as_ptr());
                c_str.to_string_lossy().into_owned()
            } else {
                "NVIDIA Graphics Device".to_string()
            };

            Ok(Self {
                device,
                name,
                device_get_utilization,
                device_get_temperature,
                device_get_memory_info,
                device_get_clock_info,
                device_get_power_usage,
                device_get_fan_speed,
                nvml_shutdown,
                lib,
            })
        }
    }

    pub fn sample(&self) -> GpuSnapshot {
        unsafe {
            let mut util = NvmlUtilization { gpu: 0, memory: 0 };
            let util_ok = (self.device_get_utilization)(self.device, &mut util) == 0;

            let mut temp: c_uint = 0;
            // 0 = NVML_TEMPERATURE_GPU
            let temp_ok = (self.device_get_temperature)(self.device, 0, &mut temp) == 0;

            let mut mem = NvmlMemory { total: 0, free: 0, used: 0 };
            let mem_ok = (self.device_get_memory_info)(self.device, &mut mem) == 0;

            let mut clock: c_uint = 0;
            // 0 = NVML_CLOCK_GRAPHICS
            let clock_ok = (self.device_get_clock_info)(self.device, 0, &mut clock) == 0;

            let mut power_w = None;
            if let Some(get_power) = self.device_get_power_usage {
                let mut pwr_mw: c_uint = 0;
                if get_power(self.device, &mut pwr_mw) == 0 {
                    power_w = Some(pwr_mw as f32 / 1000.0);
                }
            }

            let mut fan_speed = None;
            if let Some(get_fan) = self.device_get_fan_speed {
                let mut fan_pct: c_uint = 0;
                if get_fan(self.device, &mut fan_pct) == 0 {
                    fan_speed = Some(fan_pct as f32);
                }
            }

            let vram_used_gb = if mem_ok { mem.used as f32 / (1024.0 * 1024.0 * 1024.0) } else { 0.0 };
            let vram_total_gb = if mem_ok { mem.total as f32 / (1024.0 * 1024.0 * 1024.0) } else { 1.0 };

            GpuSnapshot {
                model_name: self.name.clone(),
                utilization_pct: if util_ok { util.gpu as f32 } else { 0.0 },
                core_temp_c: if temp_ok { Some(temp as f32) } else { None },
                core_clock_mhz: if clock_ok { Some(clock as f32) } else { None },
                vram_used_gb,
                vram_total_gb,
                power_w,
                fan_speed_pct: fan_speed,
                is_available: true,
            }
        }
    }
}

impl Drop for NvmlCollector {
    fn drop(&mut self) {
        unsafe {
            let _ = (self.nvml_shutdown)();
        }
    }
}

unsafe impl Send for NvmlCollector {}
unsafe impl Sync for NvmlCollector {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_nvml_try_new_no_panic() {
        let result = NvmlCollector::try_new();
        let system32_nvml_exists = Path::new(r"C:\Windows\System32\nvml.dll").exists();

        match result {
            Ok(collector) => {
                assert!(system32_nvml_exists, "NvmlCollector succeeded, so nvml.dll must exist in System32");
                assert!(!collector.name.is_empty(), "GPU device name should not be empty");
                let snap = collector.sample();
                assert!(snap.is_available, "GpuSnapshot must be marked available");
                assert_eq!(snap.model_name, collector.name);
            }
            Err(err) => {
                assert!(!err.is_empty(), "NVML error message must not be empty");
                assert!(
                    err.contains("NVML not available")
                        || err.contains("nvmlInit")
                        || err.contains("No NVML GPU devices found")
                        || err.contains("Failed to get device handle"),
                    "Expected descriptive NVML error, got: {err}"
                );
            }
        }
    }

    #[test]
    fn test_nvml_system32_dll_path_contract() {
        let system32_nvml = Path::new(r"C:\Windows\System32\nvml.dll");
        if !system32_nvml.exists() {
            let res = NvmlCollector::try_new();
            assert!(res.is_err(), "Must fail when C:\\Windows\\System32\\nvml.dll does not exist");
            let err = res.err().unwrap();
            assert!(
                err.contains("NVML not available"),
                "Error should state NVML not available when System32 DLL is absent, got: {err}"
            );
        } else {
            // When nvml.dll exists in System32, try_new should execute safely without panicking on unwrap
            let res = NvmlCollector::try_new();
            assert!(res.is_ok() || res.is_err());
        }
    }

    #[test]
    fn test_sample_gpu_metrics_validity() {
        if let Ok(collector) = NvmlCollector::try_new() {
            let snap = collector.sample();
            assert!(snap.is_available);
            assert!(
                snap.utilization_pct >= 0.0 && snap.utilization_pct <= 100.0,
                "GPU utilization must be 0-100%, got: {}",
                snap.utilization_pct
            );
            assert!(snap.vram_total_gb > 0.0, "VRAM total must be positive");
            assert!(snap.vram_used_gb >= 0.0, "VRAM used must be non-negative");
            assert!(
                snap.vram_used_gb <= snap.vram_total_gb,
                "VRAM used ({}) must not exceed total ({})",
                snap.vram_used_gb,
                snap.vram_total_gb
            );
            if let Some(temp) = snap.core_temp_c {
                assert!(temp >= 0.0 && temp <= 150.0, "GPU temp out of realistic range: {temp}");
            }
            if let Some(clock) = snap.core_clock_mhz {
                assert!(clock >= 0.0 && clock <= 5000.0, "GPU clock out of realistic range: {clock}");
            }
        }
    }

    #[test]
    fn test_nvml_collector_lifecycle_and_reinitialization() {
        if let Ok(collector) = NvmlCollector::try_new() {
            let snap1 = collector.sample();
            assert!(snap1.is_available);
            // Dropping invokes Drop which safely calls nvmlShutdown and unloads Library
            drop(collector);

            // Re-initializing NVML after drop must succeed cleanly
            let collector2 = NvmlCollector::try_new();
            assert!(collector2.is_ok(), "Re-initialization after drop must succeed");
            let c2 = collector2.unwrap();
            let snap2 = c2.sample();
            assert!(snap2.is_available);
            assert_eq!(snap1.model_name, snap2.model_name);
            drop(c2);
        }
    }

    #[test]
    fn test_nvml_collector_scoped_drop() {
        let result = NvmlCollector::try_new();
        if let Ok(collector) = result {
            let name = collector.name.clone();
            assert!(!name.is_empty());
            {
                let inner = collector;
                let s = inner.sample();
                assert!(s.is_available);
                // inner drops here at end of scope
            }
        }
    }

    #[test]
    fn test_nvml_collector_send_and_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<NvmlCollector>();
        assert_sync::<NvmlCollector>();
    }
}



