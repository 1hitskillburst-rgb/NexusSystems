#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::telemetry::models::SystemSnapshot;

/// Current state of the performance recording session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RecordingState {
    #[default]
    Idle,
    Recording,
    Stopped,
}

/// A lightweight, single recorded telemetry data point.
///
/// Designed with zero redundant allocations to remain compact and cache-friendly
/// during long recording sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordingPoint {
    /// UTC timestamp of the snapshot in milliseconds since Unix epoch.
    pub timestamp_epoch_ms: i64,
    /// Elapsed seconds since the recording session started.
    pub elapsed_secs: f64,

    // CPU telemetry
    pub cpu_pct: f32,
    pub cpu_temp_c: Option<f32>,
    pub cpu_clock_ghz: f32,

    // GPU telemetry
    pub gpu_pct: f32,
    pub gpu_temp_c: Option<f32>,
    pub gpu_clock_mhz: Option<f32>,

    // RAM telemetry
    pub ram_pct: f32,
    pub ram_used_gb: f32,
    pub ram_total_gb: f32,
    pub ram_speed: String,
}

impl RecordingPoint {
    /// Constructs a `RecordingPoint` from a `SystemSnapshot` and calculated elapsed time.
    pub fn from_snapshot(snapshot: &SystemSnapshot, elapsed_secs: f64) -> Self {
        Self {
            timestamp_epoch_ms: snapshot.timestamp_epoch_ms,
            elapsed_secs,
            cpu_pct: snapshot.cpu.utilization_pct,
            cpu_temp_c: snapshot.cpu.package_temp_c,
            cpu_clock_ghz: snapshot.cpu.clock_speed_ghz,
            gpu_pct: snapshot.gpu.utilization_pct,
            gpu_temp_c: snapshot.gpu.core_temp_c,
            gpu_clock_mhz: snapshot.gpu.core_clock_mhz,
            ram_pct: snapshot.memory.utilization_pct,
            ram_used_gb: snapshot.memory.used_gb,
            ram_total_gb: snapshot.memory.total_gb,
            ram_speed: snapshot.memory.memory_speed.clone(),
        }
    }

    /// Formats CPU temperature as retro string (e.g. "65°C" or "--°C").
    pub fn format_cpu_temp(&self) -> String {
        self.cpu_temp_c
            .map(|t| format!("{:.0}°C", t))
            .unwrap_or_else(|| "--°C".to_string())
    }

    /// Formats CPU clock speed as string (e.g. "4.20 GHz").
    pub fn format_cpu_clock(&self) -> String {
        format!("{:.2} GHz", self.cpu_clock_ghz)
    }

    /// Formats GPU temperature as retro string (e.g. "72°C" or "--°C").
    pub fn format_gpu_temp(&self) -> String {
        self.gpu_temp_c
            .map(|t| format!("{:.0}°C", t))
            .unwrap_or_else(|| "--°C".to_string())
    }

    /// Formats GPU core clock speed as string (e.g. "2100 MHz" or "-- MHz").
    pub fn format_gpu_clock(&self) -> String {
        self.gpu_clock_mhz
            .map(|m| format!("{:.0} MHz", m))
            .unwrap_or_else(|| "-- MHz".to_string())
    }

    /// Formats RAM usage (e.g. "14.2 / 31.9 GB").
    pub fn format_ram_usage(&self) -> String {
        format!("{:.1} / {:.1} GB", self.ram_used_gb, self.ram_total_gb)
    }

    /// Formats elapsed time as MM:SS.t (e.g. "01:23.4").
    pub fn format_elapsed(&self) -> String {
        format_duration(self.elapsed_secs)
    }
}

/// Helper function to format seconds into retro MM:SS.t format.
pub fn format_duration(secs: f64) -> String {
    let total_secs = secs.max(0.0);
    let mins = (total_secs / 60.0).floor() as u64;
    let rem_secs = (total_secs % 60.0).floor() as u64;
    let tenths = ((total_secs % 1.0) * 10.0).floor() as u64;
    format!("{:02}:{:02}.{}", mins, rem_secs, tenths)
}

/// Helper function to format seconds into short MM:SS format.
pub fn format_duration_short(secs: f64) -> String {
    let total_secs = secs.max(0.0);
    let mins = (total_secs / 60.0).floor() as u64;
    let rem_secs = (total_secs % 60.0).floor() as u64;
    format!("{:02}:{:02}", mins, rem_secs)
}

/// Lightweight Performance Recording Session.
///
/// Collects and manages high-resolution telemetry points on demand.
/// When recording is active, snapshots are added; upon discard, memory is immediately
/// reclaimed with zero bloat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecordingSession {
    state: RecordingState,
    samples: Vec<RecordingPoint>,
    start_epoch_ms: Option<i64>,
    #[serde(skip)]
    start_instant: Option<Instant>,
    final_duration_secs: Option<f64>,
}

impl Default for PerformanceRecordingSession {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceRecordingSession {
    /// Creates a new idle recording session.
    pub fn new() -> Self {
        Self {
            state: RecordingState::Idle,
            samples: Vec::new(),
            start_epoch_ms: None,
            start_instant: None,
            final_duration_secs: None,
        }
    }

    /// Starts or restarts a recording session.
    ///
    /// Clears any previously retained samples and resets start markers.
    pub fn start(&mut self) {
        self.discard();
        self.state = RecordingState::Recording;
        self.start_instant = Some(Instant::now());
        self.start_epoch_ms = None;
        self.final_duration_secs = None;
    }

    /// Stops the recording session, freezing the recorded data for inspection and review.
    pub fn stop(&mut self) {
        if self.state == RecordingState::Recording {
            self.state = RecordingState::Stopped;
            let duration = self.samples.last().map(|p| p.elapsed_secs).unwrap_or(0.0);
            self.final_duration_secs = Some(duration);
        }
    }

    /// Immediately empties and frees all recorded samples (`shrink_to_fit`),
    /// returning the session to `Idle` with zero memory bloat.
    pub fn discard(&mut self) {
        self.samples.clear();
        self.samples.shrink_to_fit();
        self.state = RecordingState::Idle;
        self.start_epoch_ms = None;
        self.start_instant = None;
        self.final_duration_secs = None;
    }

    /// Returns `true` if no samples have been recorded.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns the number of recorded samples.
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Returns the current state of the session.
    pub fn state(&self) -> RecordingState {
        self.state
    }

    /// Returns `true` if the session is actively recording new samples.
    pub fn is_recording(&self) -> bool {
        self.state == RecordingState::Recording
    }

    /// Returns `true` if recording has stopped and is retained for inspection.
    pub fn is_stopped(&self) -> bool {
        self.state == RecordingState::Stopped
    }

    /// Returns `true` if the session is idle with no active recording.
    pub fn is_idle(&self) -> bool {
        self.state == RecordingState::Idle
    }

    /// Returns a slice of all recorded data points.
    pub fn samples(&self) -> &[RecordingPoint] {
        &self.samples
    }

    /// Returns the total duration of the recorded session in seconds.
    pub fn duration_secs(&self) -> f64 {
        match self.state {
            RecordingState::Idle => 0.0,
            RecordingState::Recording => {
                self.samples.last().map(|p| p.elapsed_secs).unwrap_or(0.0)
            }
            RecordingState::Stopped => {
                self.final_duration_secs.unwrap_or_else(|| {
                    self.samples.last().map(|p| p.elapsed_secs).unwrap_or(0.0)
                })
            }
        }
    }

    /// Returns live elapsed duration since recording was started (useful for live UI indicators).
    pub fn live_duration_secs(&self) -> f64 {
        if self.state == RecordingState::Recording {
            if let Some(inst) = self.start_instant {
                return inst.elapsed().as_secs_f64();
            }
        }
        self.duration_secs()
    }

    /// Adds a telemetry sample from `SystemSnapshot` if currently recording.
    ///
    /// If the session is `Idle` or `Stopped`, this call is a no-op.
    pub fn add_sample(&mut self, snapshot: &SystemSnapshot) {
        if self.state != RecordingState::Recording {
            return;
        }

        let elapsed_secs = if snapshot.timestamp_epoch_ms > 0 {
            let start_ms = *self.start_epoch_ms.get_or_insert(snapshot.timestamp_epoch_ms);
            if snapshot.timestamp_epoch_ms >= start_ms {
                (snapshot.timestamp_epoch_ms - start_ms) as f64 / 1000.0
            } else {
                self.samples.last().map(|p| p.elapsed_secs).unwrap_or(0.0)
            }
        } else if let Some(inst) = self.start_instant {
            inst.elapsed().as_secs_f64()
        } else {
            self.samples.last().map(|p| p.elapsed_secs).unwrap_or(0.0)
        };

        // Guarantee monotonicity in case of slight system timestamp jitter
        let monotonic_elapsed = if let Some(last) = self.samples.last() {
            elapsed_secs.max(last.elapsed_secs)
        } else {
            elapsed_secs
        };

        let point = RecordingPoint::from_snapshot(snapshot, monotonic_elapsed);
        self.samples.push(point);
    }

    /// Pinpoints the nearest recorded point to the given elapsed time.
    ///
    /// Useful for chart mouse hover inspection. Returns `None` if no samples exist.
    pub fn find_nearest_point(&self, elapsed_secs: f64) -> Option<&RecordingPoint> {
        if self.samples.is_empty() {
            return None;
        }

        let mut best = &self.samples[0];
        let mut min_diff = (best.elapsed_secs - elapsed_secs).abs();

        for pt in &self.samples[1..] {
            let diff = (pt.elapsed_secs - elapsed_secs).abs();
            if diff < min_diff {
                min_diff = diff;
                best = pt;
            } else if pt.elapsed_secs > elapsed_secs {
                // Since points are ordered chronologically, once diff starts growing after passing target, break early
                break;
            }
        }

        Some(best)
    }

    /// Returns the (min, max) CPU utilization percentage across recorded samples.
    pub fn cpu_range(&self) -> (f32, f32) {
        if self.samples.is_empty() {
            return (0.0, 0.0);
        }
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for p in &self.samples {
            min = min.min(p.cpu_pct);
            max = max.max(p.cpu_pct);
        }
        (min, max)
    }

    /// Returns the (min, max) GPU utilization percentage across recorded samples.
    pub fn gpu_range(&self) -> (f32, f32) {
        if self.samples.is_empty() {
            return (0.0, 0.0);
        }
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for p in &self.samples {
            min = min.min(p.gpu_pct);
            max = max.max(p.gpu_pct);
        }
        (min, max)
    }

    /// Returns the (min, max) RAM utilization percentage across recorded samples.
    pub fn ram_range(&self) -> (f32, f32) {
        if self.samples.is_empty() {
            return (0.0, 0.0);
        }
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for p in &self.samples {
            min = min.min(p.ram_pct);
            max = max.max(p.ram_pct);
        }
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::models::{CpuSnapshot, GpuSnapshot, MemorySnapshot};

    fn make_test_snapshot(epoch_ms: i64, cpu: f32, gpu: f32, ram: f32) -> SystemSnapshot {
        let mut snap = SystemSnapshot::default();
        snap.timestamp_epoch_ms = epoch_ms;
        snap.cpu = CpuSnapshot {
            model_name: "AMD Ryzen 9 7950X".to_string(),
            utilization_pct: cpu,
            package_temp_c: Some(65.4),
            clock_speed_ghz: 4.85,
            package_power_w: Some(105.0),
            core_count: 16,
            logical_core_count: 32,
            per_core_pct: vec![cpu; 32],
        };
        snap.gpu = GpuSnapshot {
            model_name: "NVIDIA GeForce RTX 4090".to_string(),
            utilization_pct: gpu,
            core_temp_c: Some(68.0),
            core_clock_mhz: Some(2520.0),
            vram_used_gb: 12.5,
            vram_total_gb: 24.0,
            power_w: Some(320.0),
            fan_speed_pct: Some(55.0),
            is_available: true,
        };
        snap.memory = MemorySnapshot {
            used_gb: (ram / 100.0) * 32.0,
            total_gb: 32.0,
            utilization_pct: ram,
            memory_speed: "6,000 MT/s".to_string(),
            committed_gb: 18.0,
            available_gb: 32.0 - (ram / 100.0) * 32.0,
        };
        snap
    }

    #[test]
    fn test_session_initial_state() {
        let session = PerformanceRecordingSession::new();
        assert_eq!(session.state(), RecordingState::Idle);
        assert!(session.is_idle());
        assert!(!session.is_recording());
        assert!(!session.is_stopped());
        assert!(session.is_empty());
        assert_eq!(session.sample_count(), 0);
        assert_eq!(session.duration_secs(), 0.0);
        assert_eq!(session.samples().len(), 0);
        assert_eq!(session.samples.capacity(), 0);
    }

    #[test]
    fn test_session_state_transitions() {
        let mut session = PerformanceRecordingSession::new();

        // Start recording
        session.start();
        assert_eq!(session.state(), RecordingState::Recording);
        assert!(session.is_recording());
        assert!(!session.is_idle());
        assert!(!session.is_stopped());

        // Stop recording
        session.stop();
        assert_eq!(session.state(), RecordingState::Stopped);
        assert!(session.is_stopped());
        assert!(!session.is_recording());
        assert!(!session.is_idle());

        // Restart recording from Stopped
        session.start();
        assert_eq!(session.state(), RecordingState::Recording);
        assert!(session.is_recording());

        // Discard
        session.discard();
        assert_eq!(session.state(), RecordingState::Idle);
        assert!(session.is_idle());
        assert!(!session.is_recording());
        assert!(!session.is_stopped());

        // Stop while idle is no-op
        session.stop();
        assert_eq!(session.state(), RecordingState::Idle);
    }

    #[test]
    fn test_add_sample_when_not_recording_is_ignored() {
        let mut session = PerformanceRecordingSession::new();
        let snap = make_test_snapshot(1000, 30.0, 50.0, 40.0);

        // In Idle state: should not record
        session.add_sample(&snap);
        assert!(session.is_empty());
        assert_eq!(session.sample_count(), 0);

        // Start and stop: now in Stopped state
        session.start();
        session.add_sample(&snap);
        assert_eq!(session.sample_count(), 1);

        session.stop();
        let snap2 = make_test_snapshot(2000, 35.0, 55.0, 45.0);
        session.add_sample(&snap2);
        // Still only 1 sample, snap2 was ignored
        assert_eq!(session.sample_count(), 1);
    }

    #[test]
    fn test_add_sample_captures_all_metrics() {
        let mut session = PerformanceRecordingSession::new();
        session.start();

        let snap1 = make_test_snapshot(1_700_000_000_000, 25.5, 60.0, 45.0);
        session.add_sample(&snap1);

        assert_eq!(session.sample_count(), 1);
        let pt0 = &session.samples()[0];
        assert_eq!(pt0.timestamp_epoch_ms, 1_700_000_000_000);
        assert_eq!(pt0.elapsed_secs, 0.0);
        assert_eq!(pt0.cpu_pct, 25.5);
        assert_eq!(pt0.cpu_temp_c, Some(65.4));
        assert_eq!(pt0.cpu_clock_ghz, 4.85);
        assert_eq!(pt0.gpu_pct, 60.0);
        assert_eq!(pt0.gpu_temp_c, Some(68.0));
        assert_eq!(pt0.gpu_clock_mhz, Some(2520.0));
        assert_eq!(pt0.ram_pct, 45.0);
        assert!((pt0.ram_used_gb - 14.4).abs() < 0.01);
        assert_eq!(pt0.ram_total_gb, 32.0);
        assert_eq!(pt0.ram_speed, "6,000 MT/s");

        // Second snapshot 2.5 seconds later
        let snap2 = make_test_snapshot(1_700_000_002_500, 55.0, 80.0, 50.0);
        session.add_sample(&snap2);

        assert_eq!(session.sample_count(), 2);
        let pt1 = &session.samples()[1];
        assert_eq!(pt1.timestamp_epoch_ms, 1_700_000_002_500);
        assert!((pt1.elapsed_secs - 2.5).abs() < 0.001);
        assert_eq!(pt1.cpu_pct, 55.0);
        assert_eq!(pt1.gpu_pct, 80.0);
        assert_eq!(pt1.ram_pct, 50.0);

        assert!((session.duration_secs() - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_memory_drop_upon_discard() {
        let mut session = PerformanceRecordingSession::new();
        session.start();

        // Feed 150 samples to cause substantial heap allocation
        for i in 0..150 {
            let snap = make_test_snapshot(1_700_000_000_000 + i * 1000, 20.0 + (i as f32 % 50.0), 30.0, 40.0);
            session.add_sample(&snap);
        }

        assert_eq!(session.sample_count(), 150);
        assert!(session.samples.capacity() >= 150);
        assert_eq!(session.duration_secs(), 149.0);

        // Discard: must immediately empty and free capacity (shrink_to_fit)
        session.discard();

        assert_eq!(session.state(), RecordingState::Idle);
        assert!(session.is_empty());
        assert_eq!(session.sample_count(), 0);
        assert_eq!(session.samples.capacity(), 0, "Capacity must be 0 after discard shrink_to_fit to eliminate memory bloat");
        assert_eq!(session.duration_secs(), 0.0);
    }

    #[test]
    fn test_start_clears_previous_session_data_and_frees_memory() {
        let mut session = PerformanceRecordingSession::new();
        session.start();

        for i in 0..50 {
            let snap = make_test_snapshot(1_000_000 + i * 1000, 50.0, 50.0, 50.0);
            session.add_sample(&snap);
        }
        assert_eq!(session.sample_count(), 50);

        // Starting a new recording must flush old data completely
        session.start();
        assert_eq!(session.sample_count(), 0);
        assert_eq!(session.samples.capacity(), 0);
        assert_eq!(session.duration_secs(), 0.0);
        assert_eq!(session.state(), RecordingState::Recording);
    }

    #[test]
    fn test_stop_freezes_duration() {
        let mut session = PerformanceRecordingSession::new();
        assert_eq!(session.live_duration_secs(), 0.0);

        session.start();
        assert!(session.live_duration_secs() >= 0.0);

        let snap1 = make_test_snapshot(1000, 10.0, 10.0, 10.0);
        let snap2 = make_test_snapshot(6000, 20.0, 20.0, 20.0);
        session.add_sample(&snap1);
        session.add_sample(&snap2);

        assert_eq!(session.duration_secs(), 5.0);

        session.stop();
        assert_eq!(session.state(), RecordingState::Stopped);
        assert_eq!(session.duration_secs(), 5.0);
        assert_eq!(session.live_duration_secs(), 5.0);
    }

    #[test]
    fn test_find_nearest_point() {
        let mut session = PerformanceRecordingSession::new();
        assert!(session.find_nearest_point(5.0).is_none());

        session.start();
        let timestamps = [0, 1000, 3000, 7000, 12000];
        for &ts in &timestamps {
            let snap = make_test_snapshot(100_000 + ts, 10.0, 10.0, 10.0);
            session.add_sample(&snap);
        }

        // Points are at elapsed: 0.0, 1.0, 3.0, 7.0, 12.0
        assert_eq!(session.find_nearest_point(0.2).unwrap().elapsed_secs, 0.0);
        assert_eq!(session.find_nearest_point(0.8).unwrap().elapsed_secs, 1.0);
        assert_eq!(session.find_nearest_point(2.1).unwrap().elapsed_secs, 3.0);
        assert_eq!(session.find_nearest_point(6.5).unwrap().elapsed_secs, 7.0);
        assert_eq!(session.find_nearest_point(10.0).unwrap().elapsed_secs, 12.0);
        assert_eq!(session.find_nearest_point(99.0).unwrap().elapsed_secs, 12.0);
    }

    #[test]
    fn test_monotonic_elapsed_time_on_timestamp_skew() {
        let mut session = PerformanceRecordingSession::new();
        session.start();

        // Snapshot 1: t=5000
        let snap1 = make_test_snapshot(5000, 20.0, 20.0, 20.0);
        session.add_sample(&snap1);
        assert_eq!(session.samples()[0].elapsed_secs, 0.0);

        // Snapshot 2: t=4000 (clock jumped back)
        let snap2 = make_test_snapshot(4000, 25.0, 25.0, 25.0);
        session.add_sample(&snap2);
        assert_eq!(session.samples()[1].elapsed_secs, 0.0); // Clamped monotonically

        // Snapshot 3: t=8000
        let snap3 = make_test_snapshot(8000, 30.0, 30.0, 30.0);
        session.add_sample(&snap3);
        assert_eq!(session.samples()[2].elapsed_secs, 3.0);
    }

    #[test]
    fn test_formatting_helpers() {
        let snap = make_test_snapshot(1000, 50.0, 60.0, 70.0);
        let pt = RecordingPoint::from_snapshot(&snap, 65.4);

        assert_eq!(pt.format_cpu_temp(), "65°C");
        assert_eq!(pt.format_cpu_clock(), "4.85 GHz");
        assert_eq!(pt.format_gpu_temp(), "68°C");
        assert_eq!(pt.format_gpu_clock(), "2520 MHz");
        assert_eq!(pt.format_ram_usage(), "22.4 / 32.0 GB");
        assert_eq!(pt.format_elapsed(), "01:05.4");
        assert_eq!(format_duration_short(65.4), "01:05");

        // Test None fallbacks
        let mut default_snap = SystemSnapshot::default();
        default_snap.cpu.package_temp_c = None;
        default_snap.gpu.core_temp_c = None;
        default_snap.gpu.core_clock_mhz = None;
        let pt_fallback = RecordingPoint::from_snapshot(&default_snap, 0.0);
        assert_eq!(pt_fallback.format_cpu_temp(), "--°C");
        assert_eq!(pt_fallback.format_gpu_temp(), "--°C");
        assert_eq!(pt_fallback.format_gpu_clock(), "-- MHz");
    }

    #[test]
    fn test_metric_ranges() {
        let mut session = PerformanceRecordingSession::new();
        assert_eq!(session.cpu_range(), (0.0, 0.0));
        assert_eq!(session.gpu_range(), (0.0, 0.0));
        assert_eq!(session.ram_range(), (0.0, 0.0));

        session.start();
        session.add_sample(&make_test_snapshot(1000, 15.0, 30.0, 45.0));
        session.add_sample(&make_test_snapshot(2000, 75.0, 90.0, 60.0));
        session.add_sample(&make_test_snapshot(3000, 40.0, 50.0, 50.0));

        assert_eq!(session.cpu_range(), (15.0, 75.0));
        assert_eq!(session.gpu_range(), (30.0, 90.0));
        assert_eq!(session.ram_range(), (45.0, 60.0));
    }

    #[test]
    fn test_multicycle_recording_lifecycle_and_zero_memory_leakage() {
        let mut session = PerformanceRecordingSession::new();
        assert_eq!(session.samples.capacity(), 0);

        // Cycle 1: 500 samples
        session.start();
        for i in 0..500 {
            let snap = make_test_snapshot(1_000_000 + i * 500, 10.0 + (i % 80) as f32, 20.0, 30.0);
            session.add_sample(&snap);
        }
        assert_eq!(session.sample_count(), 500);
        assert!(session.samples.capacity() >= 500);
        session.stop();
        assert!(session.is_stopped());
        assert!((session.duration_secs() - 249.5).abs() < 0.001);

        // Discard: immediate deallocation
        session.discard();
        assert_eq!(session.sample_count(), 0);
        assert_eq!(session.samples.capacity(), 0, "Capacity must be reset to 0 upon discard");
        assert_eq!(session.state(), RecordingState::Idle);

        // Cycle 2: 1,000 samples
        session.start();
        for i in 0..1000 {
            let snap = make_test_snapshot(2_000_000 + i * 250, 50.0, 60.0, 70.0);
            session.add_sample(&snap);
        }
        assert_eq!(session.sample_count(), 1000);
        assert!(session.samples.capacity() >= 1000);

        // Re-starting without stopping first must also cleanly reclaim old allocation
        session.start();
        assert_eq!(session.sample_count(), 0);
        assert_eq!(session.samples.capacity(), 0, "Restarting must flush old samples and reclaim heap memory");
        assert_eq!(session.state(), RecordingState::Recording);

        // Cycle 3: quick record then discard
        let snap = make_test_snapshot(3_000_000, 12.0, 34.0, 56.0);
        session.add_sample(&snap);
        assert_eq!(session.sample_count(), 1);
        session.discard();
        assert_eq!(session.samples.capacity(), 0);
        assert!(session.is_empty());
        assert!(session.is_idle());
    }
}
