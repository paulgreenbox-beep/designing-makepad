//! Central, document-independent visual profile for Makepad desktop shells.
//!
//! The profile contains only application/UI concerns. It deliberately knows
//! nothing about GLYF documents, assets, or metadata, so changing a style can
//! never change a `.glyf` or `.glyp` payload.

use std::{env, fs, path::PathBuf};

use makepad_widgets::{Vec2, Vec4, vec2, vec4};

/// Palette axis for an application shell.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PaletteStyle {
    #[default]
    Dark,
    Light,
    Neon,
}

impl PaletteStyle {
    pub const ALL: [Self; 3] = [Self::Dark, Self::Light, Self::Neon];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::Neon => "neon",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::Neon => "Neon",
        }
    }

    pub fn from_code(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "light" => Self::Light,
            "neon" => Self::Neon,
            _ => Self::Dark,
        }
    }

    pub fn next(self) -> Self {
        cycle_axis(self, &Self::ALL)
    }
}

/// Corner and silhouette axis shared by controls, tabs and popups.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ShapeStyle {
    #[default]
    Rounded,
    Soft,
    Sharp,
    Angular,
}

impl ShapeStyle {
    pub const ALL: [Self; 4] = [Self::Rounded, Self::Soft, Self::Sharp, Self::Angular];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Rounded => "rounded",
            Self::Soft => "soft",
            Self::Sharp => "sharp",
            Self::Angular => "angular",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Rounded => "Rounded",
            Self::Soft => "Soft",
            Self::Sharp => "Sharp",
            Self::Angular => "Angular",
        }
    }

    pub fn from_code(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "soft" => Self::Soft,
            "sharp" => Self::Sharp,
            "angular" => Self::Angular,
            _ => Self::Rounded,
        }
    }

    pub fn next(self) -> Self {
        cycle_axis(self, &Self::ALL)
    }
}

/// Surface treatment axis. Glass and Neumorphic are intentionally laboratory
/// presets; the stable application default remains Elevated.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SurfaceStyle {
    Flat,
    #[default]
    Elevated,
    Glass,
    Neumorphic,
}

impl SurfaceStyle {
    pub const ALL: [Self; 4] = [Self::Flat, Self::Elevated, Self::Glass, Self::Neumorphic];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Elevated => "elevated",
            Self::Glass => "glass",
            Self::Neumorphic => "neumorphic",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Flat => "Flat",
            Self::Elevated => "Elevated",
            Self::Glass => "Glass",
            Self::Neumorphic => "Neumorphic",
        }
    }

    pub fn from_code(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "flat" => Self::Flat,
            "glass" => Self::Glass,
            "neumorphic" => Self::Neumorphic,
            _ => Self::Elevated,
        }
    }

    pub fn next(self) -> Self {
        cycle_axis(self, &Self::ALL)
    }
}

/// Motion policy. `Reduced` preserves short, functional feedback while
/// removing decorative travel and long fades; it is intentionally distinct
/// from `None`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MotionStyle {
    None,
    Reduced,
    #[default]
    Subtle,
    Smooth,
}

impl MotionStyle {
    pub const ALL: [Self; 4] = [Self::None, Self::Reduced, Self::Subtle, Self::Smooth];

    pub const fn code(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Reduced => "reduced",
            Self::Subtle => "subtle",
            Self::Smooth => "smooth",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Reduced => "Reduced",
            Self::Subtle => "Subtle",
            Self::Smooth => "Smooth",
        }
    }

    pub fn from_code(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "none" => Self::None,
            "reduced" => Self::Reduced,
            "smooth" => Self::Smooth,
            _ => Self::Subtle,
        }
    }

    pub fn next(self) -> Self {
        cycle_axis(self, &Self::ALL)
    }
}

/// The four independent visual axes used by all application-facing widgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StyleProfile {
    pub palette: PaletteStyle,
    pub shape: ShapeStyle,
    pub surface: SurfaceStyle,
    pub motion: MotionStyle,
}

impl StyleProfile {
    /// Calm, production-safe default used by GLYF Editor.
    pub const fn production() -> Self {
        Self {
            palette: PaletteStyle::Dark,
            shape: ShapeStyle::Rounded,
            surface: SurfaceStyle::Elevated,
            motion: MotionStyle::Subtle,
        }
    }

    pub const fn classic_light() -> Self {
        Self {
            palette: PaletteStyle::Light,
            shape: ShapeStyle::Soft,
            surface: SurfaceStyle::Flat,
            motion: MotionStyle::None,
        }
    }

    pub const fn neon() -> Self {
        Self {
            palette: PaletteStyle::Neon,
            shape: ShapeStyle::Sharp,
            surface: SurfaceStyle::Elevated,
            motion: MotionStyle::Smooth,
        }
    }

    pub const fn glass_dark() -> Self {
        Self {
            palette: PaletteStyle::Dark,
            shape: ShapeStyle::Rounded,
            surface: SurfaceStyle::Glass,
            motion: MotionStyle::Smooth,
        }
    }

    pub const fn soft_light() -> Self {
        Self {
            palette: PaletteStyle::Light,
            shape: ShapeStyle::Soft,
            surface: SurfaceStyle::Elevated,
            motion: MotionStyle::Subtle,
        }
    }

    pub const fn sharp_dark() -> Self {
        Self {
            palette: PaletteStyle::Dark,
            shape: ShapeStyle::Sharp,
            surface: SurfaceStyle::Flat,
            motion: MotionStyle::Reduced,
        }
    }

    pub fn from_preset(preset: StylePreset) -> Self {
        match preset {
            StylePreset::ClassicLight => Self::classic_light(),
            StylePreset::Midnight => Self::production(),
            StylePreset::Neon => Self::neon(),
            StylePreset::GlassDark => Self::glass_dark(),
            StylePreset::SoftLight => Self::soft_light(),
            StylePreset::SharpDark => Self::sharp_dark(),
        }
    }

    pub fn preset(self) -> Option<StylePreset> {
        StylePreset::ALL
            .iter()
            .copied()
            .find(|preset| Self::from_preset(*preset) == self)
    }

    pub fn summary(self) -> String {
        format!(
            "{} · {} · {} · {}",
            self.palette.label(),
            self.shape.label(),
            self.surface.label(),
            self.motion.label()
        )
    }

    pub fn tokens(self, tuning: StyleTuning) -> StyleTokens {
        StyleTokens::from_profile(self, tuning)
    }

    pub fn serialize(self) -> String {
        format!(
            "palette={}\nshape={}\nsurface={}\nmotion={}\n",
            self.palette.code(),
            self.shape.code(),
            self.surface.code(),
            self.motion.code(),
        )
    }

    pub fn parse(contents: &str) -> Self {
        let mut profile = Self::production();
        for line in contents.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key.trim() {
                "palette" => profile.palette = PaletteStyle::from_code(value),
                "shape" => profile.shape = ShapeStyle::from_code(value),
                "surface" => profile.surface = SurfaceStyle::from_code(value),
                "motion" => profile.motion = MotionStyle::from_code(value),
                _ => {}
            }
        }
        profile
    }
}

impl Default for StyleProfile {
    fn default() -> Self {
        Self::production()
    }
}

/// Convenience profiles for immediately useful combinations. Presets merely
/// assign axes; they never duplicate the token implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StylePreset {
    ClassicLight,
    Midnight,
    Neon,
    GlassDark,
    SoftLight,
    SharpDark,
}

impl StylePreset {
    pub const ALL: [Self; 6] = [
        Self::ClassicLight,
        Self::Midnight,
        Self::Neon,
        Self::GlassDark,
        Self::SoftLight,
        Self::SharpDark,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::ClassicLight => "classic-light",
            Self::Midnight => "midnight",
            Self::Neon => "neon",
            Self::GlassDark => "glass-dark",
            Self::SoftLight => "soft-light",
            Self::SharpDark => "sharp-dark",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ClassicLight => "Classic Light",
            Self::Midnight => "Midnight",
            Self::Neon => "Neon",
            Self::GlassDark => "Glass Dark",
            Self::SoftLight => "Soft Light",
            Self::SharpDark => "Sharp Dark",
        }
    }

    pub fn next(self) -> Self {
        cycle_axis(self, &Self::ALL)
    }
}

/// Small continuous controls exposed by Style Lab. Their normalized range is
/// deliberately bounded to prevent experimental combinations becoming unreadable.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyleTuning {
    pub accent: f32,
    pub radius: f32,
    pub elevation: f32,
    pub opacity: f32,
}

impl StyleTuning {
    pub fn clamped(self) -> Self {
        Self {
            accent: self.accent.clamp(0.0, 1.0),
            radius: self.radius.clamp(0.0, 1.0),
            elevation: self.elevation.clamp(0.0, 1.0),
            opacity: self.opacity.clamp(0.35, 1.0),
        }
    }

    pub fn serialize(self) -> String {
        format!(
            "accent={:.3}\nradius={:.3}\nelevation={:.3}\nopacity={:.3}\n",
            self.accent, self.radius, self.elevation, self.opacity
        )
    }

    pub fn parse(contents: &str) -> Self {
        let mut tuning = Self::default();
        for line in contents.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let Ok(value) = value.trim().parse::<f32>() else {
                continue;
            };
            match key.trim() {
                "accent" => tuning.accent = value,
                "radius" => tuning.radius = value,
                "elevation" => tuning.elevation = value,
                "opacity" => tuning.opacity = value,
                _ => {}
            }
        }
        tuning.clamped()
    }
}

impl Default for StyleTuning {
    fn default() -> Self {
        Self {
            accent: 0.55,
            radius: 0.55,
            elevation: 0.55,
            opacity: 0.9,
        }
    }
}

/// Concrete Makepad-facing style values calculated from a `StyleProfile`.
/// Widgets receive these values through a role, not by branching on a palette.
#[derive(Clone, Copy, Debug)]
pub struct StyleTokens {
    pub window_bg: Vec4,
    pub panel_bg: Vec4,
    pub toolbar_bg: Vec4,
    pub elevated_bg: Vec4,
    pub input_bg: Vec4,
    pub canvas_bg: Vec4,
    pub border: Vec4,
    pub border_strong: Vec4,
    pub separator: Vec4,
    pub text_primary: Vec4,
    pub text_secondary: Vec4,
    pub text_disabled: Vec4,
    pub accent: Vec4,
    pub accent_hover: Vec4,
    pub accent_pressed: Vec4,
    pub accent_subtle: Vec4,
    pub selection: Vec4,
    pub danger: Vec4,
    pub warning: Vec4,
    pub success: Vec4,
    pub control_height: f64,
    pub toolbar_height: f64,
    pub tab_height: f64,
    pub menu_item_height: f64,
    pub spacing_xs: f64,
    pub spacing_sm: f64,
    pub spacing_md: f64,
    pub spacing_lg: f64,
    pub radius_control: f32,
    pub radius_tab: f32,
    pub radius_popup: f32,
    pub surface_opacity: f32,
    pub shadow_strength: f32,
    pub shadow_radius: f32,
    pub highlight_strength: f32,
    pub border_alpha: f32,
    pub motion: MotionTokens,
}

/// Functional timing values shared by menus, tabs and controls.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    pub duration_fast: f64,
    pub duration_normal: f64,
    pub duration_popup: f64,
    pub duration_selection: f64,
    pub popup_offset: f32,
    pub decorative_motion: bool,
}

/// Visual roles keep components independent of individual palette variants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceRole {
    Window,
    Panel,
    Toolbar,
    Elevated,
    Input,
    Canvas,
    Popup,
}

/// A fully resolved appearance for a background surface.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceAppearance {
    pub fill: Vec4,
    pub fill_secondary: Vec4,
    pub border: Vec4,
    pub border_size: f32,
    pub radius: f32,
    pub shadow_color: Vec4,
    pub shadow_radius: f32,
    pub shadow_offset: Vec2,
}

/// Button roles used by a small, consistent component set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
    Icon,
}

/// Resolved control colours that retain Makepad's real hover/focus/down states.
#[derive(Clone, Copy, Debug)]
pub struct ControlAppearance {
    pub fill: Vec4,
    pub hover: Vec4,
    pub pressed: Vec4,
    pub focus: Vec4,
    pub disabled: Vec4,
    pub border: Vec4,
    pub border_focus: Vec4,
    pub text: Vec4,
    pub text_disabled: Vec4,
    pub radius: f32,
}

impl StyleTokens {
    fn from_profile(profile: StyleProfile, tuning: StyleTuning) -> Self {
        let tuning = tuning.clamped();
        let palette = PaletteTokens::from_style(profile.palette);
        let (radius_control, radius_tab, radius_popup) = match profile.shape {
            ShapeStyle::Rounded => (7.0, 8.0, 10.0),
            ShapeStyle::Soft => (11.0, 13.0, 15.0),
            ShapeStyle::Sharp => (0.0, 0.0, 0.0),
            ShapeStyle::Angular => (3.0, 2.0, 4.0),
        };
        let radius_scale = 0.55 + tuning.radius * 0.85;
        let (surface_opacity, shadow_strength, shadow_radius, highlight_strength) =
            match profile.surface {
                SurfaceStyle::Flat => (1.0, 0.0, 0.0, 0.14),
                SurfaceStyle::Elevated => (1.0, 0.36, 16.0, 0.2),
                // This is deliberately a tint + edge + shadow treatment.
                // It does not claim to be backdrop blur.
                SurfaceStyle::Glass => (0.78, 0.32, 20.0, 0.42),
                SurfaceStyle::Neumorphic => (0.96, 0.28, 18.0, 0.5),
            };
        let accent_lift = 0.12 + tuning.accent * 0.28;
        let accent = mix_color(palette.accent, vec4(1.0, 1.0, 1.0, 1.0), accent_lift);
        let accent_hover = mix_color(accent, vec4(1.0, 1.0, 1.0, 1.0), 0.16);
        let accent_pressed = mix_color(accent, palette.window, 0.24);
        let opacity = (surface_opacity * (0.7 + tuning.opacity * 0.3)).clamp(0.35, 1.0);
        let elevation = (0.4 + tuning.elevation * 0.9).clamp(0.25, 1.3);
        Self {
            window_bg: palette.window,
            panel_bg: palette.panel,
            toolbar_bg: palette.toolbar,
            elevated_bg: palette.elevated,
            input_bg: palette.input,
            canvas_bg: palette.canvas,
            border: palette.border,
            border_strong: palette.border_strong,
            separator: palette.separator,
            text_primary: palette.text_primary,
            text_secondary: palette.text_secondary,
            text_disabled: palette.text_disabled,
            accent,
            accent_hover,
            accent_pressed,
            accent_subtle: with_alpha(accent, 0.22),
            selection: with_alpha(accent, 0.34),
            danger: palette.danger,
            warning: palette.warning,
            success: palette.success,
            control_height: 32.0,
            toolbar_height: 42.0,
            tab_height: 38.0,
            menu_item_height: 29.0,
            spacing_xs: 4.0,
            spacing_sm: 8.0,
            spacing_md: 12.0,
            spacing_lg: 16.0,
            radius_control: (radius_control * radius_scale).max(0.0),
            radius_tab: (radius_tab * radius_scale).max(0.0),
            radius_popup: (radius_popup * radius_scale).max(0.0),
            surface_opacity: opacity,
            shadow_strength: shadow_strength * elevation,
            shadow_radius: shadow_radius * elevation,
            highlight_strength,
            border_alpha: if profile.surface == SurfaceStyle::Glass {
                0.58
            } else {
                0.9
            },
            motion: MotionTokens::from_style(profile.motion),
        }
    }

    pub fn surface(self, role: SurfaceRole) -> SurfaceAppearance {
        let (base, radius, elevated) = match role {
            SurfaceRole::Window => (self.window_bg, 0.0, false),
            SurfaceRole::Panel => (self.panel_bg, self.radius_popup, false),
            SurfaceRole::Toolbar => (self.toolbar_bg, self.radius_control, false),
            SurfaceRole::Elevated => (self.elevated_bg, self.radius_popup, true),
            SurfaceRole::Input => (self.input_bg, self.radius_control, false),
            SurfaceRole::Canvas => (self.canvas_bg, self.radius_popup, false),
            SurfaceRole::Popup => (self.elevated_bg, self.radius_popup, true),
        };
        let glass = self.surface_opacity < 0.99;
        let fill = if glass {
            with_alpha(mix_color(base, self.window_bg, 0.18), self.surface_opacity)
        } else {
            base
        };
        let fill_secondary = if glass {
            with_alpha(
                mix_color(base, self.accent, 0.08),
                self.surface_opacity * 0.92,
            )
        } else {
            disabled_gradient()
        };
        let border = if elevated {
            mix_color(
                self.border_strong,
                self.accent,
                self.highlight_strength * 0.22,
            )
        } else {
            self.border
        };
        let (shadow_color, shadow_radius, shadow_offset) =
            if elevated && self.shadow_strength > 0.01 {
                (
                    with_alpha(vec4(0.0, 0.0, 0.0, 1.0), self.shadow_strength),
                    self.shadow_radius,
                    vec2(0.0, 5.0),
                )
            } else {
                (
                    with_alpha(vec4(0.0, 0.0, 0.0, 1.0), 0.0),
                    0.0,
                    vec2(0.0, 0.0),
                )
            };
        SurfaceAppearance {
            fill,
            fill_secondary,
            border: with_alpha(border, self.border_alpha),
            border_size: if matches!(role, SurfaceRole::Window) {
                0.0
            } else {
                1.0
            },
            radius,
            shadow_color,
            shadow_radius,
            shadow_offset,
        }
    }

    pub fn button(self, variant: ButtonVariant) -> ControlAppearance {
        let (fill, hover, pressed, border, text) = match variant {
            ButtonVariant::Primary => (
                self.accent,
                self.accent_hover,
                self.accent_pressed,
                self.accent_hover,
                contrast_text(self.accent),
            ),
            ButtonVariant::Secondary => (
                self.elevated_bg,
                mix_color(self.elevated_bg, self.accent, 0.14),
                mix_color(self.elevated_bg, self.window_bg, 0.18),
                self.border_strong,
                self.text_primary,
            ),
            ButtonVariant::Ghost | ButtonVariant::Icon => (
                with_alpha(self.toolbar_bg, 0.0),
                mix_color(self.toolbar_bg, self.accent, 0.14),
                mix_color(self.toolbar_bg, self.accent, 0.25),
                with_alpha(self.border, 0.0),
                self.text_primary,
            ),
            ButtonVariant::Danger => (
                with_alpha(self.danger, 0.18),
                with_alpha(self.danger, 0.28),
                with_alpha(self.danger, 0.42),
                with_alpha(self.danger, 0.8),
                self.text_primary,
            ),
        };
        ControlAppearance {
            fill,
            hover,
            pressed,
            focus: self.accent_subtle,
            disabled: with_alpha(self.elevated_bg, 0.58),
            border,
            border_focus: self.accent,
            text,
            text_disabled: self.text_disabled,
            radius: self.radius_control,
        }
    }

    pub fn input(self) -> ControlAppearance {
        ControlAppearance {
            fill: self.input_bg,
            hover: mix_color(self.input_bg, self.elevated_bg, 0.26),
            pressed: self.input_bg,
            focus: self.input_bg,
            disabled: with_alpha(self.input_bg, 0.58),
            border: self.border,
            border_focus: self.accent,
            text: self.text_primary,
            text_disabled: self.text_disabled,
            radius: self.radius_control,
        }
    }

    pub fn contrast_ratio(self) -> f32 {
        contrast_ratio(self.text_primary, self.window_bg)
    }
}

impl MotionTokens {
    fn from_style(style: MotionStyle) -> Self {
        match style {
            MotionStyle::None => Self {
                duration_fast: 0.0,
                duration_normal: 0.0,
                duration_popup: 0.0,
                duration_selection: 0.0,
                popup_offset: 0.0,
                decorative_motion: false,
            },
            // A separate accessibility mode: state confirmation stays quick,
            // while travel/long fades are removed.
            MotionStyle::Reduced => Self {
                duration_fast: 0.055,
                duration_normal: 0.08,
                duration_popup: 0.08,
                duration_selection: 0.08,
                popup_offset: 0.0,
                decorative_motion: false,
            },
            MotionStyle::Subtle => Self {
                duration_fast: 0.12,
                duration_normal: 0.16,
                duration_popup: 0.18,
                duration_selection: 0.16,
                popup_offset: 2.0,
                decorative_motion: true,
            },
            MotionStyle::Smooth => Self {
                duration_fast: 0.16,
                duration_normal: 0.22,
                duration_popup: 0.22,
                duration_selection: 0.2,
                popup_offset: 4.0,
                decorative_motion: true,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PaletteTokens {
    window: Vec4,
    panel: Vec4,
    toolbar: Vec4,
    elevated: Vec4,
    input: Vec4,
    canvas: Vec4,
    border: Vec4,
    border_strong: Vec4,
    separator: Vec4,
    text_primary: Vec4,
    text_secondary: Vec4,
    text_disabled: Vec4,
    accent: Vec4,
    danger: Vec4,
    warning: Vec4,
    success: Vec4,
}

impl PaletteTokens {
    fn from_style(style: PaletteStyle) -> Self {
        match style {
            PaletteStyle::Dark => Self {
                window: vec4(0.047, 0.059, 0.078, 1.0),
                panel: vec4(0.071, 0.090, 0.118, 1.0),
                toolbar: vec4(0.060, 0.075, 0.100, 1.0),
                elevated: vec4(0.098, 0.122, 0.157, 1.0),
                input: vec4(0.039, 0.050, 0.069, 1.0),
                canvas: vec4(0.027, 0.035, 0.051, 1.0),
                border: vec4(0.183, 0.220, 0.270, 1.0),
                border_strong: vec4(0.308, 0.376, 0.455, 1.0),
                separator: vec4(0.135, 0.163, 0.204, 1.0),
                text_primary: vec4(0.890, 0.918, 0.950, 1.0),
                text_secondary: vec4(0.600, 0.655, 0.718, 1.0),
                text_disabled: vec4(0.400, 0.446, 0.500, 1.0),
                accent: vec4(0.262, 0.584, 0.965, 1.0),
                danger: vec4(0.949, 0.345, 0.357, 1.0),
                warning: vec4(0.960, 0.700, 0.260, 1.0),
                success: vec4(0.300, 0.770, 0.500, 1.0),
            },
            PaletteStyle::Light => Self {
                window: vec4(0.949, 0.961, 0.980, 1.0),
                panel: vec4(0.992, 0.995, 1.0, 1.0),
                toolbar: vec4(0.918, 0.937, 0.965, 1.0),
                elevated: vec4(1.0, 1.0, 1.0, 1.0),
                input: vec4(0.972, 0.980, 0.992, 1.0),
                canvas: vec4(0.890, 0.910, 0.945, 1.0),
                border: vec4(0.700, 0.744, 0.804, 1.0),
                border_strong: vec4(0.502, 0.573, 0.675, 1.0),
                separator: vec4(0.776, 0.808, 0.855, 1.0),
                text_primary: vec4(0.090, 0.123, 0.171, 1.0),
                text_secondary: vec4(0.315, 0.369, 0.450, 1.0),
                text_disabled: vec4(0.520, 0.560, 0.620, 1.0),
                accent: vec4(0.120, 0.400, 0.870, 1.0),
                danger: vec4(0.785, 0.160, 0.190, 1.0),
                warning: vec4(0.710, 0.455, 0.030, 1.0),
                success: vec4(0.070, 0.550, 0.290, 1.0),
            },
            PaletteStyle::Neon => Self {
                window: vec4(0.020, 0.030, 0.062, 1.0),
                panel: vec4(0.037, 0.053, 0.098, 1.0),
                toolbar: vec4(0.029, 0.044, 0.082, 1.0),
                elevated: vec4(0.055, 0.080, 0.142, 1.0),
                input: vec4(0.016, 0.024, 0.053, 1.0),
                canvas: vec4(0.010, 0.016, 0.040, 1.0),
                border: vec4(0.118, 0.280, 0.386, 1.0),
                border_strong: vec4(0.260, 0.810, 0.910, 1.0),
                separator: vec4(0.075, 0.180, 0.265, 1.0),
                text_primary: vec4(0.846, 0.975, 0.994, 1.0),
                text_secondary: vec4(0.470, 0.710, 0.765, 1.0),
                text_disabled: vec4(0.285, 0.435, 0.500, 1.0),
                accent: vec4(0.055, 0.900, 0.950, 1.0),
                danger: vec4(1.0, 0.280, 0.600, 1.0),
                warning: vec4(1.0, 0.780, 0.150, 1.0),
                success: vec4(0.250, 0.990, 0.570, 1.0),
            },
        }
    }
}

/// Returns a user-level settings path for the showcase. A missing platform
/// config directory is benign: the lab simply starts with its production preset.
pub fn style_settings_path() -> Option<PathBuf> {
    if let Ok(root) = env::var("APPDATA") {
        return Some(
            PathBuf::from(root)
                .join("Designing Makepad")
                .join("style-profile.conf"),
        );
    }
    if let Ok(root) = env::var("XDG_CONFIG_HOME") {
        return Some(
            PathBuf::from(root)
                .join("designing-makepad")
                .join("style-profile.conf"),
        );
    }
    env::var("HOME")
        .ok()
        .map(|root| PathBuf::from(root).join(".config/designing-makepad/style-profile.conf"))
}

pub fn load_persisted_style() -> (StyleProfile, StyleTuning) {
    let Some(path) = style_settings_path() else {
        return (StyleProfile::default(), StyleTuning::default());
    };
    let Ok(contents) = fs::read_to_string(path) else {
        return (StyleProfile::default(), StyleTuning::default());
    };
    (
        StyleProfile::parse(&contents),
        StyleTuning::parse(&contents),
    )
}

pub fn save_persisted_style(profile: StyleProfile, tuning: StyleTuning) -> Result<(), String> {
    let Some(path) = style_settings_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(
        path,
        format!("{}{}", profile.serialize(), tuning.serialize()),
    )
    .map_err(|error| error.to_string())
}

fn cycle_axis<T: Copy + PartialEq>(current: T, all: &[T]) -> T {
    let index = all.iter().position(|value| *value == current).unwrap_or(0);
    all[(index + 1) % all.len()]
}

fn disabled_gradient() -> Vec4 {
    vec4(-1.0, -1.0, -1.0, -1.0)
}

fn with_alpha(color: Vec4, alpha: f32) -> Vec4 {
    vec4(color.x, color.y, color.z, alpha.clamp(0.0, 1.0))
}

fn mix_color(from: Vec4, to: Vec4, amount: f32) -> Vec4 {
    let amount = amount.clamp(0.0, 1.0);
    vec4(
        from.x + (to.x - from.x) * amount,
        from.y + (to.y - from.y) * amount,
        from.z + (to.z - from.z) * amount,
        from.w + (to.w - from.w) * amount,
    )
}

fn contrast_text(background: Vec4) -> Vec4 {
    if relative_luminance(background) > 0.44 {
        vec4(0.035, 0.055, 0.080, 1.0)
    } else {
        vec4(0.970, 0.985, 1.0, 1.0)
    }
}

fn contrast_ratio(foreground: Vec4, background: Vec4) -> f32 {
    let brighter = relative_luminance(foreground).max(relative_luminance(background));
    let darker = relative_luminance(foreground).min(relative_luminance(background));
    (brighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Vec4) -> f32 {
    fn linear(channel: f32) -> f32 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(color.x) + 0.7152 * linear(color.y) + 0.0722 * linear(color.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_the_calm_production_profile() {
        assert_eq!(StyleProfile::default(), StyleProfile::production());
        assert_eq!(
            StyleProfile::default().preset(),
            Some(StylePreset::Midnight)
        );
    }

    #[test]
    fn presets_only_expand_to_axes() {
        assert_eq!(
            StyleProfile::from_preset(StylePreset::Neon),
            StyleProfile::neon()
        );
        assert_eq!(
            StyleProfile::from_preset(StylePreset::GlassDark),
            StyleProfile::glass_dark()
        );
    }

    #[test]
    fn axes_cycle_without_cross_coupling() {
        let profile = StyleProfile::production();
        assert_eq!(profile.palette.next(), PaletteStyle::Light);
        assert_eq!(profile.shape.next(), ShapeStyle::Soft);
        assert_eq!(profile.surface.next(), SurfaceStyle::Glass);
        assert_eq!(profile.motion.next(), MotionStyle::Smooth);
        assert_eq!(profile.shape, ShapeStyle::Rounded);
    }

    #[test]
    fn profile_and_tuning_round_trip() {
        let profile = StyleProfile::glass_dark();
        let tuning = StyleTuning {
            accent: 0.2,
            radius: 0.8,
            elevation: 0.4,
            opacity: 0.7,
        };
        let payload = format!("{}{}", profile.serialize(), tuning.serialize());
        assert_eq!(StyleProfile::parse(&payload), profile);
        assert_eq!(StyleTuning::parse(&payload), tuning);
    }

    #[test]
    fn light_and_dark_keep_readable_primary_text() {
        for palette in [PaletteStyle::Light, PaletteStyle::Dark, PaletteStyle::Neon] {
            let tokens = StyleProfile {
                palette,
                ..StyleProfile::production()
            }
            .tokens(StyleTuning::default());
            assert!(tokens.contrast_ratio() >= 4.5, "{palette:?}");
        }
    }

    #[test]
    fn shape_controls_all_corner_tokens() {
        let rounded = StyleProfile::production().tokens(StyleTuning::default());
        let sharp = StyleProfile {
            shape: ShapeStyle::Sharp,
            ..StyleProfile::production()
        }
        .tokens(StyleTuning::default());
        assert!(rounded.radius_control > sharp.radius_control);
        assert_eq!(sharp.radius_control, 0.0);
        assert_eq!(sharp.radius_tab, 0.0);
        assert_eq!(sharp.radius_popup, 0.0);
    }

    #[test]
    fn surface_changes_elevation_without_palette_branching_in_components() {
        let flat = StyleProfile {
            surface: SurfaceStyle::Flat,
            ..StyleProfile::production()
        }
        .tokens(StyleTuning::default());
        let elevated = StyleProfile::production().tokens(StyleTuning::default());
        assert_eq!(flat.shadow_radius, 0.0);
        assert!(elevated.shadow_radius > flat.shadow_radius);
    }

    #[test]
    fn none_and_reduced_motion_are_distinct() {
        let none = StyleProfile {
            motion: MotionStyle::None,
            ..StyleProfile::production()
        }
        .tokens(StyleTuning::default())
        .motion;
        let reduced = StyleProfile {
            motion: MotionStyle::Reduced,
            ..StyleProfile::production()
        }
        .tokens(StyleTuning::default())
        .motion;
        assert_eq!(none.duration_popup, 0.0);
        assert!(reduced.duration_popup > none.duration_popup);
        assert_eq!(reduced.popup_offset, 0.0);
        assert!(!reduced.decorative_motion);
    }
}
