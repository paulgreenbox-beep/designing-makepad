//! Mouse-first transient overlay primitives.

mod overlay_host;
mod overlay_state;

pub use overlay_host::{OverlayHost, OverlayHostAction};
pub use overlay_state::{OverlayKind, OverlayState};

/// Registers the visual overlay host widget.
pub fn script_mod(vm: &mut makepad_widgets::ScriptVm) -> makepad_widgets::ScriptValue {
    overlay_host::script_mod(vm)
}
