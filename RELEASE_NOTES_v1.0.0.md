# NEXUS / SYSTEMS — Release Notes v1.0.0

**Release Tag:** `v1.0.0`  
**Release Name:** `Production-Ready Consumer Release`  
**Target Platform:** Windows 10 / Windows 11 (64-bit)  
**Binary Name:** `NexusSystems.exe`  
**Distribution Bundle:** `NexusSystems-v1.0.0-windows-x64.zip`  
**Date:** November 22, 2026  

---

## 🚀 Release Highlights

We are thrilled to announce the official **v1.0.0 production-ready release of NEXUS / SYSTEMS** — the fast, elegant, and ultra-lightweight PC Control Center and hardware telemetry monitor for Windows.

NEXUS / SYSTEMS replaces bloated, resource-heavy hardware monitoring utilities with a single, standalone native application written in pure Rust.

- **100% Privacy Guarantee:** Absolutely zero telemetry sent over the network. Zero analytics, zero cloud dependencies, zero accounts or logins. 0 bytes leave your machine.
- **Ultra-Lightweight Performance:** Consumes **under 30 MB of RAM** and **negligible idle CPU overhead (<0.2%)** with sub-second cold boot times.
- **Pure Native Desktop Application:** No Electron, no Chromium, no Node.js runtimes, and no WebView2 background services.
- **Consumer Double-Click Launch:** Compiled with `#![windows_subsystem = "windows"]` so everyday users simply double-click `NexusSystems.exe` to run—no black console windows or command prompts.
- **New Settings Cog (`⚙`):** Complete in-app configuration modal allowing customizable refresh intervals (500ms, 1s, 2s), dynamic card visibility toggles, and seamless persistence in `%APPDATA%\NexusSystems\nexus_settings.json`.
- **Authentic Hardware Instrumentation:** Direct NVIDIA NVML integration, SMBIOS Type 17 DDR RAM speed extraction, real Win32 ICMP gateway latency, storage volume metrics, and privacy-hardened top process delta rankings.

---

## 🌟 What's New in v1.0.0

### 1. Hardware Monitoring & Telemetry Engine
- **Processor Telemetry (CPU):**
  - Real-time CPU utilization % with dynamic peak-level indicators.
  - Active core clock frequencies (GHz) and logical core topology counts.
  - Thermal package readings (°C) with clean fallback handling.
  - Active Windows power profile detection (`BALANCED`, `HIGH PERFORMANCE`, `POWER SAVER`, etc.) via `powrprof.dll`.
  - Intelligent processor model formatting that strips redundant vendor boilerplate strings while preserving critical generation indicators.
- **Graphics Instrumentation (GPU):**
  - Direct NVIDIA NVML dynamic driver binding (`nvml.dll`) querying GPU core load %, core temperature, clock speeds, and dedicated VRAM allocation (used/total GB).
  - Universal graphics fallback for non-NVIDIA GPUs, integrated graphics, or systems without driver management tools.
- **Memory & RAM Subsystem:**
  - Real-time physical RAM usage (GB and utilization %).
  - Direct hardware SMBIOS Type 17 extraction via Win32 `GetSystemFirmwareTable` (zero administrator elevation required), displaying authentic DDR memory speeds in MT/s (MHz).
  - Windows virtual memory commitment charge tracking.
- **60-Second Rolling Performance History:**
  - Synchronized dual-trend real-time plot displaying continuous CPU and GPU loads across the preceding 60 seconds with anti-aliased curves and clamped width safety.
- **Storage Subsystem:**
  - Real-time disk capacity utilization %, free space, and mount-point mappings across all NVMe SSDs, SATA drives, and external storage devices.
- **Top Active Processes:**
  - High-frequency delta sampling calculating exact CPU % and memory footprints for top resource consumers.
  - **Privacy Hardening:** Restricts process inspection strictly to process names, PIDs, CPU usage, and memory usage—never touches environment variables, user command-line parameters, tokens, or personal files.
- **Network Throughput & Ping Latency:**
  - Dynamic real-time download and upload rate counters in Mbps.
  - Active network interface identification (Wi-Fi 6/6E/7, Gigabit Ethernet, Loopback).
  - Genuine Win32 ICMP echo latency probing via `iphlpapi.dll` with zero raw socket or administrative elevation requirements.
- **Cooling Telemetry:**
  - Real-time fan speed RPM reporting and segmented duty bars for CPU, GPU, and chassis cooling, with automatic detection for passive zero-RPM cooling modes.

### 2. Retro-Industrial Vintage User Interface
- **Signature Industrial Aesthetic:**
  - High-contrast Ivory chassis background (`#EAE6DC`) with crisp dark borders (`#282828`).
  - High-visibility vivid orange segmented LED bar gauges (`#FA5816` / `#FA5411`) delivering instantaneous legibility inspired by classic rackmount hardware instruments.
- **Smart Responsive Layout & Auto-Collapse:**
  - Dynamic multi-column responsive grid reflow.
  - When secondary cards are hidden or unpopulated, remaining cards dynamically expand to fill row space with **zero whitespace gaps**.
  - If all secondary cards are disabled, primary hardware cards seamlessly expand vertically.
- **Rock-Solid Visual Safety:**
  - String truncation safely parses UTF-8 boundaries by Unicode code points, eliminating panics or visual clipping on multi-byte characters (CJK, Cyrillic, emoji).
  - Enforced inner clip rects prevent widget overflow under custom DPI scaling.

### 3. Settings Modal (`⚙`)
- Accessible via the top-right header cog button.
- **Telemetry Refresh Rate:** Select between `500ms (Fast)`, `1s (Normal / Default)`, or `2s (Eco)`. The background telemetry thread dynamically adopts the new sampling interval without restart.
- **Secondary Cards Visibility:** Toggle individual dashboard cards (Storage Volumes, Top Processes, Network Throughput, Cooling Fans).
- **Auto-Collapse Empty Cards:** Toggle automatic removal of unpopulated sensor rows.
- **Reset to Defaults:** Instantly revert all preferences back to factory defaults.
- **User Profile Persistence:** Automatically reads and writes configuration to `%APPDATA%\NexusSystems\nexus_settings.json` with atomic write protection and legacy fallback compatibility.

### 4. CLI & Headless Audit Mode
- In addition to standard GUI execution, running `NexusSystems.exe --test-telemetry` attaches directly to the active parent console (`AttachConsole`), prints a full diagnostic snapshot of all system sensors, and exits immediately.

---

## 🔒 Architecture & Privacy Guarantee

Nexus Systems was engineered from the ground up to respect user privacy and system resources:

| Pillar | Architecture Guarantee |
| :--- | :--- |
| **Outbound Network Traffic** | **0 Bytes Sent.** No HTTP client, no web socket library, no DNS queries, no analytics pings, and no crash reporter beacons. |
| **Authentication & Accounts** | **None.** No login, no registration, no license keys, no telemetry identifier. |
| **System Modification** | **Read-Only.** Nexus Systems does not alter clock speeds, voltages, fan curves, or power schemes. |
| **Disk Storage** | **Minimal Local Config Only.** All telemetry operates in-memory; only UI preferences are persisted in `%APPDATA%\NexusSystems\nexus_settings.json`. |
| **Process Inspection** | **Confined Scope.** Only PID, process name, and CPU/RAM usage are queried. Command-line arguments, environment variables, and authentication tokens are strictly ignored. |

---

## 📊 Performance & Binary Metrics

The v1.0.0 release binary has been compiled with maximum release optimizations:
- **Optimization Level:** `opt-level = 3` (Maximum performance)
- **Link-Time Optimization (LTO):** `lto = true` (Full cross-crate link-time optimization)
- **Codegen Units:** `codegen-units = 1` (Optimal binary generation)
- **Symbol Stripping:** `strip = true` (Removes debug symbols, minimizing binary size)
- **Panic Strategy:** `panic = "abort"` (Removes unwinding tables)

| Metric | Measured Value |
| :--- | :--- |
| **Release Binary Size (`NexusSystems.exe`)** | **~3.7 MB** |
| **Distribution ZIP Bundle Size** | **~1.7 MB** |
| **RAM Consumption (Runtime)** | **< 30 MB** |
| **Idle CPU Utilization** | **< 0.2%** |
| **Startup Latency** | **< 150 ms** |
| **External Dependencies** | **Zero (Completely self-contained)** |

---

## 💻 System Requirements

- **Operating System:** Windows 10 (64-bit, Version 1903 or higher) or Windows 11 (64-bit)
- **Architecture:** x86_64
- **Processor:** 1.0 GHz or faster dual-core 64-bit processor
- **Memory (RAM):** 64 MB minimum available system RAM
- **Graphics Acceleration:** GPU supporting Direct3D 12, Direct3D 11, or OpenGL 3.3+
- **Display Resolution:** Minimum 960 × 440 pixels (Recommended: 1220 × 460 or higher)
- **User Permissions:** Standard user permissions (**Administrator rights are NOT required**)

---

## 📥 Download & Installation

### Option 1: Standalone Consumer ZIP (Recommended)
1. Download **`NexusSystems-v1.0.0-windows-x64.zip`** from the [GitHub Releases](https://github.com/1hitskillburst-rgb/NexusSystems/releases/tag/v1.0.0) section.
2. Extract the archive to any convenient directory (e.g. `C:\Users\<You>\Desktop` or `C:\Program Files\NexusSystems`).
3. Double-click **`NexusSystems.exe`** to immediately start monitoring.

> **Tip:** You can right-click `NexusSystems.exe` and select **Pin to taskbar** or **Pin to Start** for quick one-click access at any time!

### Option 2: Building from Source
Ensure you have the latest Rust toolchain installed from [https://rustup.rs](https://rustup.rs):

```powershell
# Clone the repository
git clone https://github.com/1hitskillburst-rgb/NexusSystems.git
cd NexusSystems

# Build the release executable
cargo build --release

# Run the compiled binary
.\target\release\nexus_systems.exe
```

---

## 📦 Distribution Package Contents

The `NexusSystems-v1.0.0-windows-x64.zip` release package includes:
- **`NexusSystems.exe`** — The fully optimized, standalone, zero-dependency 64-bit executable.
- **`README.md`** — Complete consumer and technical documentation.
- **`QuickStart.txt`** — Instant plain-text guide for quick reference.

---

## 🛠️ Verification & Checksums

| File | Type | SHA-256 Checksum |
| :--- | :--- | :--- |
| **`NexusSystems.exe`** | 64-bit Portable Executable | `CAB7F8093B7644F9BF5D4DA11AA052F5FF5BE36B6A60F8DEA9077ED16AF53A14` |
| **`NexusSystems-v1.0.0-windows-x64.zip`** | Compressed Archive | `3024B34771D69A28EFFA1EC3107B4656BE5797F1B0841E41F9B9DA8D64BA61C4` |

To verify the integrity of the downloaded executable on Windows PowerShell:
```powershell
Get-FileHash -Algorithm SHA256 .\NexusSystems.exe
```

---

## 🤝 Community & Support

- **Issue Tracker:** [GitHub Issues](https://github.com/1hitskillburst-rgb/NexusSystems/issues)
- **Repository:** [https://github.com/1hitskillburst-rgb/NexusSystems](https://github.com/1hitskillburst-rgb/NexusSystems)
- **Author:** [1hitskillburst-rgb](https://github.com/1hitskillburst-rgb)
- **License:** Licensed under the [MIT License](LICENSE).
