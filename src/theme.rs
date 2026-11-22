use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Vintage, // Retro Industrial Ivory & Orange
}

impl Serialize for ThemeMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Vintage")
    }
}

impl<'de> Deserialize<'de> for ThemeMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = serde::de::IgnoredAny::deserialize(deserializer);
        Ok(ThemeMode::Vintage)
    }
}

#[allow(dead_code)]
impl ThemeMode {
    pub fn all() -> &'static [ThemeMode] {
        &[ThemeMode::Vintage]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThemeMode::Vintage => "Vintage",
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

impl Default for ThemePalette {
    fn default() -> Self {
        Self::vintage()
    }
}

impl ThemePalette {
    pub fn vintage() -> Self {
        Self {
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
        }
    }

    #[allow(dead_code)]
    pub fn for_mode(_mode: ThemeMode) -> Self {
        Self::vintage()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_default_and_name() {
        let mode = ThemeMode::default();
        assert_eq!(mode, ThemeMode::Vintage);
        assert_eq!(mode.name(), "Vintage");
        assert_eq!(ThemeMode::all(), &[ThemeMode::Vintage]);
    }

    #[test]
    fn test_theme_palette_vintage() {
        let palette = ThemePalette::vintage();
        assert_eq!(palette.background, Color32::from_rgb(0xEA, 0xE6, 0xDC));
        assert_eq!(palette.accent, Color32::from_rgb(0xFA, 0x58, 0x16));
        let default_palette = ThemePalette::default();
        assert_eq!(default_palette.background, palette.background);
        let mode_palette = ThemePalette::for_mode(ThemeMode::Vintage);
        assert_eq!(mode_palette.background, palette.background);
    }

    #[test]
    fn test_theme_mode_lenient_deserialization() {
        let vintage: ThemeMode = serde_json::from_str("\"Vintage\"").unwrap();
        assert_eq!(vintage, ThemeMode::Vintage);

        let nebula: ThemeMode = serde_json::from_str("\"Nebula\"").unwrap();
        assert_eq!(nebula, ThemeMode::Vintage);

        let clearspace: ThemeMode = serde_json::from_str("\"Clearspace\"").unwrap();
        assert_eq!(clearspace, ThemeMode::Vintage);

        let forge: ThemeMode = serde_json::from_str("\"Forge\"").unwrap();
        assert_eq!(forge, ThemeMode::Vintage);

        let terminal: ThemeMode = serde_json::from_str("\"Terminal\"").unwrap();
        assert_eq!(terminal, ThemeMode::Vintage);

        let aura: ThemeMode = serde_json::from_str("\"Aura\"").unwrap();
        assert_eq!(aura, ThemeMode::Vintage);

        let unknown: ThemeMode = serde_json::from_str("\"UnknownFutureTheme\"").unwrap();
        assert_eq!(unknown, ThemeMode::Vintage);
    }
}
