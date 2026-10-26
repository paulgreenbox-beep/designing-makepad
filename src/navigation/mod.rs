//! Navigation primitives for desktop/editor shells.

mod document_tabs;
mod tab_model;

pub use document_tabs::{DocumentTabFrame, DocumentTabStrip, DocumentTabStripAction};
pub use tab_model::{
    DocumentTab, TabId, TabIdExhausted, TabOptions, TabRegistry, TabRegistryError,
};

/// Registers the visual navigation widgets.
pub fn script_mod(vm: &mut makepad_widgets::ScriptVm) -> makepad_widgets::ScriptValue {
    document_tabs::script_mod(vm)
}
