# NEXUS / SYSTEMS — PC Control Center

[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D6?logo=windows&logoColor=white)](https://github.com/1hitskillburst-rgb/NexusSystems)
[![Built with Rust](https://img.shields.io/badge/Language-Rust%202021-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Renderer](https://img.shields.io/badge/Engine-Direct3D%2012%20%2F%20wgpu-blueviolet)](https://github.com/1hitskillburst-rgb/NexusSystems)
[![License](https://img.shields.io/badge/License-MIT-brightgreen)](LICENSE)

> A fast, elegant, and lightweight native Windows desktop application for monitoring PC performance, hardware health, and system activity in one polished dashboard.

---

## ⚡ What is NEXUS / SYSTEMS?

**NEXUS / SYSTEMS** brings the most essential metrics from Windows Task Manager, HWMonitor, and hardware utilities into a single, cohesive desktop interface. 

Built from scratch in **Rust** with direct hardware acceleration, it runs as a pure standalone executable — **no web engines, no Electron, and zero background bloat**. It consumes under **30 MB of RAM** and negligible CPU overhead while delivering accurate, sub-second telemetry updates.

---

## 🖥️ Dashboard Features

### 1. At-a-Glance Hardware Overview
- **CPU / Processor:** Real-time utilization, clock speeds (GHz), core counts, and thermal readings for Intel Core and AMD Ryzen processors.
- **GPU / Graphics:** Real-time GPU load, core temperature, core clock speeds, and dedicated VRAM allocation via direct NVIDIA NVML integration with universal graphics fallback.
- **Memory / RAM:** Physical memory usage, active/available allocation, DDR speed rating, and Windows memory commitment.
- **Performance History:** Rolling 60-second dual-trend performance graph tracking CPU and GPU loads in real time with anti-aliased curves.

### 2. Activity & Subsystems
- **Storage Drives:** Real-time capacity utilization, free space, and mount-point temperatures across NVMe SSDs and external drives.
- **Top Processes:** Live ranking of the highest CPU- and memory-consuming processes calculated from sampling deltas.
- **Network Throughput:** Dynamic download/upload speed counters in Mbps, ping latency, and active network interface detection.
- **Cooling Telemetry:** Fan speed readouts (RPM) and visual segmented duty bars for CPU, GPU, and chassis cooling.

### 3. Retro-Industrial Segmented Instrumentation
All metrics feature custom segmented LED bar gauges inspired by precision hardware instruments, delivering instant legibility without distracting decorative animations.

---

## 🎨 Visual Themes

Switch themes instantly with number keys **`1`** through **`6`** or by clicking the theme selector pills in the header:

| Key | Theme | Aesthetic |
| :---: | :--- | :--- |
| **`1`** | **Vintage** *(Default)* | Retro-industrial ivory (`#EAE6DC`), crisp black borders, vivid orange segmented LED bars |
| **`2`** | **Nebula** | Futuristic deep navy gaming aesthetic (`#090A12`) with violet and neon cyan accents |
| **`3`** | **Clearspace** | Clean, minimalist light slate (`#E9EEF4`) designed for bright work environments |
| **`4`** | **Forge** | Rugged industrial amber (`#15100B`) inspired by avionics and test-bench hardware |
| **`5`** | **Terminal** | Classic retro hacker green phosphor display (`#07110D`) |
| **`6`** | **Aura** | Atmospheric plum-violet (`#140E18`) with soft pink accents |

*Your selected theme is automatically saved to `nexus_settings.json` and restored whenever the app launches.*

---

## 🚀 Getting Started

### Prerequisites
- **Operating System:** Windows 10 or Windows 11 (64-bit)
- **Rust Toolchain:** (Only needed if building from source) [rustup.rs](https://rustup.rs)

### Launching the Application

Clone the repository and run:

```powershell
# Clone the repository
git clone https://github.com/1hitskillburst-rgb/NexusSystems.git
cd NexusSystems

# Run in optimized release mode
cargo run --release
```

Or build the standalone binary:

```powershell
cargo build --release
```
The compiled, zero-dependency executable will be located at:
```text
.\target\release\nexus_systems.exe
```

### Headless Telemetry Audit
To run a quick command-line audit of your system's hardware telemetry without launching the graphical UI:

```powershell
cargo run -- --test-telemetry
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| :---: | :--- |
| **`1`** | Switch to **Vintage** theme |
| **`2`** | Switch to **Nebula** theme |
| **`3`** | Switch to **Clearspace** theme |
| **`4`** | Switch to **Forge** theme |
| **`5`** | Switch to **Terminal** theme |
| **`6`** | Switch to **Aura** theme |

---

## 🛡️ Privacy & Reliability

- **100% Local:** No telemetry is collected or sent over the network.
- **Read-Only Telemetry:** Nexus Systems does not modify hardware clocks, voltages, or fan curves.
- **Fault-Tolerant:** If a hardware sensor or GPU driver is not exposed, the interface shows clean fallback states without crashing.

---

## 👤 Author & Contributor

- **1hitskillburst-rgb** ([GitHub](https://github.com/1hitskillburst-rgb))
