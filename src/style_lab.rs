//! Runtime Style Lab bridge for the executable showcase.
//!
//! The pure style model lives in `designing_makepad::style`; this module only
//! projects its resolved tokens onto the real widgets that the showcase owns.

use designing_makepad::{
    ButtonVariant, ControlAppearance, MotionStyle, MotionTokens, PaletteStyle, StylePreset,
    StyleTokens, SurfaceAppearance, SurfaceRole, save_persisted_style,
};
use makepad_widgets::*;

use crate::App;

impl App {
    pub(crate) fn load_style_profile(&mut self, cx: &mut Cx) {
        let (profile, tuning) = designing_makepad::load_persisted_style();
        self.style_profile = profile;
        self.style_tuning = tuning;
        self.style_note = "Loaded persisted Style Lab profile".to_owned();
        self.apply_style_profile(cx, false);
    }

    pub(crate) fn cycle_style_palette(&mut self, cx: &mut Cx) {
        self.style_profile.palette = self.style_profile.palette.next();
        self.style_note = format!("Palette → {}", self.style_profile.palette.label());
        self.apply_style_profile(cx, true);
    }

    pub(crate) fn cycle_style_shape(&mut self, cx: &mut Cx) {
        self.style_profile.shape = self.style_profile.shape.next();
        self.style_note = format!("Shape → {}", self.style_profile.shape.label());
        self.apply_style_profile(cx, true);
    }

    pub(crate) fn cycle_style_surface(&mut self, cx: &mut Cx) {
        self.style_profile.surface = self.style_profile.surface.next();
        self.style_note = format!("Surface → {}", self.style_profile.surface.label());
        self.apply_style_profile(cx, true);
    }

    pub(crate) fn cycle_style_motion(&mut self, cx: &mut Cx) {
        self.style_profile.motion = self.style_profile.motion.next();
        self.style_note = format!("Motion → {}", self.style_profile.motion.label());
        self.apply_style_profile(cx, true);
    }

    pub(crate) fn cycle_style_preset(&mut self, cx: &mut Cx) {
        let next = self
            .style_profile
            .preset()
            .map(StylePreset::next)
            .unwrap_or(StylePreset::Midnight);
        self.style_profile = designing_makepad::StyleProfile::from_preset(next);
        self.style_note = format!("Preset → {}", next.label());
        self.apply_style_profile(cx, true);
    }

    pub(crate) fn reset_style_profile(&mut self, cx: &mut Cx) {
        self.style_profile = designing_makepad::StyleProfile::production();
        self.style_tuning = designing_makepad::StyleTuning::default();
        self.style_note = "Reset to GLYF production profile".to_owned();
        self.apply_style_profile(cx, true);
    }

    pub(crate) fn update_style_slider(&mut self, cx: &mut Cx, key: &str, value: f64) {
        let value = value as f32;
        match key {
            "accent" => self.style_tuning.accent = value,
            "radius" => self.style_tuning.radius = value,
            "elevation" => self.style_tuning.elevation = value,
            "opacity" => self.style_tuning.opacity = value,
            _ => return,
        }
        self.style_tuning = self.style_tuning.clamped();
        self.style_note = format!("{} tuned", key.to_ascii_uppercase());
        self.apply_style_profile(cx, true);
    }

    /// Applies a fully resolved profile to existing widgets. Widgets receive a
    /// role and a computed appearance; no widget branches on `PaletteStyle`.
    pub(crate) fn apply_style_profile(&mut self, cx: &mut Cx, persist: bool) {
        self.style_revision = self.style_revision.wrapping_add(1);
        if persist {
            if let Err(error) = save_persisted_style(self.style_profile, self.style_tuning) {
                self.style_note = format!("Style applied but could not be persisted: {error}");
            }
        }
        self.apply_makepad_palette(cx);
        let tokens = self.style_profile.tokens(self.style_tuning);
        self.apply_showcase_style(cx, tokens);
        self.sync_style_lab(cx, tokens);
        self.refresh(cx);
        self.ui.redraw(cx);
    }

    pub(crate) fn play_style_popup_open(&mut self, cx: &mut Cx, _id: &[LiveId], name: &str) {
        let tokens = self.style_profile.tokens(self.style_tuning);
        self.style_note = format!(
            "{name} opened · motion profile: {}",
            self.style_profile.motion.label()
        );
        self.sync_style_lab(cx, tokens);
        self.refresh(cx);
    }

    /// Runs a static DSL Animator on a real Button. The durations are declared
    /// at build time because runtime Splash evaluation cannot construct Animator
    /// values safely. These static durations match duration_normal tokens.
    /// None cuts to the final state; Reduced uses the short path. Other controls
    /// and popup openings do not receive these timings from this probe.
    pub(crate) fn run_style_motion_probe(&mut self, cx: &mut Cx) {
        if let Some(mut probe) = self.ui.button(cx, ids!(style_motion_probe)).borrow_mut() {
            probe.animator_cut(cx, ids!(style_motion.idle));
            match self.style_profile.motion {
                MotionStyle::None => probe.animator_cut(cx, ids!(style_motion.instant)),
                MotionStyle::Reduced => probe.animator_play(cx, ids!(style_motion.reduced)),
                MotionStyle::Subtle => probe.animator_play(cx, ids!(style_motion.subtle)),
                MotionStyle::Smooth => probe.animator_play(cx, ids!(style_motion.smooth)),
            }
        }
        let tokens = self.style_profile.tokens(self.style_tuning);
        self.style_note = match self.style_profile.motion {
            MotionStyle::None => "Motion probe: instant cut; no decorative animation".to_owned(),
            MotionStyle::Reduced => "Motion probe: native reduced 80 ms highlight".to_owned(),
            MotionStyle::Subtle => "Motion probe: native subtle 160 ms highlight".to_owned(),
            MotionStyle::Smooth => "Motion probe: native smooth 220 ms highlight".to_owned(),
        };
        self.sync_style_lab(cx, tokens);
        self.refresh(cx);
    }
    fn apply_makepad_palette(&self, cx: &mut Cx) {
        // TabBar, TextInput and other Makepad-native widgets read `theme.*`
        // directly. Select a predeclared palette/shape variant so those real
        // widgets update alongside the token-applied surfaces without dynamic
        // template construction.
        match (self.style_profile.palette, self.style_profile.shape) {
            (PaletteStyle::Dark, designing_makepad::ShapeStyle::Rounded) => {
                script_eval!(cx, {mod.theme = mod.themes.design_dark_rounded});
            }
            (PaletteStyle::Dark, designing_makepad::ShapeStyle::Soft) => {
                script_eval!(cx, {mod.theme = mod.themes.design_dark_soft});
            }
            (PaletteStyle::Dark, designing_makepad::ShapeStyle::Sharp) => {
                script_eval!(cx, {mod.theme = mod.themes.design_dark_sharp});
            }
            (PaletteStyle::Dark, designing_makepad::ShapeStyle::Angular) => {
                script_eval!(cx, {mod.theme = mod.themes.design_dark_angular});
            }
            (PaletteStyle::Light, designing_makepad::ShapeStyle::Rounded) => {
                script_eval!(cx, {mod.theme = mod.themes.design_light_rounded});
            }
            (PaletteStyle::Light, designing_makepad::ShapeStyle::Soft) => {
                script_eval!(cx, {mod.theme = mod.themes.design_light_soft});
            }
            (PaletteStyle::Light, designing_makepad::ShapeStyle::Sharp) => {
                script_eval!(cx, {mod.theme = mod.themes.design_light_sharp});
            }
            (PaletteStyle::Light, designing_makepad::ShapeStyle::Angular) => {
                script_eval!(cx, {mod.theme = mod.themes.design_light_angular});
            }
            (PaletteStyle::Neon, designing_makepad::ShapeStyle::Rounded) => {
                script_eval!(cx, {mod.theme = mod.themes.design_neon_rounded});
            }
            (PaletteStyle::Neon, designing_makepad::ShapeStyle::Soft) => {
                script_eval!(cx, {mod.theme = mod.themes.design_neon_soft});
            }
            (PaletteStyle::Neon, designing_makepad::ShapeStyle::Sharp) => {
                script_eval!(cx, {mod.theme = mod.themes.design_neon_sharp});
            }
            (PaletteStyle::Neon, designing_makepad::ShapeStyle::Angular) => {
                script_eval!(cx, {mod.theme = mod.themes.design_neon_angular});
            }
        }
    }

    fn apply_showcase_style(&mut self, cx: &mut Cx, tokens: StyleTokens) {
        self.apply_background(cx, ids!(shell), tokens.window_bg);

        self.apply_rounded_surface(cx, ids!(menu_bar), tokens.surface(SurfaceRole::Toolbar));
        self.apply_rounded_surface(cx, ids!(toolbar), tokens.surface(SurfaceRole::Toolbar));
        self.apply_rounded_surface(cx, ids!(status_bar), tokens.surface(SurfaceRole::Toolbar));

        for id in [ids!(workspace_panel)] {
            self.apply_shadow_surface(cx, id, tokens.surface(SurfaceRole::Panel));
        }
        for id in [
            ids!(context_area),
            ids!(icon_catalog),
            ids!(style_lab),
            ids!(style_canvas_mock),
        ] {
            self.apply_shadow_surface(cx, id, tokens.surface(SurfaceRole::Elevated));
        }
        self.apply_shadow_surface(
            cx,
            ids!(style_canvas_mock),
            tokens.surface(SurfaceRole::Canvas),
        );
        for id in [
            ids!(menu_panel),
            ids!(recent_submenu),
            ids!(export_submenu),
            ids!(context_panel),
            ids!(settings_modal_card),
        ] {
            let popup = tokens.surface(SurfaceRole::Popup);
            self.apply_shadow_surface(cx, id, popup);
        }

        let primary = tokens.button(ButtonVariant::Primary);
        let secondary = tokens.button(ButtonVariant::Secondary);
        let ghost = tokens.button(ButtonVariant::Ghost);
        let danger = tokens.button(ButtonVariant::Danger);
        let icon = tokens.button(ButtonVariant::Icon);
        for id in [
            ids!(catalog_primary),
            ids!(style_motion_probe),
            ids!(settings_close),
        ] {
            self.apply_button(cx, id, primary, tokens.motion);
        }
        for id in [
            ids!(toolbar_flip),
            ids!(style_palette),
            ids!(style_shape),
            ids!(style_surface),
            ids!(style_motion),
            ids!(style_preset),
        ] {
            self.apply_button(cx, id, secondary, tokens.motion);
        }
        for id in [
            ids!(menu_file),
            ids!(menu_edit),
            ids!(menu_view),
            ids!(menu_window),
            ids!(menu_language),
            ids!(menu_help),
            ids!(style_reset),
            ids!(menu_new_tab),
            ids!(menu_open_recent),
            ids!(menu_export),
            ids!(menu_dirty),
            ids!(menu_close_tab),
            ids!(menu_dismiss),
            ids!(menu_recent_0),
            ids!(menu_recent_1),
            ids!(menu_recent_2),
            ids!(menu_recent_clear),
            ids!(menu_export_png),
            ids!(menu_export_svg),
            ids!(menu_export_pdf),
            ids!(context_new_tab),
            ids!(context_dirty),
            ids!(context_dismiss),
        ] {
            self.apply_button(cx, id, ghost, tokens.motion);
        }
        for id in [
            ids!(toolbar_new),
            ids!(toolbar_dirty),
            ids!(toolbar_overlay),
            ids!(toolbar_modal),
            ids!(toolbar_many),
            ids!(gallery_file),
            ids!(gallery_folder),
            ids!(gallery_save),
            ids!(gallery_pack),
            ids!(gallery_settings),
            ids!(gallery_help),
            ids!(gallery_pointer),
            ids!(gallery_region),
            ids!(gallery_crop),
            ids!(gallery_layers),
            ids!(gallery_image),
            ids!(gallery_selected),
            ids!(gallery_disabled),
        ] {
            self.apply_button(cx, id, icon, tokens.motion);
        }
        self.apply_button(cx, ids!(catalog_danger), danger, tokens.motion);
        self.apply_input(cx, ids!(catalog_input), tokens.input(), tokens.motion);
    }

    fn sync_style_lab(&mut self, cx: &mut Cx, tokens: StyleTokens) {
        self.ui
            .button(cx, ids!(style_palette))
            .set_text(cx, &format!("{}  v", self.style_profile.palette.label()));
        self.ui
            .button(cx, ids!(style_shape))
            .set_text(cx, &format!("{}  v", self.style_profile.shape.label()));
        self.ui
            .button(cx, ids!(style_surface))
            .set_text(cx, &format!("{}  v", self.style_profile.surface.label()));
        self.ui
            .button(cx, ids!(style_motion))
            .set_text(cx, &format!("{}  v", self.style_profile.motion.label()));
        self.ui.button(cx, ids!(style_preset)).set_text(
            cx,
            self.style_profile
                .preset()
                .map(StylePreset::label)
                .unwrap_or("Custom"),
        );
        self.ui.label(cx, ids!(style_profile_summary)).set_text(
            cx,
            &format!(
                "{}\nProfile revision {}",
                self.style_profile.summary(),
                self.style_revision
            ),
        );
        self.ui.label(cx, ids!(style_token_summary)).set_text(
            cx,
            &format!(
                "radius {:.0} / {:.0} / {:.0}\nshadow {:.0}px · contrast {:.1}:1",
                tokens.radius_control,
                tokens.radius_tab,
                tokens.radius_popup,
                tokens.shadow_radius,
                tokens.contrast_ratio(),
            ),
        );
        self.ui.label(cx, ids!(style_motion_summary)).set_text(
            cx,
            &format!(
                "tokens: hover {:.0}ms · popup {:.0}ms{}",
                tokens.motion.duration_fast * 1000.0,
                tokens.motion.duration_popup * 1000.0,
                if tokens.motion.decorative_motion {
                    " · decorative motion allowed"
                } else {
                    " · no decorative travel"
                }
            ),
        );
        self.ui
            .label(cx, ids!(style_note_label))
            .set_text(cx, &self.style_note);
        self.ui
            .slider(cx, ids!(style_accent))
            .set_value(cx, self.style_tuning.accent as f64);
        self.ui
            .slider(cx, ids!(style_radius))
            .set_value(cx, self.style_tuning.radius as f64);
        self.ui
            .slider(cx, ids!(style_elevation))
            .set_value(cx, self.style_tuning.elevation as f64);
        self.ui
            .slider(cx, ids!(style_opacity))
            .set_value(cx, self.style_tuning.opacity as f64);
    }

    fn apply_background(&mut self, cx: &mut Cx, id: &[LiveId], color: Vec4) {
        let mut widget = self.ui.widget(cx, id);
        script_apply_eval!(cx, widget, {
            draw_bg +: {
                color: #(color)
            }
        });
    }

    fn apply_rounded_surface(&mut self, cx: &mut Cx, id: &[LiveId], surface: SurfaceAppearance) {
        let mut widget = self.ui.widget(cx, id);
        let SurfaceAppearance {
            fill,
            fill_secondary: _,
            border,
            border_size,
            radius,
            ..
        } = surface;
        script_apply_eval!(cx, widget, {
            draw_bg +: {
                color: #(fill)
                border_color: #(border)
                border_size: #(border_size)
                border_radius: #(radius)
            }
        });
    }

    fn apply_shadow_surface(&mut self, cx: &mut Cx, id: &[LiveId], surface: SurfaceAppearance) {
        let mut widget = self.ui.widget(cx, id);
        let SurfaceAppearance {
            fill,
            fill_secondary: _,
            border,
            border_size,
            radius,
            shadow_color,
            shadow_radius,
            shadow_offset,
        } = surface;
        script_apply_eval!(cx, widget, {
            draw_bg +: {
                color: #(fill)
                border_color: #(border)
                border_size: #(border_size)
                border_radius: #(radius)
                shadow_color: #(shadow_color)
                shadow_radius: #(shadow_radius)
                shadow_offset: #(shadow_offset)
            }
        });
    }

    fn apply_button(
        &mut self,
        cx: &mut Cx,
        id: &[LiveId],
        appearance: ControlAppearance,
        _motion: MotionTokens,
    ) {
        let mut button = self.ui.button(cx, id);
        let ControlAppearance {
            fill,
            hover,
            pressed,
            focus,
            disabled,
            border,
            border_focus,
            text,
            text_disabled,
            radius,
        } = appearance;
        script_apply_eval!(cx, button, {
            draw_bg +: {
                color: #(fill)
                color_hover: #(hover)
                color_down: #(pressed)
                color_focus: #(focus)
                color_disabled: #(disabled)
                border_color: #(border)
                border_color_hover: #(border_focus)
                border_color_down: #(border_focus)
                border_color_focus: #(border_focus)
                border_color_disabled: #(border)
                border_radius: #(radius)
            }
            draw_text +: {
                color: #(text)
                color_hover: #(text)
                color_down: #(text)
                color_focus: #(text)
                color_disabled: #(text_disabled)
            }
        });
    }

    fn apply_input(
        &mut self,
        cx: &mut Cx,
        id: &[LiveId],
        appearance: ControlAppearance,
        _motion: MotionTokens,
    ) {
        let mut input = self.ui.text_input(cx, id);
        let ControlAppearance {
            fill,
            hover,
            focus,
            disabled,
            border,
            border_focus,
            text,
            text_disabled,
            radius,
            ..
        } = appearance;
        let placeholder = with_alpha(text_disabled, 0.86);
        script_apply_eval!(cx, input, {
            draw_bg +: {
                color: #(fill)
                color_hover: #(hover)
                color_focus: #(focus)
                color_disabled: #(disabled)
                border_color: #(border)
                border_color_hover: #(border_focus)
                border_color_focus: #(border_focus)
                border_radius: #(radius)
            }
            draw_text +: {
                color: #(text)
                color_hover: #(text)
                color_focus: #(text)
                color_empty: #(placeholder)
                color_empty_hover: #(placeholder)
                color_empty_focus: #(placeholder)
                color_disabled: #(text_disabled)
            }
        });
    }
}

fn with_alpha(color: Vec4, alpha: f32) -> Vec4 {
    vec4(color.x, color.y, color.z, alpha.clamp(0.0, 1.0))
}
