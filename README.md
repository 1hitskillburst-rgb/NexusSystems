# NEXUS / SYSTEMS --- PC Control Center

> A customizable Windows desktop application for monitoring PC
> performance, hardware health, and system activity in one polished
> dashboard.

**Project status:** Design prototype / planning stage\
**Current prototype:** `pc_dashboard_designs.html`\
**Intended final product:** A native Windows desktop application ---
**not a website**.

------------------------------------------------------------------------

## 1. Project vision

Build a personal PC monitoring application that brings the most useful
information from tools such as Windows Task Manager, HWMonitor, and MSI
Afterburner into one cohesive interface.

The application should make it easy to understand what the PC is doing
at a glance:

-   How hard the CPU and GPU are working.
-   How hot the CPU, GPU, and drives are.
-   How much RAM and VRAM are in use.
-   Which applications are consuming the most resources.
-   How much data is moving over the network.
-   How quickly storage drives are being read and written.
-   What the fans and other available hardware sensors are reporting.

The app should feel like a real desktop utility: fast, readable,
customizable, and reliable. The visual design is important, but accurate
telemetry and stability matter more than decorative effects.

### Core product principles

1.  **Native Windows application:** The final product must run as a
    desktop app, not as a browser-hosted website.
2.  **At-a-glance monitoring:** Important readings should be visible
    without navigating through multiple windows.
3.  **Live information:** Metrics and graphs should update continuously
    at sensible intervals.
4.  **Customizable presentation:** Users should be able to choose a
    theme and eventually arrange or hide dashboard widgets.
5.  **Honest telemetry:** Show unavailable sensors as unavailable. Never
    invent readings or silently substitute sample values.
6.  **Graceful operation:** Missing sensors, unsupported hardware, or
    permission limitations should not crash the application.
7.  **Efficient resource use:** Monitoring software should have a small
    CPU and memory footprint of its own.

------------------------------------------------------------------------

## 2. Current design prototype

The existing `pc_dashboard_designs.html` file is a **visual-only concept
prototype**. It demonstrates the dashboard layout and five visual
themes. It does not communicate with Windows, read hardware sensors, or
display live PC statistics.

All current readings are illustrative sample data. They must not be
mistaken for real hardware measurements.

### Prototype features

-   One dashboard layout shared by all five themes.
-   A theme selector at the bottom of the page.
-   A horizontal range slider numbered 1--5.
-   Five numbered buttons that select the corresponding theme.
-   Keyboard shortcuts `1`, `2`, `3`, `4`, and `5` to switch themes when
    focus is not inside a text input.
-   A small theme-change toast notification.
-   Responsive layout adjustments for narrower screens.
-   CSS custom properties used to change colors, borders, typography,
    and visual styling.

### Prototype layout

The mock dashboard currently contains:

  -----------------------------------------------------------------------
  Dashboard area                      What it displays in the mockup
  ----------------------------------- -----------------------------------
  Header                              NEXUS / SYSTEMS branding, PC
                                      CONTROL CENTER subtitle, status
                                      indicator, and icon button

  Overview                            Theme-specific eyebrow, main
                                      heading, and explanatory subtitle

  CPU / Processor                     Utilization ring, temperature,
                                      utilization trend, clock speed, and
                                      power

  GPU / Graphics                      Utilization ring, core temperature,
                                      clock speed, and VRAM

  Memory / RAM                        Utilization ring, used and
                                      available memory, memory speed, and
                                      committed memory

  Performance history                 CPU and GPU trend lines over a
                                      60-second illustrative window

  Storage / Drives                    Drive name, drive letter, capacity,
                                      used space, and sample temperature

  Top processes                       Application name, CPU use, and
                                      memory use

  Network                             Download speed, upload speed, ping,
                                      and adapter label

  Cooling / Fans                      CPU fan, GPU fan, and case-fan RPM
                                      with visual bars

  Footer                              Explicit concept-preview /
                                      sample-metrics notice

  Design selector                     Five-way theme slider and numbered
                                      theme buttons
  -----------------------------------------------------------------------

The prototype uses a 12-column dashboard grid. At medium widths, cards
rearrange into fewer columns; at small widths, cards stack into a single
column. Preserve this responsive behavior in the eventual desktop UI
where practical.

------------------------------------------------------------------------

## 3. The five visual design directions

All five themes should remain available as selectable appearance
options. They share the same core information architecture, but each
should have its own color palette, typography, shape language, and
atmosphere.

### Theme 1 --- Nebula

**Direction:** Futuristic gaming dashboard; the default theme.

-   Deep navy-black background.
-   Purple and violet accents.
-   Rounded cards and soft glow effects.
-   Bright, high-contrast metric values.
-   Modern sans-serif typography.
-   Gaming-oriented, polished, and slightly futuristic.

**Prototype palette:**

  Token              Color
  ------------------ -----------
  Background         `#090A12`
  Main panel         `#111321`
  Secondary panel    `#171A2B`
  Borders            `#282B42`
  Main text          `#F5F4FF`
  Muted text         `#9294AE`
  Accent             `#A78BFA`
  Secondary accent   `#6D5CE7`
  Positive status    `#65E6BD`
  Warning            `#FFBD69`
  Critical           `#FF718E`

### Theme 2 --- Clearspace

**Direction:** Clean, bright, minimalist, and professional.

-   Pale blue-gray background.
-   White and near-white cards.
-   Blue and cyan accents.
-   Dark slate text.
-   Subtle borders and soft shadows.
-   More spacious, rounded card shapes.

**Prototype palette:**

  Token              Color
  ------------------ -----------
  Background         `#E9EEF4`
  Main panel         `#F8FAFC`
  Secondary panel    `#EDF2F7`
  Borders            `#D9E1EB`
  Main text          `#1C2938`
  Muted text         `#718096`
  Accent             `#2889D8`
  Secondary accent   `#4AB3D8`
  Positive status    `#159D83`
  Warning            `#D58B28`
  Critical           `#D95C71`

### Theme 3 --- Forge

**Direction:** Industrial hardware telemetry; rugged and technical.

-   Dark brown-black background.
-   Warm amber and orange accents.
-   Cream-colored text.
-   More angular, compact cards.
-   Uppercase headings and monospaced labels.
-   Strong borders and a hardware-instrumentation feel.

**Prototype palette:**

  Token              Color
  ------------------ -----------
  Background         `#15100B`
  Main panel         `#211810`
  Secondary panel    `#2C2015`
  Borders            `#493322`
  Main text          `#FFF2DC`
  Muted text         `#B49B7A`
  Accent             `#FFAD42`
  Secondary accent   `#E66C28`
  Positive status    `#B7D36B`
  Warning            `#FFB347`
  Critical           `#F06A52`

### Theme 4 --- Terminal

**Direction:** Terminal / command-line inspired system monitor.

-   Very dark green-black background.
-   Bright green accents.
-   Monospaced typography throughout.
-   Squarer cards and compact spacing.
-   Subtle green glow on important readings.
-   Technical, data-dense, retro-computing atmosphere.

**Prototype palette:**

  Token              Color
  ------------------ -----------
  Background         `#07110D`
  Main panel         `#0B1C14`
  Secondary panel    `#10271B`
  Borders            `#1E4931`
  Main text          `#D9FFE8`
  Muted text         `#75A88A`
  Accent             `#64F59A`
  Secondary accent   `#1AB96B`
  Positive status    `#A4FFB8`
  Warning            `#F5D66E`
  Critical           `#FF7979`

### Theme 5 --- Aura

**Direction:** Elegant, soft, expressive, and visually distinctive.

-   Deep plum background.
-   Pink and violet accents.
-   Soft gradients and rounded cards.
-   Gentle contrast and refined typography.
-   A more atmospheric, personalized appearance.

**Prototype palette:**

  Token              Color
  ------------------ -----------
  Background         `#140E18`
  Main panel         `#211625`
  Secondary panel    `#2D1C31`
  Borders            `#4A3150`
  Main text          `#FFF0FA`
  Muted text         `#B494B1`
  Accent             `#F18ACB`
  Secondary accent   `#A77BFF`
  Positive status    `#82E3CF`
  Warning            `#FFC17A`
  Critical           `#FF729C`

**Important:** These are the prototype's starting palettes, not
restrictions. The final implementation may refine them, but all five
visual identities should remain recognizably different.

------------------------------------------------------------------------

## 4. Intended application features

The project should be built in stages. The following is the target
feature set, not a claim that these features already exist.

### 4.1 Hardware overview

#### CPU

Display as many of the following as the hardware and sensor library
support:

-   Overall CPU utilization.
-   Per-core utilization.
-   CPU package temperature.
-   Individual core temperatures, if available.
-   Current clock speed.
-   Per-core clock speeds, if available.
-   CPU package power, if available.
-   Fan speed associated with the CPU, if available.

#### GPU

Display:

-   GPU utilization.
-   GPU temperature.
-   GPU hotspot temperature, if available.
-   GPU memory-junction temperature, if available.
-   GPU core clock.
-   GPU memory clock, if available.
-   Dedicated VRAM used and total.
-   Shared GPU memory, if available.
-   GPU board power, if available.
-   GPU fan speed, if available.

GPU metrics can vary significantly by vendor, driver, and hardware.
Missing readings must be handled gracefully.

#### RAM

Display:

-   Total physical memory.
-   Used physical memory.
-   Available memory.
-   Memory utilization percentage.
-   Committed memory, where available.
-   Memory speed, only when a reliable source exposes it.

Do not assume that Windows exposes the RAM's configured speed through
the same source as current memory utilization.

#### Storage

For every detected drive, display:

-   Drive name or model, if available.
-   Drive letter or mount point.
-   Total capacity.
-   Used capacity.
-   Free capacity.
-   Read speed.
-   Write speed.
-   Drive temperature, if supported.
-   Drive type where it can be identified reliably (for example, SSD or
    HDD).

Distinguish **capacity usage** from **current read/write activity**.
They are different metrics.

#### Network

Display:

-   Current download throughput.
-   Current upload throughput.
-   Network adapter name.
-   Connection type, if detectable.
-   Optional historical throughput graph.

Ping/latency is not automatically available as a universal local network
counter. If implemented, make it an explicitly configured test to a
chosen endpoint and label it accordingly. Do not imply that an arbitrary
ping value is the user's internet latency.

#### Fans and cooling

Display available fan readings:

-   CPU fan RPM.
-   GPU fan RPM.
-   Case/system fan RPM.
-   Other motherboard fan headers, if exposed.

Fan sensors are not universal. Some systems do not expose all fan speeds
to third-party applications.

**Fan control is a separate, higher-risk feature.** The first version
should be read-only. Do not write fan speeds, voltage, or firmware
settings until a dedicated design, hardware compatibility plan, safety
limits, and recovery behavior exist.

### 4.2 Performance history and graphs

-   Live CPU utilization graph.
-   Live GPU utilization graph.
-   Optional temperature, memory, network, and storage graphs.
-   A rolling history window.
-   Potential time-range choices: 1 minute, 5 minutes, and 1 hour.
-   Clear legends, units, and timestamps.
-   A readable empty/unavailable state when a metric cannot be
    collected.

The graph should use real samples collected by the application. Avoid
generating decorative fake movement and presenting it as telemetry.

### 4.3 Process monitoring

A process table inspired by Task Manager:

-   Process/application name.
-   CPU utilization.
-   Memory usage.
-   Optional GPU utilization, where reliable per-process data is
    available.
-   Optional disk activity.
-   Sort by a selected column.
-   Search/filter process names.
-   Refresh automatically.

Process CPU readings should be calculated from process CPU-time deltas
over a known sampling interval, rather than treating lifetime CPU time
as current utilization.

For the initial release, process monitoring should be read-only.
Terminating processes or changing process priority can be considered
later, with confirmation and clear warnings.

### 4.4 Customization

Planned customization options:

-   Select one of the five themes.
-   Remember the selected theme after restart.
-   Show or hide dashboard widgets.
-   Reorder widgets.
-   Choose graph history duration.
-   Configure refresh rate within safe limits.
-   Set temperature warning thresholds.
-   Choose units and formatting where relevant.
-   Restore default layout/settings.

The first release can ship with a fixed dashboard layout and theme
selection. Widget rearrangement can come later.

### 4.5 Alerts

Potential alerts:

-   CPU temperature above a user-configured threshold.
-   GPU temperature above a user-configured threshold.
-   RAM usage above a user-configured threshold.
-   Drive space running low.
-   Sensor unavailable or lost.

Alerts should be configurable, avoid repeated notification spam, and
distinguish a warning from a critical condition. Thresholds should be
user-adjustable rather than hard-coded for every processor or GPU.

### 4.6 Future convenience features

These are stretch goals, not requirements for the first working version:

-   Windows system-tray mode.
-   Compact always-on-top mini-monitor.
-   Gaming overlay.
-   Launch on Windows startup.
-   Historical session logging.
-   Export telemetry to CSV or JSON.
-   Hardware summary page.
-   Optional per-widget refresh controls.

An in-game overlay may require additional integration and should be
treated as a separate feature from the main dashboard.

------------------------------------------------------------------------

## 5. Recommended technology

The final target is Windows, so use a desktop application framework
rather than shipping the HTML mockup as the final product.

### Recommended starting stack

  -----------------------------------------------------------------------
  Layer                   Recommendation          Responsibility
  ----------------------- ----------------------- -----------------------
  Language                C#                      Application logic and
                                                  data models

  Runtime                 Modern .NET, preferably Runtime and standard
                          a supported LTS release libraries

  Desktop UI              WPF for Windows-only    Native Windows desktop
                          development             interface

  Hardware sensors        LibreHardwareMonitor    Hardware sensor
                                                  discovery and readings

  OS metrics              .NET APIs and Windows   Processes, memory,
                          APIs                    disks, and network
                                                  counters

  Charts                  A WPF-compatible        Historical telemetry
                          charting library        graphs

  Settings                JSON configuration      Persist theme,
                          initially               thresholds, and
                                                  preferences

  Packaging               Windows desktop         Distributable
                          publishing / installer  application
  -----------------------------------------------------------------------

### Why WPF?

WPF is a practical choice for a Windows-only utility:

-   Mature desktop UI framework.
-   Good support for custom layouts and styling.
-   Works naturally with C# and .NET.
-   Supports data binding and reusable controls.
-   Suitable for a dashboard with cards, tables, and graphs.

Avalonia is a reasonable alternative if cross-platform support becomes a
real requirement. Do not build both UI frameworks. Choose one and keep
the application architecture clean.

### Hardware monitoring library

**LibreHardwareMonitor:**\
https://github.com/LibreHardwareMonitor/LibreHardwareMonitor

Use this as the first hardware-sensor integration to investigate. It can
expose supported hardware sensors such as temperatures, clocks, loads,
powers, and fan speeds depending on the machine.

Important caveats:

-   Not every motherboard, sensor chip, GPU, or drive exposes every
    metric.
-   Sensor labels and availability vary by hardware.
-   Some readings may require elevated permissions.
-   Some devices may require vendor-specific software or drivers.
-   A missing reading is not necessarily a hardware fault.
-   Check the library's current license, supported .NET versions, and
    redistribution requirements before shipping.

Do not promise that every metric will work on every PC.

------------------------------------------------------------------------

## 6. Suggested application architecture

Keep hardware collection separate from the UI. The dashboard should
consume a normalized stream of metric snapshots, not directly poll
hardware from individual visual controls.

A suggested high-level flow:

``` text
Hardware / Windows APIs
          |
          v
   Data Collectors
          |
          v
  Metric Normalization
          |
          v
   Telemetry Service
          |
          +------> History Buffer ------> Graphs
          |
          +------> Current Snapshot ----> Dashboard Widgets
          |
          +------> Alert Evaluator -----> Notifications
          |
          +------> Logger / Exporter ---> CSV or JSON
```

### Suggested project structure

``` text
NexusSystems/
|
+-- NexusSystems.sln
|
+-- src/
|   |
|   +-- NexusSystems.App/
|   |   +-- App.xaml
|   |   +-- MainWindow.xaml
|   |   +-- Views/
|   |   +-- ViewModels/
|   |   +-- Controls/
|   |   +-- Themes/
|   |   |   +-- Nebula.xaml
|   |   |   +-- Clearspace.xaml
|   |   |   +-- Forge.xaml
|   |   |   +-- Terminal.xaml
|   |   |   +-- Aura.xaml
|   |   +-- Assets/
|   |
|   +-- NexusSystems.Core/
|   |   +-- Models/
|   |   +-- Interfaces/
|   |   +-- Services/
|   |   +-- Configuration/
|   |
|   +-- NexusSystems.Monitoring/
|       +-- Hardware/
|       +-- Cpu/
|       +-- Gpu/
|       +-- Memory/
|       +-- Storage/
|       +-- Network/
|       +-- Processes/
|       +-- Cooling/
|
+-- tests/
|   +-- NexusSystems.Core.Tests/
|   +-- NexusSystems.Monitoring.Tests/
|
+-- docs/
|   +-- design/
|
+-- README.md
```

This is a suggested structure, not a requirement to create every folder
immediately. Start small and split components when there is enough code
to justify it.

### Architectural responsibilities

-   **App/UI:** Windows, navigation, dashboard layout, theme resources,
    user interactions.
-   **ViewModels:** Expose observable metric values and commands to the
    UI.
-   **Core:** Shared models, interfaces, settings, and
    application-independent logic.
-   **Monitoring:** Collect metrics from hardware and Windows.
-   **Telemetry service:** Coordinate sampling and publish snapshots.
-   **History buffer:** Keep a bounded amount of recent data for graphs.
-   **Alert evaluator:** Compare real readings with configured
    thresholds.
-   **Persistence:** Save user preferences and optional logs.

Use dependency injection or a similarly clear composition approach so
that the UI does not need to know which hardware library provides a
metric.

------------------------------------------------------------------------

## 7. Telemetry model and data handling

Normalize readings into consistent models. A metric should include its
value, unit, timestamp, source, and availability status.

Example conceptual model:

``` csharp
public sealed record MetricReading(
    string Name,
    double? Value,
    string Unit,
    DateTimeOffset Timestamp,
    string Source,
    bool IsAvailable
);
```

A more specialized model can be used for the dashboard snapshot:

``` csharp
public sealed record SystemSnapshot(
    DateTimeOffset Timestamp,
    CpuSnapshot Cpu,
    GpuSnapshot Gpu,
    MemorySnapshot Memory,
    IReadOnlyList<DriveSnapshot> Drives,
    NetworkSnapshot Network,
    IReadOnlyList<ProcessSnapshot> Processes,
    IReadOnlyList<FanSnapshot> Fans
);
```

These are starting examples, not a finalized API. Each specialized model
should clearly represent unavailable or unsupported values. Avoid using
`0` to mean "unknown" because zero can be a valid measurement.

### Sampling and performance

-   Start with a moderate sampling interval, such as 1 second for
    dashboard metrics.
-   Do not refresh every sensor as fast as possible.
-   Keep graph history bounded; do not store unlimited samples in
    memory.
-   Use background tasks for collection.
-   Keep blocking hardware calls off the UI thread.
-   Marshal UI updates through the framework's supported dispatcher
    mechanism.
-   Avoid creating a new timer or polling loop for every widget.
-   Handle collector exceptions so one failed sensor does not stop all
    monitoring.
-   Make the sampling interval configurable only within tested limits.

A central telemetry service should collect once and distribute the
resulting snapshot to all interested UI components.

### Units and correctness

-   CPU/GPU utilization: percent.
-   Temperatures: degrees Celsius initially; optionally support
    Fahrenheit later.
-   Clock speeds: MHz or GHz, consistently labeled.
-   Power: watts.
-   Fan speed: RPM.
-   Memory: clearly labeled GB/GiB or MB/MiB; do not mix units silently.
-   Network throughput: Mbps or MB/s, clearly distinguished.
-   Storage activity: MB/s or another clearly labeled throughput unit.

Document whether a reported memory quantity uses decimal GB or binary
GiB if precision matters.

------------------------------------------------------------------------

## 8. Development roadmap

Build a working application in small, testable milestones. Do not
attempt the entire feature list at once.

### Phase 0 --- Preserve and translate the design

-   Keep `pc_dashboard_designs.html` as the visual reference.
-   Choose a final application name and logo treatment.
-   Choose the default theme.
-   Recreate the layout in the selected desktop framework.
-   Implement the five themes using shared theme resources.
-   Confirm the dashboard scales correctly at common window sizes.

**Completion condition:** A native Windows window displays the dashboard
and switches between all five themes. Sample data is visibly labeled as
sample data.

### Phase 1 --- Application foundation

-   Create the solution and project structure.
-   Build the main window and dashboard navigation.
-   Add theme switching and persist the selected theme.
-   Create reusable metric cards, progress rings/bars, graph panels, and
    tables.
-   Create a mock telemetry provider for UI development.
-   Keep mock data and real data providers separate.

**Completion condition:** The UI is stable, attractive, and can be
tested without depending on hardware sensors.

### Phase 2 --- Real CPU, memory, and process metrics

-   Implement CPU utilization.
-   Implement total, used, and available physical memory.
-   Implement process enumeration and memory usage.
-   Calculate process CPU usage using sampling deltas.
-   Replace mock values with real values.
-   Show timestamps and unavailable states.

**Completion condition:** CPU, RAM, and process readings update from the
actual Windows machine.

### Phase 3 --- Hardware sensor integration

-   Integrate LibreHardwareMonitor.
-   Enumerate available hardware and sensors.
-   Identify CPU and GPU sensors.
-   Add temperatures, clocks, loads, and power where available.
-   Add drive and fan sensors where available.
-   Map raw sensor names into friendly dashboard labels.
-   Handle duplicate, missing, and unsupported sensors.

**Completion condition:** The dashboard displays actual supported
hardware readings and handles missing sensors without crashing.

### Phase 4 --- Storage and network

-   Enumerate drives and capacities.
-   Add read/write activity.
-   Add network adapter enumeration.
-   Calculate upload/download throughput from counter deltas.
-   Add optional ping testing as a separately labeled feature.

**Completion condition:** Storage and network cards show real values and
correct units.

### Phase 5 --- Graphs, history, and alerts

-   Build a bounded telemetry history buffer.
-   Add live CPU/GPU graphs.
-   Add optional temperature, memory, and network graphs.
-   Add time-range selection.
-   Add configurable warning thresholds.
-   Prevent repeated alert spam.

**Completion condition:** Graphs represent collected samples and alerts
behave predictably.

### Phase 6 --- Polish and reliability

-   Persist settings.
-   Add sensor refresh and error recovery.
-   Test startup, shutdown, and long-running behavior.
-   Measure the application's own CPU and memory overhead.
-   Improve accessibility and keyboard navigation.
-   Test multiple window sizes and display scaling.
-   Verify behavior without administrator permissions and document when
    elevation is needed.
-   Add logs useful for diagnosing sensor failures.

**Completion condition:** The application can run for extended periods
without freezing, leaking unbounded history, or crashing when a sensor
disappears.

### Phase 7 --- Optional advanced features

Only after the monitoring foundation is stable:

-   System tray mode.
-   Compact always-on-top monitor.
-   Gaming overlay.
-   CSV/JSON export.
-   Startup on login.
-   Custom widget layout.
-   Per-process GPU and disk metrics where reliable.
-   Fan control, only after a separate safety and hardware compatibility
    design.

------------------------------------------------------------------------

## 9. UI and interaction requirements

### Dashboard

-   Keep the most important CPU, GPU, and RAM readings near the top.
-   Make temperature and utilization immediately legible.
-   Keep units next to values.
-   Use color to communicate status, not as the only status indicator.
-   Avoid excessive animation.
-   Provide a clear unavailable state for unsupported readings.
-   Use consistent card spacing and typography within each theme.

### Themes

-   Theme selection should update the whole application, not only the
    background.
-   Theme colors should be defined centrally as resources/tokens.
-   Components should not hard-code colors that break theme switching.
-   Persist the user's selected theme.
-   Keep warning and critical states distinguishable in every theme.

### Process table

-   Support sorting.
-   Keep columns aligned.
-   Use readable units.
-   Avoid allowing long process names to break the layout.
-   Show a useful empty/loading/error state.

### Accessibility and usability

-   Support keyboard navigation.
-   Maintain readable contrast.
-   Do not rely on color alone to indicate warnings.
-   Scale cleanly with Windows display scaling.
-   Keep controls large enough to use comfortably.
-   Avoid unnecessary popups and notifications.

------------------------------------------------------------------------

## 10. Reliability, permissions, and safety

Hardware monitoring is not perfectly uniform across Windows PCs.

-   Some sensors are unavailable on certain motherboards or devices.
-   Some low-level sensor access may require administrator privileges.
-   Do not request administrator privileges for the entire application
    unless necessary.
-   Prefer a documented, optional elevated mode if a specific sensor
    requires it.
-   Never crash the dashboard because one sensor fails.
-   Do not assume a sensor's label or index is stable across machines.
-   Do not assume every PC has a discrete GPU, multiple drives, or
    controllable fans.
-   Do not silently show stale values as live. Include timestamps or
    mark stale readings.
-   Do not use fake fallback numbers for missing sensors.
-   Do not write to hardware controllers in the initial read-only
    release.
-   Avoid collecting or transmitting telemetry outside the machine. The
    initial app should work locally.

For any later fan-control feature, define safe limits, device-specific
compatibility checks, user confirmation, a way to restore automatic
control, and failure recovery before implementation.

------------------------------------------------------------------------

## 11. Testing checklist

### UI tests

-   [ ] All five themes load.
-   [ ] Theme selection persists after restart.
-   [ ] Cards resize without overlapping.
-   [ ] The dashboard works at narrow and wide window sizes.
-   [ ] Long process names do not break the table.
-   [ ] Missing values show an unavailable state.
-   [ ] Warning and critical states are readable in every theme.

### Monitoring tests

-   [ ] CPU utilization updates correctly.
-   [ ] CPU temperature is labeled with the correct sensor/source.
-   [ ] GPU readings are identified correctly when available.
-   [ ] RAM totals and used/available values are internally consistent.
-   [ ] Process CPU readings use a defined sampling interval.
-   [ ] Storage capacity is not confused with storage activity.
-   [ ] Network throughput uses counter deltas and correct units.
-   [ ] Unsupported fan and temperature sensors do not crash collection.
-   [ ] Sensor disconnection or collection errors recover gracefully.

### Reliability tests

-   [ ] App can run for an extended session.
-   [ ] History buffers remain bounded.
-   [ ] UI remains responsive during sensor collection.
-   [ ] One failing collector does not stop unrelated metrics.
-   [ ] App shuts down cleanly.
-   [ ] App behaves sensibly without administrator privileges.
-   [ ] Application overhead is measured and kept reasonable.

------------------------------------------------------------------------

## 12. Scope boundaries

### In scope for the first useful release

-   Native Windows desktop application.
-   Five visual themes.
-   CPU, GPU, and RAM overview.
-   Supported temperature, clock, load, power, and fan readings.
-   Storage capacity and activity.
-   Network throughput.
-   Process CPU and memory table.
-   Live performance graphs.
-   Settings persistence.
-   Graceful handling of missing sensors.

### Explicitly not required for the first release

-   Replacing every feature in Windows Task Manager.
-   Overclocking.
-   Voltage adjustment.
-   BIOS/UEFI configuration.
-   Fan-speed control.
-   GPU tuning.
-   Driver management.
-   Automatic process termination.
-   Cloud accounts or remote telemetry.
-   A web-hosted dashboard.

These can be reconsidered later, but they should not delay a reliable
read-only monitoring application.

------------------------------------------------------------------------

## 13. Definition of done

The first complete version is successful when:

1.  It runs as a native Windows desktop application.
2.  It has all five distinct themes: Nebula, Clearspace, Forge,
    Terminal, and Aura.
3.  The selected theme persists across restarts.
4.  CPU, GPU, and RAM readings come from real system sources where
    supported.
5.  Available temperatures, clocks, power, fan, storage, and network
    readings are shown with correct units.
6.  The process table displays real process information and correctly
    calculated CPU usage.
7.  Live graphs are based on actual collected samples.
8.  Missing or unsupported readings are clearly identified rather than
    fabricated.
9.  One sensor failure does not crash the application.
10. The app remains responsive and uses a bounded history buffer.
11. The application has been tested on the target Windows PC.
12. The app's own resource usage is reasonable.

------------------------------------------------------------------------

## 14. Starting instructions for whoever builds this

Start by inspecting `pc_dashboard_designs.html` and preserving it as the
visual reference. Do not replace the design with a generic monitoring
template.

Then:

1.  Create a Windows desktop solution using C# and WPF.
2.  Rebuild the current dashboard layout as native WPF controls.
3.  Implement the five themes as centralized theme resources.
4.  Add a mock telemetry provider so the interface can be developed
    independently.
5.  Create normalized metric models and a central telemetry service.
6.  Add real CPU, memory, and process collection first.
7.  Integrate LibreHardwareMonitor for supported hardware sensors.
8.  Add storage and network collection.
9.  Add history, graphs, settings, and alerts.
10. Test reliability and resource overhead before adding stretch
    features.

**Do not begin with fan control or a gaming overlay.** Get a dependable,
attractive, read-only monitoring dashboard working first.

The existing HTML prototype is a design reference, not the final
runtime. The final deliverable should be a Windows desktop application
with real telemetry.

------------------------------------------------------------------------

## 15. Reference

-   **Current visual prototype:** `pc_dashboard_designs.html`
-   **Hardware sensor library:**
    [LibreHardwareMonitor](https://github.com/LibreHardwareMonitor/LibreHardwareMonitor)
-   **Desktop UI direction:** C# / .NET / WPF
-   **Project identity in prototype:** NEXUS / SYSTEMS --- PC CONTROL
    CENTER
