use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Vintage,    // Reference Image: Retro Industrial Ivory & Orange
    Nebula,     // 1. Futuristic Gaming Dark
    Clearspace, // 2. Clean Minimal Slate
    Forge,      // 3. Industrial Amber
    Terminal,   // 4. Retro Terminal Green
    Aura,       // 5. Elegant Plum / Violet
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Vintage
    }
}

impl ThemeMode {
    pub fn all() -> &'static [ThemeMode] {
        &[
            ThemeMode::Vintage,
            ThemeMode::Nebula,
            ThemeMode::Clearspace,
            ThemeMode::Forge,
            ThemeMode::Terminal,
            ThemeMode::Aura,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThemeMode::Vintage => "Vintage (Reference)",
            ThemeMode::Nebula => "Nebula",
            ThemeMode::Clearspace => "Clearspace",
            ThemeMode::Forge => "Forge",
            ThemeMode::Terminal => "Terminal",
            ThemeMode::Aura => "Aura",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub background: Color32,
    pub panel_bg: Color32,
    pub card_bg: Color32,
    pub border: Color32,
    pub text_main: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub secondary_accent: Color32,
    pub bar_active: Color32,
    pub bar_track: Color32,
    pub chart_line1: Color32,
    pub chart_line2: Color32,
    pub positive: Color32,
    pub warning: Color32,
    pub critical: Color32,
}

impl ThemePalette {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Vintage => Self {
                background: Color32::from_rgb(0xEA, 0xE6, 0xDC),
                panel_bg: Color32::from_rgb(0xEA, 0xE6, 0xDC),
                card_bg: Color32::from_rgb(0xEA, 0xE6, 0xDC),
                border: Color32::from_rgb(0x28, 0x28, 0x28),
                text_main: Color32::from_rgb(0x16, 0x16, 0x16),
                text_muted: Color32::from_rgb(0x5A, 0x58, 0x52),
                accent: Color32::from_rgb(0xFA, 0x58, 0x16),
                secondary_accent: Color32::from_rgb(0x28, 0x28, 0x28),
                bar_active: Color32::from_rgb(0xFA, 0x54, 0x11),
                bar_track: Color32::from_rgb(0xDF, 0xDA, 0xCF),
                chart_line1: Color32::from_rgb(0x22, 0x22, 0x22), // CPU (black)
                chart_line2: Color32::from_rgb(0xFA, 0x54, 0x11), // GPU (orange)
                positive: Color32::from_rgb(0x15, 0x8C, 0x48),
                warning: Color32::from_rgb(0xD9, 0x62, 0x12),
                critical: Color32::from_rgb(0xD9, 0x28, 0x38),
            },
            ThemeMode::Nebula => Self {
                background: Color32::from_rgb(0x09, 0x0A, 0x12),
                panel_bg: Color32::from_rgb(0x11, 0x13, 0x21),
                card_bg: Color32::from_rgb(0x17, 0x1A, 0x2B),
                border: Color32::from_rgb(0x28, 0x2B, 0x42),
                text_main: Color32::from_rgb(0xF5, 0xF4, 0xFF),
                text_muted: Color32::from_rgb(0x92, 0x94, 0xAE),
                accent: Color32::from_rgb(0xA7, 0x8B, 0xFA),
                secondary_accent: Color32::from_rgb(0x6D, 0x5C, 0xE7),
                bar_active: Color32::from_rgb(0xA7, 0x8B, 0xFA),
                bar_track: Color32::from_rgb(0x1F, 0x22, 0x38),
                chart_line1: Color32::from_rgb(0xA7, 0x8B, 0xFA),
                chart_line2: Color32::from_rgb(0x65, 0xE6, 0xBD),
                positive: Color32::from_rgb(0x65, 0xE6, 0xBD),
                warning: Color32::from_rgb(0xFF, 0xBD, 0x69),
                critical: Color32::from_rgb(0xFF, 0x71, 0x8E),
            },
            ThemeMode::Clearspace => Self {
                background: Color32::from_rgb(0xE9, 0xEE, 0xF4),
                panel_bg: Color32::from_rgb(0xF8, 0xFA, 0xFC),
                card_bg: Color32::from_rgb(0xED, 0xF2, 0xF7),
                border: Color32::from_rgb(0xD9, 0xE1, 0xEB),
                text_main: Color32::from_rgb(0x1C, 0x29, 0x38),
                text_muted: Color32::from_rgb(0x71, 0x80, 0x96),
                accent: Color32::from_rgb(0x28, 0x89, 0xD8),
                secondary_accent: Color32::from_rgb(0x4A, 0xB3, 0xD8),
                bar_active: Color32::from_rgb(0x28, 0x89, 0xD8),
                bar_track: Color32::from_rgb(0xD5, 0xE0, 0xED),
                chart_line1: Color32::from_rgb(0x28, 0x89, 0xD8),
                chart_line2: Color32::from_rgb(0x15, 0x9D, 0x83),
                positive: Color32::from_rgb(0x15, 0x9D, 0x83),
                warning: Color32::from_rgb(0xD5, 0x8B, 0x28),
                critical: Color32::from_rgb(0xD9, 0x5C, 0x71),
            },
            ThemeMode::Forge => Self {
                background: Color32::from_rgb(0x15, 0x10, 0x0B),
                panel_bg: Color32::from_rgb(0x21, 0x18, 0x10),
                card_bg: Color32::from_rgb(0x2C, 0x20, 0x15),
                border: Color32::from_rgb(0x49, 0x33, 0x22),
                text_main: Color32::from_rgb(0xFF, 0xF2, 0xDC),
                text_muted: Color32::from_rgb(0xB4, 0x9B, 0x7A),
                accent: Color32::from_rgb(0xFF, 0xAD, 0x42),
                secondary_accent: Color32::from_rgb(0xE6, 0x6C, 0x28),
                bar_active: Color32::from_rgb(0xFF, 0xAD, 0x42),
                bar_track: Color32::from_rgb(0x38, 0x28, 0x1A),
                chart_line1: Color32::from_rgb(0xFF, 0xAD, 0x42),
                chart_line2: Color32::from_rgb(0xE6, 0x6C, 0x28),
                positive: Color32::from_rgb(0xB7, 0xD3, 0x6B),
                warning: Color32::from_rgb(0xFF, 0xB3, 0x47),
                critical: Color32::from_rgb(0xF0, 0x6A, 0x52),
            },
            ThemeMode::Terminal => Self {
                background: Color32::from_rgb(0x07, 0x11, 0x0D),
                panel_bg: Color32::from_rgb(0x0B, 0x1C, 0x14),
                card_bg: Color32::from_rgb(0x10, 0x27, 0x1B),
                border: Color32::from_rgb(0x1E, 0x49, 0x31),
                text_main: Color32::from_rgb(0xD9, 0xFF, 0xE8),
                text_muted: Color32::from_rgb(0x75, 0xA8, 0x8A),
                accent: Color32::from_rgb(0x64, 0xF5, 0x9A),
                secondary_accent: Color32::from_rgb(0x1A, 0xB9, 0x6B),
                bar_active: Color32::from_rgb(0x64, 0xF5, 0x9A),
                bar_track: Color32::from_rgb(0x14, 0x33, 0x23),
                chart_line1: Color32::from_rgb(0x64, 0xF5, 0x9A),
                chart_line2: Color32::from_rgb(0xA4, 0xFF, 0xB8),
                positive: Color32::from_rgb(0xA4, 0xFF, 0xB8),
                warning: Color32::from_rgb(0xF5, 0xD6, 0x6E),
                critical: Color32::from_rgb(0xFF, 0x79, 0x79),
            },
            ThemeMode::Aura => Self {
                background: Color32::from_rgb(0x14, 0x0E, 0x18),
                panel_bg: Color32::from_rgb(0x21, 0x16, 0x25),
                card_bg: Color32::from_rgb(0x2D, 0x1C, 0x31),
                border: Color32::from_rgb(0x4A, 0x31, 0x50),
                text_main: Color32::from_rgb(0xFF, 0xF0, 0xFA),
                text_muted: Color32::from_rgb(0xB4, 0x94, 0xB1),
                accent: Color32::from_rgb(0xF1, 0x8A, 0xCB),
                secondary_accent: Color32::from_rgb(0xA7, 0x7B, 0xFF),
                bar_active: Color32::from_rgb(0xF1, 0x8A, 0xCB),
                bar_track: Color32::from_rgb(0x3B, 0x24, 0x41),
                chart_line1: Color32::from_rgb(0xF1, 0x8A, 0xCB),
                chart_line2: Color32::from_rgb(0x82, 0xE3, 0xCF),
                positive: Color32::from_rgb(0x82, 0xE3, 0xCF),
                warning: Color32::from_rgb(0xFF, 0xC1, 0x7A),
                critical: Color32::from_rgb(0xFF, 0x72, 0x9C),
            },
        }
    }
}
