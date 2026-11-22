use std::ffi::c_void;
use std::ptr;

// ============================================================================
// 1. SMBIOS Memory Speed Extraction (Type 17 - Memory Device)
// ============================================================================

#[link(name = "kernel32")]
extern "system" {
    fn GetSystemFirmwareTable(
        firmware_table_provider_signature: u32,
        firmware_table_id: u32,
        firmware_table_buffer: *mut u8,
        buffer_size: u32,
    ) -> u32;
}

const RSMB_PROVIDER: u32 = 0x52534D42; // 'RSMB' in big-endian

/// Queries the raw SMBIOS table via Win32 `GetSystemFirmwareTable` (zero elevation required)
/// and extracts the configured memory clock speed in MT/s (MHz) from Type 17 (Memory Device) records.
pub fn query_smbios_memory_speed() -> Option<u32> {
    unsafe {
        // First call: query required buffer size
        let buf_size = GetSystemFirmwareTable(RSMB_PROVIDER, 0, ptr::null_mut(), 0);
        if buf_size == 0 || buf_size > 1_000_000 {
            return None;
        }

        let mut buffer = vec![0u8; buf_size as usize];
        let bytes_read = GetSystemFirmwareTable(RSMB_PROVIDER, 0, buffer.as_mut_ptr(), buf_size);
        if bytes_read == 0 || bytes_read > buf_size {
            return None;
        }

        // SMBIOS table header: 8 bytes
        // byte 0: Used20CallingMethod
        // byte 1: MajorVersion
        // byte 2: MinorVersion
        // byte 3: DmiRevision
        // bytes 4..8: Length (u32)
        if buffer.len() < 8 {
            return None;
        }

        let mut offset = 8;
        let mut max_speed = 0u32;

        while offset + 4 <= buffer.len() {
            let struct_type = buffer[offset];
            let struct_len = buffer[offset + 1] as usize;

            if struct_len < 4 || offset + struct_len > buffer.len() {
                break;
            }

            // Type 17 is SMBIOS Memory Device
            if struct_type == 17 && struct_len >= 0x22 {
                // Offset 0x20 in Type 17 is ConfiguredMemoryClockSpeed (u16 little-endian)
                let speed_bytes = [buffer[offset + 0x20], buffer[offset + 0x21]];
                let speed = u16::from_le_bytes(speed_bytes) as u32;
                if speed > 0 && speed < 20_000 {
                    max_speed = max_speed.max(speed);
                }
            }

            // Move past the formatted structure
            offset += struct_len;

            // Move past the string table (ends with double null: 0x00 0x00)
            while offset + 1 < buffer.len() {
                if buffer[offset] == 0 && buffer[offset + 1] == 0 {
                    offset += 2;
                    break;
                }
                offset += 1;
            }
        }

        if max_speed > 0 {
            Some(max_speed)
        } else {
            None
        }
    }
}

// ============================================================================
// 2. Win32 ICMP Ping / Latency Probing (iphlpapi.dll)
// ============================================================================

type IcmpHandle = *mut c_void;

#[repr(C)]
struct IcmpEchoReply {
    address: u32,
    status: u32,
    round_trip_time: u32,
    data_size: u16,
    reserved: u16,
    data: *mut c_void,
    options: [u8; 8],
}

#[link(name = "iphlpapi")]
extern "system" {
    fn IcmpCreateFile() -> IcmpHandle;
    fn IcmpCloseHandle(handle: IcmpHandle) -> i32;
    fn IcmpSendEcho(
        handle: IcmpHandle,
        dest_address: u32,
        request_data: *const u8,
        request_size: u16,
        request_options: *const c_void,
        reply_buffer: *mut u8,
        reply_size: u32,
        timeout: u32,
    ) -> u32;
}

/// Sends an ICMP echo request using standard Win32 IP Helper API (zero elevation required)
/// and measures network latency in milliseconds.
pub fn query_network_ping() -> Option<u32> {
    unsafe {
        let handle = IcmpCreateFile();
        if handle.is_null() || handle == (usize::MAX as IcmpHandle) {
            return None;
        }

        // Target: 1.1.1.1 (Cloudflare DNS) - in network/host byte order: 0x01010101
        let ip_bytes: [u8; 4] = [1, 1, 1, 1];
        let dest_addr = u32::from_ne_bytes(ip_bytes);

        let send_data = b"nexus_probe";
        let mut reply_buf = [0u8; 128];

        let ret = IcmpSendEcho(
            handle,
            dest_addr,
            send_data.as_ptr(),
            send_data.len() as u16,
            ptr::null(),
            reply_buf.as_mut_ptr(),
            reply_buf.len() as u32,
            500, // 500ms timeout
        );

        let _ = IcmpCloseHandle(handle);

        if ret > 0 && reply_buf.len() >= std::mem::size_of::<IcmpEchoReply>() {
            let reply = &*(reply_buf.as_ptr() as *const IcmpEchoReply);
            if reply.status == 0 {
                return Some(reply.round_trip_time);
            }
        }

        None
    }
}

// ============================================================================
// 3. Win32 Active Power Profile Detection (powrprof.dll)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WinGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[link(name = "powrprof")]
extern "system" {
    fn PowerGetActiveScheme(
        user_root_power_key: *const c_void,
        active_policy_guid: *mut *mut WinGuid,
    ) -> u32;
}

#[link(name = "kernel32")]
extern "system" {
    fn LocalFree(hmem: *mut c_void) -> *mut c_void;
}

// Standard Windows Scheme GUIDs
const GUID_BALANCED: WinGuid = WinGuid {
    data1: 0x381b4222,
    data2: 0xf694,
    data3: 0x41f0,
    data4: [0x96, 0x85, 0xff, 0x5b, 0xb2, 0x60, 0xdf, 0x2e],
};

const GUID_HIGH_PERFORMANCE: WinGuid = WinGuid {
    data1: 0x8c5e7fda,
    data2: 0xe8bf,
    data3: 0x4a96,
    data4: [0x9a, 0x85, 0xa6, 0xe2, 0x3a, 0x8c, 0x63, 0x5c],
};

const GUID_POWER_SAVER: WinGuid = WinGuid {
    data1: 0xa1841308,
    data2: 0x3541,
    data3: 0x4fab,
    data4: [0xbc, 0x81, 0xf7, 0x15, 0x56, 0xf2, 0x0b, 0x4a],
};

const GUID_ULTIMATE_PERFORMANCE: WinGuid = WinGuid {
    data1: 0xe9a42b02,
    data2: 0xd5df,
    data3: 0x448d,
    data4: [0xaa, 0x00, 0x03, 0xf1, 0x47, 0x49, 0xeb, 0x61],
};

/// Queries the active Windows power scheme dynamically via `PowerGetActiveScheme`.
pub fn query_active_power_scheme() -> String {
    unsafe {
        let mut guid_ptr: *mut WinGuid = ptr::null_mut();
        let res = PowerGetActiveScheme(ptr::null(), &mut guid_ptr);
        if res == 0 && !guid_ptr.is_null() {
            let guid = *guid_ptr;
            let _ = LocalFree(guid_ptr as *mut c_void);

            if guid == GUID_HIGH_PERFORMANCE {
                "Profile: High Performance".to_string()
            } else if guid == GUID_ULTIMATE_PERFORMANCE {
                "Profile: Ultimate".to_string()
            } else if guid == GUID_POWER_SAVER {
                "Profile: Power Saver".to_string()
            } else if guid == GUID_BALANCED {
                "Profile: Balanced".to_string()
            } else {
                "Profile: Active".to_string()
            }
        } else {
            "Profile: Balanced".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smbios_query_does_not_panic() {
        let speed = query_smbios_memory_speed();
        println!("--> SMBIOS RAM Speed: {:?}", speed);
        if let Some(spd) = speed {
            assert!(spd > 0 && spd < 20_000, "SMBIOS speed should be reasonable MT/s: {}", spd);
        }
    }

    #[test]
    fn test_power_scheme_returns_valid_string() {
        let profile = query_active_power_scheme();
        println!("--> Active Power Profile: {}", profile);
        assert!(profile.starts_with("Profile:"), "Profile string must start with 'Profile:': {}", profile);
    }

    #[test]
    fn test_ping_query_does_not_panic() {
        let ping = query_network_ping();
        println!("--> Measured Ping: {:?}", ping);
    }
}
