# NEXUS / SYSTEMS — PC Control Center

[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D6?logo=windows&logoColor=white)](https://github.com/1hitskillburst-rgb/NexusSystems)
[![Release](https://img.shields.io/badge/Release-v1.0.0-orange?logo=github)](https://github.com/1hitskillburst-rgb/NexusSystems/releases/tag/v1.0.0)
[![Built with Rust](https://img.shields.io/badge/Language-Rust%202021-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Renderer](https://img.shields.io/badge/Engine-Direct3D%2012%20%2F%20wgpu-blueviolet)](https://github.com/1hitskillburst-rgb/NexusSystems)
[![Memory](https://img.shields.io/badge/Memory-%3C%2030%20MB%20RAM-brightgreen)](https://github.com/1hitskillburst-rgb/NexusSystems)
[![Privacy](https://img.shields.io/badge/Privacy-100%25%20Local%20%2F%20Zero%20Telemetry-success)](https://github.com/1hitskillburst-rgb/NexusSystems)
[![License](https://img.shields.io/badge/License-MIT-brightgreen)](LICENSE)

> **A fast, elegant, and ultra-lightweight native Windows desktop application for monitoring PC performance, hardware health, and system activity in one polished retro-industrial dashboard.**

---

## ⚡ What is NEXUS / SYSTEMS?

**NEXUS / SYSTEMS** brings the most essential metrics from Windows Task Manager, HWMonitor, and hardware diagnostic utilities into a single, cohesive, distraction-free desktop interface.

Engineered from scratch in **pure Rust** with direct GPU hardware acceleration, Nexus Systems runs as a standalone native Windows desktop application:
- **No Web Engines & No Electron Bloat:** Zero Chromium runtimes, zero NodeJS background processes, and zero WebView2 overhead.
- **Instant Launch:** Launches in milliseconds.
- **Microscopic Footprint:** Consumes **under 30 MB of RAM** and **negligible idle CPU (<0.2%)**.
- **Double-Click Simplicity:** Just double-click `NexusSystems.exe`—no command prompt, no terminal window, and no setup wizard required.
- **On-Demand Performance Recording:** Integrated high-resolution telemetry recording session with 3-curve graph, interactive mouse hover inspection, and zero-leak discard.

```
+---------------------------------------------------------------------------------------------------+
|  NEXUS  PC Control Center       [⏺ START RECORDING]         OPTIMAL  1h 24m  09:00:00  BALANCED ⚙ |
+------------------------------------+----------------------------------+---------------------------+
| [ CPU: Core i9-13900K ]            | [ GPU: RTX 4090 ]                | [ RAM: DDR5 6000 MT/s ]   |
| 14.2%  4.80 GHz  32 Cores  42°C    | 28.5%  52°C  2550 MHz  16/24 GB  | 18.4 / 64.0 GB (28.7%)    |
| [||||||||                          | [|||||||||||||                   | [|||||||||||||            |
+------------------------------------+----------------------------------+---------------------------+
| [ LIVE | ● REC GRAPH ] (01:24 84p) | [ STORAGE DRIVES ]               | [ NETWORK & LATENCY ]     |
| 3-Curve CPU/GPU/RAM synced plot    | C:\ (NVMe) 420/1000 GB (42%)     | DL: 84.5 Mbps  UL: 12.1 M |
| Interactive hover pinpoint tooltip | D:\ (Data) 1.2/4.0 TB  (30%)     | Latency: 14 ms (ICMP)     |
+------------------------------------+----------------------------------+---------------------------+
| [ TOP ACTIVE PROCESSES ]           | [ COOLING & FAN TELEMETRY ]                                  |
| System Idle        85.4%     -- MB | CPU Fan: 1,240 RPM [||||||||||                               |
| nexus_systems.exe   0.2%     28 MB | GPU Fan:     0 RPM (Passive Zero-RPM Mode)                    |
| code.exe            1.4%    312 MB | Chassis:   850 RPM [||||||                                    |
+------------------------------------+--------------------------------------------------------------+
```

---

## 🛡️ 100% Privacy Guarantee

In an era of telemetry-heavy utilities and cloud accounts, Nexus Systems is built on a strict privacy-first foundation:

- **Zero Outbound Telemetry:** Exactly **0 bytes** of data ever leave your computer. The codebase contains zero HTTP, web socket, or telemetry networking dependencies.
- **No Cloud Services & No Accounts:** No registration, no login, no API keys, and no persistent online connection required.
- **100% Offline by Design:** Functions completely disconnected from the internet. The network throughput and latency card monitors local adapter metrics and direct ICMP gateway response entirely within your local OS environment.
- **Strict Privacy Isolation:** Process scanning is strictly confined to process names, PIDs, CPU deltas, and memory usage. Environment variables, user command-line parameters, tokens, and file system contents are never inspected.
- **Read-Only Telemetry:** Nexus Systems strictly reads hardware diagnostic registers. It does not alter system voltages, overclock frequencies, modify fan profiles, or flash firmware.

---

## 🚀 Double-Click Simplicity (Zero Terminal Required)

Nexus Systems is compiled specifically as a native Windows GUI application (`#![windows_subsystem = "windows"]`).

### For Regular Users (Consumers)
1. Download the standalone bundle: `NexusSystems-v1.0.0-windows-x64.zip`.
2. Extract the folder anywhere you like (e.g., Desktop or `Program Files`).
3. **Double-click `NexusSystems.exe`**.
4. The dashboard launches instantaneously in its own standalone window—**no black console window, no terminal prompts, and no command lines**.

### Clean Portability
- **Zero Installers:** No MSI, no InstallShield, no administrative registry modifications.
- **Zero Leftover Services:** No background daemons running when the window is closed.
- **User Settings:** Preferences are cleanly saved to `%APPDATA%\NexusSystems\nexus_settings.json` and can be reset at any time with one click.

---

## 🖥️ Dashboard Instrumentation

### 1. Primary Hardware Sensors
- **CPU / Processor:** Real-time CPU utilization %, live core clock frequencies (GHz), core/thread topology counts, thermal package readouts (°C), active Windows power profile (`BALANCED`, `HIGH PERFORMANCE`, etc.), and sanitized processor name formatting.
- **GPU / Graphics:** Real-time GPU core load %, core temperature (°C), live core clock speeds (MHz), and dedicated VRAM allocation (used/total GB). Powered by direct **NVIDIA NVML** dynamic driver binding with universal graphics fallback.
- **Memory / RAM:** Real-time physical RAM usage (GB and %), total system capacity, SMBIOS Type 17 DDR clock speed rating (MT/s or MHz queried directly without elevation), and Windows virtual memory commitment charge.
- **60-Second Rolling History:** High-contrast dual-trend synchronized line chart plotting continuous CPU and GPU loads across the last 60 seconds with anti-aliased curves.

### 2. Activity & Subsystems
- **Storage Drives:** Real-time capacity utilization, free space, and mount-point metrics across all NVMe SSDs, SATA drives, and external volumes with segmented capacity bars.
- **Top Active Processes:** Delta-sampled ranking of the most resource-intensive processes sorted by CPU % and memory footprint.
- **Network & Latency:** Dynamic download and upload throughput counters in Mbps, active network adapter identification (Wi-Fi 6/6E/7, Gigabit Ethernet), and genuine Win32 ICMP ping latency to your gateway.
- **Cooling Telemetry:** Fan speed readouts (RPM) and visual segmented duty bars for CPU, GPU, and chassis cooling. Features automatic detection of passive zero-RPM cooling modes.

### 3. Retro-Industrial Segmented Gauges
All performance stats feature custom segmented LED bar gauges inspired by classic high-end rackmount instruments. Gauges dynamically adjust color thresholds (Green/Ivory -> Amber -> Red Alert) to signal heavy hardware stress.

### 4. Performance Recording & Multi-Curve Telemetry
Nexus Systems includes an on-demand, high-resolution **Performance Recording** suite designed for benchmarking, gaming stress tests, and hardware diagnostics without requiring heavy third-party logging utilities.

```
+---------------------------------------------------------------------------------------------------+
|  [⏺ START RECORDING]  -->  [● REC 01:23.4 (84 pts)] [⏹ STOP] [🗑 DISCARD]                          |
|  [⏹ STOPPED REVIEW]   -->  [RECORDED 02:45.0 (165 pts)] [⏺ NEW REC] [🗑 DISCARD & DONE]            |
+---------------------------------------------------------------------------------------------------+
```

#### A. Vintage Recording Controls (Top Header next to Settings)
Directly accessible in the upper right of the main header, positioned immediately beside the **`⚙`** Settings cog:
- **`[⏺ REC]`**: Compact button next to Settings that initializes a fresh recording session, resets the monotonic timeline, and automatically transitions the chart panel into **`● REC GRAPH`** view.
- **`[● REC MM:SS.t (N pts)]`**: High-visibility retro recording indicator badge displaying live session elapsed time down to tenths of a second along with total collected data points. Automatically throttles UI paint events to ensure fluid, jitter-free timer rendering.
- **`[⏹ STOP]`**: Freezes the recorded data at the exact final duration, locking the timeline for in-depth mouse hover review and analysis.
- **`[⏺ NEW REC]`**: Instantly initiates a subsequent recording cycle, clearing previous data.
- **`[🗑 DISCARD]` / `[🗑 DISCARD & DONE]`**: Immediately purges the recording buffer and reclaims all allocated heap memory (see Discard & Zero Memory Leakage below).

#### B. Synchronized 3-Curve Graph
The recording graph visualizes the three primary compute metrics synchronized along a unified monotonic time axis:
- **CPU Load (Dark Chocolate / `palette.chart_line1`):** Primary processor package utilization %.
- **GPU Load (Warm Amber / `palette.chart_line2`):** Discrete graphics core compute utilization %.
- **RAM Load (Vintage Green / `palette.positive`):** Physical system memory utilization %.
- **Guide Grid:** 25%, 50%, and 75% subtle dotted horizontal gridlines provide immediate visual scale.
- **Time Calibration:** Monotonic bottom axis labels display relative elapsed time (`00:00`, `25%`, `50%`, `75%`, `End`).

#### C. Interactive Mouse Hover Inspection & Floating Tooltip Card
Hovering your mouse anywhere over the recording graph enables high-precision retrospective telemetry inspection:
- **Temporal Snapping:** Monotonically resolves the cursor's horizontal position to the exact nearest recorded telemetry snapshot.
- **Vertical Time Marker:** Renders a clean vertical hairline (`palette.bar_active`) across the chart at the inspected timestamp with an integrated time badge (e.g., `T +01:05.4`).
- **Synchronized Data Nodes:** Places highlighted indicator dots directly on the CPU, GPU, and RAM curves at the inspected point.
- **Retro Vintage Telemetry Card:** Pops up an inspection tooltip formatted in the signature industrial monospace style:
  - **Timestamp:** `T +MM:SS.t`
  - **CPU:** Utilization %, thermal package temperature (`65°C`), and core clock speed (`4.85 GHz`).
  - **GPU:** Utilization %, thermal core temperature (`68°C`), and core clock frequency (`2520 MHz`).
  - **RAM:** Utilization %, memory allocation in gigabytes (`14.2 / 31.9 GB`), and SMBIOS transfer speed (`6,000 MT/s`).

#### D. Lightweight Design & Zero Memory Leakage
The performance recording engine is engineered to strictly maintain Nexus Systems' microscopic footprint:
- **Compact In-Memory Footprint:** Each `RecordingPoint` struct requires only ~88 bytes of memory. An intensive 5-minute recording session at 1-second intervals consumes less than **30 KB** of RAM.
- **Zero Heap Bloat on Discard:** Unlike standard vector deallocations that preserve allocated capacity, clicking **`[🗑 DISCARD]`** or **`[🗑 DISCARD & DONE]`** triggers `samples.shrink_to_fit()`, immediately freeing **100% of heap capacity** back to the allocator. Memory capacity drops to strictly `0 bytes`.
- **Purely In-Memory / Zero Disk Overhead:** Data is held purely in RAM during active inspection. Discarding leaves zero residual files, zero temporary cache files, and zero disk writes.

---

## ⚙️ Customization & The Settings Cog

Click the **`⚙`** cog button in the upper-right corner of the header (or press **`Esc`** while open) to toggle the system preferences modal. Designed with 100% theme consistency matching the Vintage industrial styling (sharp 0px borders, monospace typography, retro ON/OFF pills, themed separators, and a focus-enhancing backdrop scrim):

| Setting | Options / Description |
| :--- | :--- |
| **Telemetry Refresh Rate** | **`500ms (Fast)`** (2 Hz sampling for intense gaming/benchmarks)<br>**`1s (Normal)`** (Default: optimal balance of responsiveness and 0% CPU)<br>**`2s (Eco)`** (0.5 Hz ultra-low resource profile) |
| **Secondary Cards Visibility** | Selectively toggle individual dashboard cards via retro industrial pills:<br>• **Storage Volumes (Drives)**<br>• **Top Processes (CPU/RAM)**<br>• **Network Throughput (DL/UL)**<br>• **Cooling & Fan Instrumentation** |
| **Auto-Collapse Unpopulated Cards** | Automatically removes empty sensor slots (e.g. absent fan sensors) with **zero whitespace gaps**—secondary cards dynamically expand to fill available space. |
| **Active Visual Theme** | Displays the active **Vintage** palette status (Retro-Industrial Ivory `#EAE6DC`, Deep Charcoal `#161616`, and Amber Alert `#FA5411`). |
| **Reset to Defaults** | Instantly restores all factory dashboard settings with one click. |

*All preferences are automatically serialized to `%APPDATA%\NexusSystems\nexus_settings.json` and restored across sessions.*

---

## 🏎️ Performance & Resource Comparison

| Metric | NEXUS / SYSTEMS | Typical Web / Electron Monitors | Heavy Vendor Suites |
| :--- | :---: | :---: | :---: |
| **Runtime Language** | **Native Rust** | JavaScript / Node.js | C++ / C# / Qt |
| **RAM Footprint** | **< 30 MB** | 250 MB – 600 MB | 150 MB – 400 MB |
| **Idle CPU Usage** | **~0.0% – 0.2%** | 1.5% – 5.0% | 1.0% – 3.0% |
| **Cold Startup Time** | **< 150 ms** | 2.5 s – 6.0 s | 3.0 s – 8.0 s |
| **Network Telemetry** | **0 Bytes (None)** | Telemetry beacons / Crashlytics | Cloud account sync / Ads |
| **Installer Required** | **No (Portable EXE)** | Yes (Squirrel / MSI) | Yes (Kernel drivers / Services) |
| **Background Services** | **Zero** | 1 – 3 background workers | 2 – 5 persistent services |

---

## 💻 System Requirements

- **Operating System:** Windows 10 (64-bit) or Windows 11 (64-bit)
- **Processor:** 64-bit Intel, AMD, or compatible x86_64 CPU
- **Memory:** 64 MB available RAM (Application uses < 30 MB)
- **Graphics:** DirectX 12, DirectX 11, or OpenGL 3.3+ compatible GPU
- **Permissions:** Standard user privileges (**Administrator rights are NOT required**)

---

## 📥 Download & Quick Start

### Option A: Standalone Release (Recommended for Consumers)
1. Navigate to the [Releases](https://github.com/1hitskillburst-rgb/NexusSystems/releases) page.
2. Download **`NexusSystems-v1.0.0-windows-x64.zip`**.
3. Extract the ZIP file and double-click **`NexusSystems.exe`**.

### Option B: Build from Source (For Developers)
Requires the [Rust toolchain](https://rustup.rs):

```powershell
# Clone the repository
git clone https://github.com/1hitskillburst-rgb/NexusSystems.git
cd NexusSystems

# Run in optimized release mode
cargo run --release

# Build standalone distribution binary
cargo build --release
```
The optimized executable will be located at:
```text
.\target\release\nexus_systems.exe
```

### Option C: Headless Telemetry Audit (CLI Mode)
For system administrators, CI runners, or diagnostic scripts, Nexus Systems supports a headless terminal audit mode:

```powershell
.\NexusSystems.exe --test-telemetry
```
This automatically attaches to your active console, outputs a clean summary of all hardware metrics, and exits immediately without launching the graphical UI.

---

## ⌨️ Shortcuts & Controls

| Control | Action |
| :---: | :--- |
| **`[⏺ START RECORDING]`** | Start new telemetry recording session and switch to recording graph |
| **`[⏹ STOP]`** | Freeze active recording session for retrospective review |
| **`[🗑 DISCARD]`** | Immediately purge all recorded data points and release heap memory |
| **`[LIVE] / [REC GRAPH]`** | Toggle between rolling 60s history and performance recording graph |
| **Mouse Hover on Chart** | Inspect nearest recorded sample with vertical hairline and vintage tooltip card |
| **`⚙`** *(Top Right)* | Open or close System Preferences modal |
| **`Alt + F4`** | Close Nexus Systems |
| **Window Resize** | Drag any window edge (enforces minimum bounds: 960 × 440 px) |

---

## 📄 License & Credits

- **Author:** [1hitskillburst-rgb](https://github.com/1hitskillburst-rgb)
- **License:** Licensed under the [MIT License](LICENSE).
- **Renderer:** Built using [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) and [egui](https://github.com/emilk/egui).
