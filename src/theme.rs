//! Shared visual primitives for compact desktop/editor Makepad applications.
//!
//! This is deliberately a token vocabulary and a small set of Makepad presets,
//! not a parallel CSS engine. Applications retain their normal Makepad theme
//! choice and reference the same named primitives from one crate.

use makepad_widgets::*;

/// One documented visual token used by the desktop design system.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DesignToken {
    pub name: &'static str,
    pub makepad_token: &'static str,
    pub purpose: &'static str,
}

/// Numeric rhythm retained in Rust so layout decisions have one named source.
pub mod metrics {
    pub const SPACE_XS: f64 = 4.0;
    pub const SPACE_SM: f64 = 8.0;
    pub const SPACE_MD: f64 = 12.0;
    pub const SPACE_LG: f64 = 16.0;

    pub const RADIUS_SM: f64 = 4.0;
    pub const RADIUS_MD: f64 = 8.0;
    pub const RADIUS_LG: f64 = 12.0;

    pub const CONTROL_COMPACT: f64 = 28.0;
    pub const CONTROL_REGULAR: f64 = 32.0;
    pub const TOOLBAR_HEIGHT: f64 = 42.0;
    pub const TAB_HEIGHT: f64 = 38.0;
}

/// Central inventory for all supported colors, rhythm and type roles.
pub const TOKENS: &[DesignToken] = &[
    DesignToken {
        name: "background_window",
        makepad_token: "theme.color_bg_app",
        purpose: "application canvas behind shell chrome",
    },
    DesignToken {
        name: "background_panel",
        makepad_token: "theme.color_bg_container",
        purpose: "standard content panels",
    },
    DesignToken {
        name: "background_toolbar",
        makepad_token: "theme.color_bg_container",
        purpose: "menu and toolbar strips",
    },
    DesignToken {
        name: "background_elevated",
        makepad_token: "theme.color_bg_even",
        purpose: "tabs, menus and transient surfaces",
    },
    DesignToken {
        name: "background_input",
        makepad_token: "theme.color_inset",
        purpose: "editable controls",
    },
    DesignToken {
        name: "background_canvas",
        makepad_token: "theme.color_bg_app",
        purpose: "image/canvas work area",
    },
    DesignToken {
        name: "border",
        makepad_token: "theme.color_bevel",
        purpose: "quiet panel edges",
    },
    DesignToken {
        name: "border_strong",
        makepad_token: "theme.color_bevel_hover",
        purpose: "focused or elevated edges",
    },
    DesignToken {
        name: "separator",
        makepad_token: "theme.color_bevel",
        purpose: "thin structural dividers",
    },
    DesignToken {
        name: "text_primary",
        makepad_token: "theme.color_text",
        purpose: "headings, control labels and body text",
    },
    DesignToken {
        name: "text_secondary",
        makepad_token: "theme.color_text_meta",
        purpose: "supporting and status text",
    },
    DesignToken {
        name: "text_disabled",
        makepad_token: "theme.color_text_disabled",
        purpose: "unavailable controls",
    },
    DesignToken {
        name: "accent",
        makepad_token: "theme.color_bg_highlight_inline",
        purpose: "selected and primary action state",
    },
    DesignToken {
        name: "accent_hover",
        makepad_token: "theme.color_bg_even",
        purpose: "hover feedback",
    },
    DesignToken {
        name: "accent_pressed",
        makepad_token: "theme.color_highlight",
        purpose: "pressed feedback",
    },
    DesignToken {
        name: "selection",
        makepad_token: "theme.color_selection",
        purpose: "selection background",
    },
    DesignToken {
        name: "danger",
        makepad_token: "theme.color_error",
        purpose: "destructive actions and errors",
    },
    DesignToken {
        name: "warning",
        makepad_token: "theme.color_warning",
        purpose: "non-blocking warnings",
    },
    DesignToken {
        name: "success",
        makepad_token: "theme.color_low",
        purpose: "successful / healthy state",
    },
    DesignToken {
        name: "space_xs",
        makepad_token: "4",
        purpose: "micro gaps",
    },
    DesignToken {
        name: "space_sm",
        makepad_token: "8",
        purpose: "control groups",
    },
    DesignToken {
        name: "space_md",
        makepad_token: "12",
        purpose: "panel rhythm",
    },
    DesignToken {
        name: "space_lg",
        makepad_token: "16",
        purpose: "section rhythm",
    },
    DesignToken {
        name: "radius_sm",
        makepad_token: "4",
        purpose: "compact control corners",
    },
    DesignToken {
        name: "radius_md",
        makepad_token: "8",
        purpose: "panels, menus and tabs",
    },
    DesignToken {
        name: "radius_lg",
        makepad_token: "12",
        purpose: "dialogs and prominent surfaces",
    },
    DesignToken {
        name: "control_compact",
        makepad_token: "28",
        purpose: "dense menu and inline control height",
    },
    DesignToken {
        name: "control_regular",
        makepad_token: "32",
        purpose: "default desktop control height",
    },
    DesignToken {
        name: "toolbar_height",
        makepad_token: "42",
        purpose: "toolbar strip height",
    },
    DesignToken {
        name: "tab_height",
        makepad_token: "38",
        purpose: "document tab row height",
    },
    DesignToken {
        name: "type_caption",
        makepad_token: "theme.font_size_p",
        purpose: "compact metadata",
    },
    DesignToken {
        name: "type_body",
        makepad_token: "theme.font_size_p",
        purpose: "general UI copy",
    },
    DesignToken {
        name: "type_label",
        makepad_token: "theme.font_size_3",
        purpose: "section labels",
    },
    DesignToken {
        name: "type_title",
        makepad_token: "theme.font_size_2",
        purpose: "panel and modal title",
    },
    DesignToken {
        name: "type_heading",
        makepad_token: "theme.font_size_1",
        purpose: "screen heading",
    },
];

/// Looks up a central token without creating a second runtime styling system.
pub fn token(name: &str) -> Option<&'static DesignToken> {
    TOKENS.iter().find(|token| token.name == name)
}

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // Static theme variants keep built-in widgets such as TabBar and TextInput
    // on Makepad's supported theme path. Style Lab selects one at runtime;
    // it never tries to construct a Tab/Animator in a dynamic Splash eval.
    mod.themes.design_dark_rounded = mod.themes.dark {
        corner_radius: 7.0
        container_corner_radius: 10.0
        textselection_corner_radius: 3.5
    }
    mod.themes.design_dark_soft = mod.themes.dark {
        corner_radius: 11.0
        container_corner_radius: 15.0
        textselection_corner_radius: 5.5
    }
    mod.themes.design_dark_sharp = mod.themes.dark {
        corner_radius: 0.0
        container_corner_radius: 0.0
        textselection_corner_radius: 0.0
    }
    mod.themes.design_dark_angular = mod.themes.dark {
        corner_radius: 3.0
        container_corner_radius: 4.0
        textselection_corner_radius: 1.5
    }

    mod.themes.design_light_rounded = mod.themes.light {
        corner_radius: 7.0
        container_corner_radius: 10.0
        textselection_corner_radius: 3.5
    }
    mod.themes.design_light_soft = mod.themes.light {
        corner_radius: 11.0
        container_corner_radius: 15.0
        textselection_corner_radius: 5.5
    }
    mod.themes.design_light_sharp = mod.themes.light {
        corner_radius: 0.0
        container_corner_radius: 0.0
        textselection_corner_radius: 0.0
    }
    mod.themes.design_light_angular = mod.themes.light {
        corner_radius: 3.0
        container_corner_radius: 4.0
        textselection_corner_radius: 1.5
    }

    // Neon retains the calm dark hierarchy. Only selection/focus and the
    // active accent gain cyan energy, so it never turns into a full HUD.
    mod.themes.design_neon_rounded = mod.themes.dark {
        corner_radius: 7.0
        container_corner_radius: 10.0
        textselection_corner_radius: 3.5
        color_tint: #18314dff
        color_tint_amount: 0.22
        color_highlight: #59e6ffff
        color_bg_highlight_inline: #124d67ff
        color_bevel_hover: #59e6ffff
    }
    mod.themes.design_neon_soft = mod.themes.design_neon_rounded {
        corner_radius: 11.0
        container_corner_radius: 15.0
        textselection_corner_radius: 5.5
    }
    mod.themes.design_neon_sharp = mod.themes.design_neon_rounded {
        corner_radius: 0.0
        container_corner_radius: 0.0
        textselection_corner_radius: 0.0
    }
    mod.themes.design_neon_angular = mod.themes.design_neon_rounded {
        corner_radius: 3.0
        container_corner_radius: 4.0
        textselection_corner_radius: 1.5
    }

    // The calm production alias is deliberately narrow: it is the style
    // adopted by GLYF Editor, while the other combinations remain Style Lab
    // experimentation.
    mod.themes.glyf_production = mod.themes.design_dark_rounded {}
    mod.themes.glyf_light = mod.themes.design_light_rounded {}

    // Structural surfaces
    mod.widgets.DsWindow = mod.widgets.View {
        show_bg: true
        draw_bg.color: theme.color_bg_app
    }

    mod.widgets.DsPanel = mod.widgets.RoundedShadowView {
        show_bg: true
        new_batch: true
        draw_bg.color: theme.color_bg_container
        draw_bg.border_radius: 8.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: theme.color_bevel
        draw_bg.shadow_color: #0000
        draw_bg.shadow_radius: 0.0
        draw_bg.shadow_offset: vec2(0.0 0.0)
    }

    mod.widgets.DsElevatedPanel = mod.widgets.RoundedShadowView {
        show_bg: true
        new_batch: true
        draw_bg.color: theme.color_bg_even
        draw_bg.border_radius: 8.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: theme.color_bevel_hover
        draw_bg.shadow_color: #0005
        draw_bg.shadow_radius: 14.0
        draw_bg.shadow_offset: vec2(0.0 4.0)
    }

    mod.widgets.DsToolbar = mod.widgets.RoundedView {
        height: 42
        flow: Right
        spacing: 8
        padding: Inset{left: 10 right: 10 top: 5 bottom: 5}
        align: Align {y: 0.5}
        show_bg: true
        new_batch: true
        draw_bg.color: theme.color_bg_container
        draw_bg.border_radius: 6.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: theme.color_bevel
    }

    mod.widgets.DsMenuBar = mod.widgets.RoundedView {
        height: 34
        flow: Right
        spacing: 2
        padding: Inset{left: 8 right: 8 top: 3 bottom: 3}
        show_bg: true
        new_batch: true
        draw_bg.color: theme.color_bg_container
        draw_bg.border_radius: 6.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: theme.color_bevel
    }

    mod.widgets.DsStatusBar = mod.widgets.RoundedView {
        height: 28
        padding: Inset{left: 12 right: 12 top: 5 bottom: 5}
        show_bg: true
        new_batch: true
        draw_bg.color: theme.color_bg_container
        draw_bg.border_radius: 6.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: theme.color_bevel
    }

    // Text and standard controls
    mod.widgets.DsButton = mod.widgets.ButtonFlat {
        height: 32
        padding: Inset{left: 10 right: 10 top: 5 bottom: 5}
    }

    mod.widgets.DsPrimaryButton = mod.widgets.Button {
        height: 32
        padding: Inset{left: 12 right: 12 top: 5 bottom: 5}
    }

    mod.widgets.DsMotionProbe = mod.widgets.DsPrimaryButton {
        reset_hover_on_click: true
        animator +: {
            style_motion: {
                default: @idle
                idle: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        draw_bg: {down: 0.0 hover: 0.0}
                        draw_text: {down: 0.0 hover: 0.0}
                    }
                }
                instant: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        draw_bg: {down: 0.0 hover: 1.0}
                        draw_text: {down: 0.0 hover: 1.0}
                    }
                }
                reduced: AnimatorState{
                    from: {all: Forward {duration: 0.06}}
                    ease: OutQuad
                    apply: {
                        draw_bg: {down: 0.0 hover: 1.0}
                        draw_text: {down: 0.0 hover: 1.0}
                    }
                }
                subtle: AnimatorState{
                    from: {all: Forward {duration: 0.16}}
                    ease: OutQuad
                    apply: {
                        draw_bg: {down: 0.0 hover: 1.0}
                        draw_text: {down: 0.0 hover: 1.0}
                    }
                }
                smooth: AnimatorState{
                    from: {all: Forward {duration: 0.28}}
                    ease: OutQuad
                    apply: {
                        draw_bg: {down: 0.0 hover: 1.0}
                        draw_text: {down: 0.0 hover: 1.0}
                    }
                }
            }
        }
    }
    mod.widgets.DsDangerButton = mod.widgets.ButtonFlat {
        height: 32
        padding: Inset{left: 10 right: 10 top: 5 bottom: 5}
        draw_text.color: theme.color_error
    }

    mod.widgets.DsInput = mod.widgets.TextInput {
        height: 32
    }

    // Menus are overlays. Their owning application places the surface at an
    // absolute anchor and routes outside clicks before normal content.
    mod.widgets.DsMenuButton = mod.widgets.ButtonFlatter {
        height: 28
        padding: Inset{left: 9 right: 9 top: 4 bottom: 4}
        draw_text.color: theme.color_text
        draw_bg +: {
            border_radius: 4.0
            color_hover: theme.color_bg_even
            color_down: theme.color_bg_highlight_inline
        }
    }

    // A popup must be an opaque surface: menus otherwise blend into the canvas
    // and lose their visual affordance even though their actions remain active.
    mod.widgets.DsMenuSurface = mod.widgets.RoundedShadowView {
        show_bg: true
        new_batch: true
        draw_bg.color: theme.color_bg_app
        draw_bg.border_radius: 8.0
        draw_bg.border_size: 2.0
        draw_bg.border_color: theme.color_highlight
        draw_bg.shadow_color: #0008
        draw_bg.shadow_radius: 18.0
        draw_bg.shadow_offset: vec2(0.0 5.0)
    }

    mod.widgets.DsMenuItem = mod.widgets.ButtonFlatter {
        width: Fill
        height: 29
        padding: Inset{left: 10 right: 10 top: 4 bottom: 4}
        draw_text.color: theme.color_text
        draw_bg +: {
            border_radius: 4.0
            color_hover: theme.color_bg_highlight_inline
            color_down: theme.color_bg_highlight_inline
        }
    }

    mod.widgets.DsMenuSeparator = mod.widgets.Hr {
        width: Fill
        height: 1
        margin: Inset{left: 4 right: 4 top: 4 bottom: 4}
    }

    // IconButton is a normal Makepad ButtonFlatterIcon. `IconButtonSpec`
    // provides its document-independent icon/tooltip/selected/disabled/text
    // contract; applications set SVG and accessible name at their edge.
    mod.widgets.DsIcon = mod.widgets.Icon {
        icon_walk: Walk{width: 18 height: 18}
        draw_icon.color: theme.color_text
    }

    mod.widgets.DsIconButton = mod.widgets.ButtonFlatterIcon {
        width: 32
        height: 32
        padding: Inset{left: 6 right: 6 top: 6 bottom: 6}
        draw_bg +: {
            border_radius: 6.0
            color_hover: theme.color_bg_even
            color_down: theme.color_bg_highlight_inline
        }
        draw_icon.color: theme.color_text
    }

    mod.widgets.DsIconButtonSelected = mod.widgets.DsIconButton {
        draw_bg +: {
            color: theme.color_bg_highlight_inline
            color_hover: theme.color_bg_even
            color_down: theme.color_highlight
            border_color: theme.color_bevel_hover
        }
    }

    // Rune stones keep the interaction affordance in the button while the
    // icon remains a compact, two-colour SVG. The lower accent is deliberately
    // inside the widget bounds, so it stays visible in a dense toolbar.
    mod.widgets.DsRuneStoneButton = mod.widgets.ButtonFlatterIcon {
        width: 46
        height: 46
        padding: Inset{left: 13 right: 13 top: 11 bottom: 15}
        icon_walk: Walk{width: 19 height: 19}
        draw_bg +: {
            color: #121924ff
            color_hover: #ff7a18ff
            color_down: #cc4f00ff
            color_focus: #ff7a18ff
            border_size: 1.0
            border_color: #354355ff
            border_color_hover: #ff7a18ff
            border_color_down: #cc4f00ff
            border_color_focus: #ff7a18ff
        }
        draw_icon.color: vec4(-1.0 -1.0 -1.0 -1.0)
    }

    // Eight deliberately distinct, softly tumbled-stone silhouettes. Their
    // outlines stay rounded and slightly organic, like polished river stones.
    mod.widgets.DsRuneStoneCircle = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.circle(22.5 20.0 16.2)
                sdf.circle(19.5 23.5 15.4)
                sdf.gloop(4.0)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneSquircle = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.box_all(4.0 3.5 self.rect_size.x - 8.0 self.rect_size.y - 10.0 14.0 15.5 15.0 13.0)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneTall = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.box_all(7.0 2.5 self.rect_size.x - 14.0 self.rect_size.y - 9.0 14.5 13.5 15.5 14.5)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneWide = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.box_all(2.5 6.0 self.rect_size.x - 5.0 self.rect_size.y - 15.0 14.5 15.0 13.5 15.5)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneHex = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.hexagon(23.0 20.5 16.0)
                sdf.circle(22.0 21.5 15.3)
                sdf.gloop(5.0)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneShard = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.circle(22.5 10.5 10.0)
                sdf.circle(28.5 22.5 12.0)
                sdf.gloop(7.0)
                sdf.circle(17.0 28.5 11.5)
                sdf.gloop(7.0)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneArch = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.box_all(5.0 2.5 self.rect_size.x - 10.0 self.rect_size.y - 8.0 15.0 16.0 15.0 16.0)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    mod.widgets.DsRuneStoneRiver = mod.widgets.DsRuneStoneButton {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let glow = max(self.hover, self.focus * 0.45)
                let accent = mix(self.color_hover, self.color_down, self.down)
                if glow > 0.001 {
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.fill(vec4(accent.xyz, glow * 0.82))
                    sdf.box(9.0 self.rect_size.y - 4.0 self.rect_size.x - 18.0 1.6 0.8)
                    sdf.glow(vec4(accent.xyz, glow * 0.72), 4.0)
                }
                let stone = mix(self.color, self.color_hover, glow * 0.08)
                let edge = mix(self.border_color, self.border_color_hover, glow * 0.88)
                sdf.circle(13.0 20.5 10.5)
                sdf.circle(21.5 12.5 11.5)
                sdf.gloop(6.0)
                sdf.circle(31.0 21.0 10.5)
                sdf.gloop(6.0)
                sdf.circle(22.5 30.0 10.5)
                sdf.gloop(6.0)
                sdf.fill_keep(stone)
                sdf.stroke(edge 1.0)
                return sdf.result
            }
        }
    }

    // Accent families: ember = project, verdant = resources, azure =
    // annotation, violet = viewport, gold = universal edit actions.
    mod.widgets.DsSignalIconButton = mod.widgets.DsRuneStoneCircle {}
    mod.widgets.DsSignalIconButtonSelected = mod.widgets.DsRuneStoneCircle {
        draw_bg +: {
            color_hover: #36b6ffff
            color_down: #1a7bcaff
            color_focus: #36b6ffff
            border_color_hover: #36b6ffff
            border_color_down: #1a7bcaff
            border_color_focus: #36b6ffff
        }
    }

    mod.widgets.DsSignalIconButtonNew = mod.widgets.DsRuneStoneCircle {
        draw_bg +: { color_hover: #ff7a18ff color_down: #cc4f00ff color_focus: #ff7a18ff border_color_hover: #ff7a18ff border_color_down: #cc4f00ff border_color_focus: #ff7a18ff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-new.svg")
    }
    mod.widgets.DsSignalIconButtonOpen = mod.widgets.DsRuneStoneWide {
        draw_bg +: { color_hover: #f4c34dff color_down: #c28a20ff color_focus: #f4c34dff border_color_hover: #f4c34dff border_color_down: #c28a20ff border_color_focus: #f4c34dff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-open.svg")
    }
    mod.widgets.DsSignalIconButtonSave = mod.widgets.DsRuneStoneSquircle {
        draw_bg +: { color_hover: #ff7a18ff color_down: #cc4f00ff color_focus: #ff7a18ff border_color_hover: #ff7a18ff border_color_down: #cc4f00ff border_color_focus: #ff7a18ff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-save.svg")
    }
    mod.widgets.DsSignalIconButtonImage = mod.widgets.DsRuneStoneRiver {
        draw_bg +: { color_hover: #7bdc70ff color_down: #4fab55ff color_focus: #7bdc70ff border_color_hover: #7bdc70ff border_color_down: #4fab55ff border_color_focus: #7bdc70ff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-image.svg")
    }
    mod.widgets.DsSignalIconButtonSelect = mod.widgets.DsRuneStoneShard {
        draw_bg +: { color_hover: #36b6ffff color_down: #1a7bcaff color_focus: #36b6ffff border_color_hover: #36b6ffff border_color_down: #1a7bcaff border_color_focus: #36b6ffff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-select.svg")
    }
    mod.widgets.DsSignalIconButtonPoint = mod.widgets.DsRuneStoneCircle {
        draw_bg +: { color_hover: #36b6ffff color_down: #1a7bcaff color_focus: #36b6ffff border_color_hover: #36b6ffff border_color_down: #1a7bcaff border_color_focus: #36b6ffff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-point.svg")
    }
    mod.widgets.DsSignalIconButtonRegion = mod.widgets.DsRuneStoneArch {
        draw_bg +: { color_hover: #36b6ffff color_down: #1a7bcaff color_focus: #36b6ffff border_color_hover: #36b6ffff border_color_down: #1a7bcaff border_color_focus: #36b6ffff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-region.svg")
    }
    mod.widgets.DsSignalIconButtonFit = mod.widgets.DsRuneStoneHex {
        draw_bg +: { color_hover: #b980ffff color_down: #805ad5ff color_focus: #b980ffff border_color_hover: #b980ffff border_color_down: #805ad5ff border_color_focus: #b980ffff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-fit.svg")
    }
    mod.widgets.DsSignalIconButtonZoomIn = mod.widgets.DsRuneStoneTall {
        draw_bg +: { color_hover: #b980ffff color_down: #805ad5ff color_focus: #b980ffff border_color_hover: #b980ffff border_color_down: #805ad5ff border_color_focus: #b980ffff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-zoom-in.svg")
    }
    mod.widgets.DsSignalIconButtonZoomOut = mod.widgets.DsRuneStoneWide {
        draw_bg +: { color_hover: #b980ffff color_down: #805ad5ff color_focus: #b980ffff border_color_hover: #b980ffff border_color_down: #805ad5ff border_color_focus: #b980ffff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-zoom-out.svg")
    }
    mod.widgets.DsSignalIconButtonLayers = mod.widgets.DsRuneStoneSquircle {
        draw_bg +: { color_hover: #7bdc70ff color_down: #4fab55ff color_focus: #7bdc70ff border_color_hover: #7bdc70ff border_color_down: #4fab55ff border_color_focus: #7bdc70ff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-layers.svg")
    }
    mod.widgets.DsSignalIconButtonPackage = mod.widgets.DsRuneStoneHex {
        draw_bg +: { color_hover: #f4c34dff color_down: #c28a20ff color_focus: #f4c34dff border_color_hover: #f4c34dff border_color_down: #c28a20ff border_color_focus: #f4c34dff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-package.svg")
    }
    mod.widgets.DsSignalIconButtonUndo = mod.widgets.DsRuneStoneRiver {
        draw_bg +: { color_hover: #f4c34dff color_down: #c28a20ff color_focus: #f4c34dff border_color_hover: #f4c34dff border_color_down: #c28a20ff border_color_focus: #f4c34dff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-undo.svg")
    }
    mod.widgets.DsSignalIconButtonRedo = mod.widgets.DsRuneStoneShard {
        draw_bg +: { color_hover: #7bdc70ff color_down: #4fab55ff color_focus: #7bdc70ff border_color_hover: #7bdc70ff border_color_down: #4fab55ff border_color_focus: #7bdc70ff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-redo.svg")
    }
    mod.widgets.DsSignalIconButtonCopy = mod.widgets.DsRuneStoneArch {
        draw_bg +: { color_hover: #f4c34dff color_down: #c28a20ff color_focus: #f4c34dff border_color_hover: #f4c34dff border_color_down: #c28a20ff border_color_focus: #f4c34dff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-copy.svg")
    }
    mod.widgets.DsSignalIconButtonCut = mod.widgets.DsRuneStoneShard {
        draw_bg +: { color_hover: #f4c34dff color_down: #c28a20ff color_focus: #f4c34dff border_color_hover: #f4c34dff border_color_down: #c28a20ff border_color_focus: #f4c34dff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-cut.svg")
    }
    mod.widgets.DsSignalIconButtonPaste = mod.widgets.DsRuneStoneTall {
        draw_bg +: { color_hover: #f4c34dff color_down: #c28a20ff color_focus: #f4c34dff border_color_hover: #f4c34dff border_color_down: #c28a20ff border_color_focus: #f4c34dff }
        draw_icon.svg: crate_resource("self:resources/icons/signal/signal-paste.svg")
    }
    mod.widgets.DsIconFilePlus = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/file-plus.svg") }
    mod.widgets.DsIconFolderOpen = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/folder-open.svg") }
    mod.widgets.DsIconSave = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/save.svg") }
    mod.widgets.DsIconPackage = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/package.svg") }
    mod.widgets.DsIconUndo = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/undo-2.svg") }
    mod.widgets.DsIconRedo = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/redo-2.svg") }
    mod.widgets.DsIconCopy = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/copy.svg") }
    mod.widgets.DsIconScissors = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/scissors.svg") }
    mod.widgets.DsIconClipboard = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/clipboard.svg") }
    mod.widgets.DsIconPlus = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/plus.svg") }
    mod.widgets.DsIconClose = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/x.svg") }
    mod.widgets.DsIconChevronDown = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/chevron-down.svg") }
    mod.widgets.DsIconChevronLeft = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/chevron-left.svg") }
    mod.widgets.DsIconChevronRight = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/chevron-right.svg") }
    mod.widgets.DsIconMore = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/ellipsis.svg") }
    mod.widgets.DsIconMoreVertical = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/ellipsis-vertical.svg") }
    mod.widgets.DsIconSettings = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/settings-2.svg") }
    mod.widgets.DsIconHelp = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/circle-question-mark.svg") }
    mod.widgets.DsIconInfo = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/info.svg") }
    mod.widgets.DsIconPointer = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/mouse-pointer-2.svg") }
    mod.widgets.DsIconPoint = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/crosshair.svg") }
    mod.widgets.DsIconRegion = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/square-dashed-mouse-pointer.svg") }
    mod.widgets.DsIconCrop = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/crop.svg") }
    mod.widgets.DsIconFit = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/maximize.svg") }
    mod.widgets.DsIconZoomIn = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/zoom-in.svg") }
    mod.widgets.DsIconZoomOut = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/zoom-out.svg") }
    mod.widgets.DsIconPan = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/move.svg") }
    mod.widgets.DsIconLayers = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/layers.svg") }
    mod.widgets.DsIconImage = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/image.svg") }
    mod.widgets.DsIconPanelRight = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/panel-right-open.svg") }
    mod.widgets.DsIconPanelLeft = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/panel-left-open.svg") }
    mod.widgets.DsIconEye = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/eye.svg") }
    mod.widgets.DsIconWarning = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/triangle-alert.svg") }
    mod.widgets.DsIconTrash = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/trash-2.svg") }
    mod.widgets.DsIconCheck = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/check.svg") }
    mod.widgets.DsIconFileImage = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/file-image.svg") }
    mod.widgets.DsIconSliders = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/sliders-horizontal.svg") }
    mod.widgets.DsIconLayoutPanelLeft = mod.widgets.DsIcon { draw_icon.svg: crate_resource("self:resources/icons/layout-panel-left.svg") }

    mod.widgets.DsIconButtonFilePlus = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/file-plus.svg") }
    mod.widgets.DsIconButtonFolderOpen = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/folder-open.svg") }
    mod.widgets.DsIconButtonSave = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/save.svg") }
    mod.widgets.DsIconButtonPackage = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/package.svg") }
    mod.widgets.DsIconButtonUndo = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/undo-2.svg") }
    mod.widgets.DsIconButtonRedo = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/redo-2.svg") }
    mod.widgets.DsIconButtonCopy = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/copy.svg") }
    mod.widgets.DsIconButtonScissors = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/scissors.svg") }
    mod.widgets.DsIconButtonClipboard = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/clipboard.svg") }
    mod.widgets.DsIconButtonPlus = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/plus.svg") }
    mod.widgets.DsIconButtonClose = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/x.svg") }
    mod.widgets.DsIconButtonChevronDown = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/chevron-down.svg") }
    mod.widgets.DsIconButtonChevronLeft = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/chevron-left.svg") }
    mod.widgets.DsIconButtonChevronRight = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/chevron-right.svg") }
    mod.widgets.DsIconButtonMore = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/ellipsis.svg") }
    mod.widgets.DsIconButtonMoreVertical = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/ellipsis-vertical.svg") }
    mod.widgets.DsIconButtonSettings = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/settings-2.svg") }
    mod.widgets.DsIconButtonHelp = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/circle-question-mark.svg") }
    mod.widgets.DsIconButtonInfo = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/info.svg") }
    mod.widgets.DsIconButtonPointer = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/mouse-pointer-2.svg") }
    mod.widgets.DsIconButtonPoint = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/crosshair.svg") }
    mod.widgets.DsIconButtonRegion = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/square-dashed-mouse-pointer.svg") }
    mod.widgets.DsIconButtonCrop = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/crop.svg") }
    mod.widgets.DsIconButtonFit = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/maximize.svg") }
    mod.widgets.DsIconButtonZoomIn = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/zoom-in.svg") }
    mod.widgets.DsIconButtonZoomOut = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/zoom-out.svg") }
    mod.widgets.DsIconButtonPan = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/move.svg") }
    mod.widgets.DsIconButtonLayers = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/layers.svg") }
    mod.widgets.DsIconButtonImage = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/image.svg") }
    mod.widgets.DsIconButtonPanelRight = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/panel-right-open.svg") }
    mod.widgets.DsIconButtonPanelLeft = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/panel-left-open.svg") }
    mod.widgets.DsIconButtonEye = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/eye.svg") }
    mod.widgets.DsIconButtonWarning = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/triangle-alert.svg") }
    mod.widgets.DsIconButtonTrash = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/trash-2.svg") }
    mod.widgets.DsIconButtonCheck = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/check.svg") }
    mod.widgets.DsIconButtonFileImage = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/file-image.svg") }
    mod.widgets.DsIconButtonSliders = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/sliders-horizontal.svg") }
    mod.widgets.DsIconButtonLayoutPanelLeft = mod.widgets.DsIconButton { draw_icon.svg: crate_resource("self:resources/icons/layout-panel-left.svg") }

    // Backwards-compatible names used by the previous catalog milestone.
    mod.widgets.DesigningPanel = mod.widgets.DsPanel {}
    mod.widgets.DesigningMenuPanel = mod.widgets.DsMenuSurface {}
    mod.widgets.DesigningMenuButton = mod.widgets.DsMenuButton {}
    mod.widgets.DesigningToolbarButton = mod.widgets.DsButton {}
}
