//! Small reusable desktop/editor building blocks for Makepad.
//!
//! This crate deliberately stays narrow: it owns a generic document-tab
//! model, a visual `DocumentTabStrip`, a mouse-first `OverlayHost`, and a
//! shared vocabulary for design tokens and automation semantics. Applications
//! remain responsible for document data, commands, and persistence.

pub mod icons;
pub mod menu;
pub mod navigation;
pub mod overlay;
pub mod style;
pub mod testing;
pub mod theme;

pub use testing::semantics;

pub use icons::{IconButtonSpec, IconName, RuneStoneShape, SignalColorSet, SignalIconName};
pub use menu::{
    DesktopMenu, MenuChainState, MenuItem, MenuItemKind, MenuRect, MenuState, MenuTransition,
    SubmenuDirection, SubmenuPlacement, place_submenu,
};
pub use navigation::{
    DocumentTab, DocumentTabFrame, DocumentTabStrip, DocumentTabStripAction, TabId, TabIdExhausted,
    TabOptions, TabRegistry, TabRegistryError,
};
pub use overlay::{OverlayHost, OverlayHostAction, OverlayKind, OverlayState};
pub use style::{
    ButtonVariant, ControlAppearance, MotionStyle, MotionTokens, PaletteStyle, ShapeStyle,
    StylePreset, StyleProfile, StyleTokens, StyleTuning, SurfaceAppearance, SurfaceRole,
    SurfaceStyle, load_persisted_style, save_persisted_style,
};

/// Registers the reusable Makepad widget definitions exposed by this crate.
pub fn script_mod(vm: &mut makepad_widgets::ScriptVm) -> makepad_widgets::ScriptValue {
    theme::script_mod(vm);
    navigation::script_mod(vm);
    overlay::script_mod(vm)
}
