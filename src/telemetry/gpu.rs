use std::ffi::c_void;
use std::os::raw::{c_char, c_uint};
use libloading::{Library, Symbol};
use crate::telemetry::models::GpuSnapshot;

type NvmlReturn = c_uint;
type NvmlDevice = *mut c_void;

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
    _lib: Library,
    device: NvmlDevice,
    pub name: String,
    device_get_utilization: Symbol<'static, unsafe extern "C" fn(NvmlDevice, *mut NvmlUtilization) -> NvmlReturn>,
    device_get_temperature: Symbol<'static, unsafe extern "C" fn(NvmlDevice, c_uint, *mut c_uint) -> NvmlReturn>,
    device_get_memory_info: Symbol<'static, unsafe extern "C" fn(NvmlDevice, *mut NvmlMemory) -> NvmlReturn>,
    device_get_clock_info: Symbol<'static, unsafe extern "C" fn(NvmlDevice, c_uint, *mut c_uint) -> NvmlReturn>,
    device_get_power_usage: Option<Symbol<'static, unsafe extern "C" fn(NvmlDevice, *mut c_uint) -> NvmlReturn>>,
    device_get_fan_speed: Option<Symbol<'static, unsafe extern "C" fn(NvmlDevice, *mut c_uint) -> NvmlReturn>>,
}

impl NvmlCollector {
    pub fn try_new() -> Result<Self, String> {
        let lib = unsafe {
            Library::new("nvml.dll")
                .or_else(|_| Library::new(r"C:\Windows\System32\nvml.dll"))
                .map_err(|e| format!("NVML not available: {e}"))?
        };

        // Leak library reference to allow static lifetime symbols
        let lib_ref: &'static Library = Box::leak(Box::new(lib));

        unsafe {
            let nvml_init: Symbol<unsafe extern "C" fn() -> NvmlReturn> = lib_ref
                .get(b"nvmlInit_v2\0")
                .or_else(|_| lib_ref.get(b"nvmlInit\0"))
                .map_err(|e| format!("nvmlInit missing: {e}"))?;

            let res = nvml_init();
            if res != 0 {
                return Err(format!("nvmlInit returned error code: {res}"));
            }

            let get_count: Symbol<unsafe extern "C" fn(*mut c_uint) -> NvmlReturn> = lib_ref
                .get(b"nvmlDeviceGetCount_v2\0")
                .or_else(|_| lib_ref.get(b"nvmlDeviceGetCount\0"))
                .map_err(|e| format!("nvmlDeviceGetCount missing: {e}"))?;

            let mut count: c_uint = 0;
            if get_count(&mut count) != 0 || count == 0 {
                return Err("No NVML GPU devices found".to_string());
            }

            let get_handle: Symbol<unsafe extern "C" fn(c_uint, *mut NvmlDevice) -> NvmlReturn> = lib_ref
                .get(b"nvmlDeviceGetHandleByIndex_v2\0")
                .or_else(|_| lib_ref.get(b"nvmlDeviceGetHandleByIndex\0"))
                .map_err(|e| format!("nvmlDeviceGetHandleByIndex missing: {e}"))?;

            let mut device: NvmlDevice = std::ptr::null_mut();
            if get_handle(0, &mut device) != 0 {
                return Err("Failed to get device handle 0".to_string());
            }

            let get_name: Symbol<unsafe extern "C" fn(NvmlDevice, *mut c_char, c_uint) -> NvmlReturn> = lib_ref
                .get(b"nvmlDeviceGetName\0")
                .map_err(|e| format!("nvmlDeviceGetName missing: {e}"))?;

            let mut name_buf = [0i8; 128];
            let name = if get_name(device, name_buf.as_mut_ptr(), 128) == 0 {
                let c_str = std::ffi::CStr::from_ptr(name_buf.as_ptr());
                c_str.to_string_lossy().into_owned()
            } else {
                "NVIDIA Graphics Device".to_string()
            };

            let device_get_utilization = lib_ref
                .get(b"nvmlDeviceGetUtilizationRates\0")
                .map_err(|e| format!("nvmlDeviceGetUtilizationRates missing: {e}"))?;

            let device_get_temperature = lib_ref
                .get(b"nvmlDeviceGetTemperature\0")
                .map_err(|e| format!("nvmlDeviceGetTemperature missing: {e}"))?;

            let device_get_memory_info = lib_ref
                .get(b"nvmlDeviceGetMemoryInfo\0")
                .map_err(|e| format!("nvmlDeviceGetMemoryInfo missing: {e}"))?;

            let device_get_clock_info = lib_ref
                .get(b"nvmlDeviceGetClockInfo\0")
                .map_err(|e| format!("nvmlDeviceGetClockInfo missing: {e}"))?;

            let device_get_power_usage = lib_ref.get(b"nvmlDeviceGetPowerUsage\0").ok();
            let device_get_fan_speed = lib_ref.get(b"nvmlDeviceGetFanSpeed\0").ok();

            Ok(Self {
                _lib: Library::new("nvml.dll").unwrap(), // placeholder struct field
                device,
                name,
                device_get_utilization,
                device_get_temperature,
                device_get_memory_info,
                device_get_clock_info,
                device_get_power_usage,
                device_get_fan_speed,
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
            if let Some(ref get_power) = self.device_get_power_usage {
                let mut pwr_mw: c_uint = 0;
                if get_power(self.device, &mut pwr_mw) == 0 {
                    power_w = Some(pwr_mw as f32 / 1000.0);
                }
            }

            let mut fan_speed = None;
            if let Some(ref get_fan) = self.device_get_fan_speed {
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

unsafe impl Send for NvmlCollector {}
unsafe impl Sync for NvmlCollector {}

