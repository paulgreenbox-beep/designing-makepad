//! Stable semantic names and pure snapshots for AI/UI automation.
//!
//! The Windows accessibility bridge uses the stable tab ID (`tab.item.<id>`),
//! never a visual index. `tab.item.0`-style strings remain useful only as a
//! human-readable snapshot alias and must not be used as an action target.

use crate::{OverlayState, TabRegistry};

/// Stable semantic namespace shared by showcase and future integrations.
pub mod id {
    pub const APP: &str = "designing.makepad";
    pub const TABS: &str = "tabs";
    pub const TAB_COUNT: &str = "tab.count";
    pub const TAB_ACTIVE: &str = "tab.active";
    pub const TAB_NEW: &str = "tab.new";
    pub const OVERLAY_ACTIVE: &str = "overlay.active";
    pub const OVERLAY_KIND: &str = "overlay.kind";
    pub const MENU_FILE: &str = "menu.file";
    pub const MENU_EDIT: &str = "menu.edit";
    pub const MENU_VIEW: &str = "menu.view";
    pub const MENU_WINDOW: &str = "menu.window";
    pub const MENU_LANGUAGE: &str = "menu.language";
    pub const MENU_HELP: &str = "menu.help";
    /// Current desktop-menu state, separate from generic overlay kind.
    pub const MENU_ACTIVE: &str = "menu.active";
    pub const STYLE_LAB: &str = "style.lab";
    pub const STYLE_PALETTE: &str = "style.palette";
    pub const STYLE_SHAPE: &str = "style.shape";
    pub const STYLE_SURFACE: &str = "style.surface";
    pub const STYLE_MOTION: &str = "style.motion";
    pub const STYLE_PRESET: &str = "style.preset";
    pub const STYLE_RESET: &str = "style.reset";
    pub const STYLE_MOTION_PROBE: &str = "style.motion.probe";
    pub const STYLE_PROFILE: &str = "style.profile";
    pub const STYLE_MOTION_STATE: &str = "style.motion.state";
    /// Last Style Lab interaction, exposed for physical UI evidence.
    pub const STYLE_NOTE: &str = "style.note";
    /// Stable UI Automation ID for one logical tab.
    pub fn tab_item(tab_id: u64) -> String {
        format!("tab.item.{tab_id}")
    }

    /// Stable UI Automation ID for a tab's close affordance.
    pub fn tab_close(tab_id: u64) -> String {
        format!("tab.item.{tab_id}.close")
    }

    /// Snapshot-only convenience alias. Do not use it for UI Automation.
    pub fn indexed_tab_alias(index: usize) -> String {
        format!("tab.item.{index}")
    }
}

/// One semantic tab snapshot detached from rendering implementation details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTab {
    pub id: String,
    pub indexed_alias: String,
    pub close_id: Option<String>,
    pub title: String,
    pub dirty: bool,
    pub selected: bool,
}

/// Semantic state exposed by a document-tab shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabSemanticSnapshot {
    pub count: usize,
    pub active: Option<String>,
    pub tabs: Vec<SemanticTab>,
}

/// Projects a pure tab model to stable automation-facing semantics.
pub fn tab_snapshot<T>(tabs: &TabRegistry<T>) -> TabSemanticSnapshot {
    let active = tabs.active().map(|id| id.get());
    let tabs = tabs
        .tabs()
        .enumerate()
        .map(|(index, (id, tab))| SemanticTab {
            id: id::tab_item(id.get()),
            indexed_alias: id::indexed_tab_alias(index),
            close_id: tab.is_closable().then(|| id::tab_close(id.get())),
            title: tab.title().to_owned(),
            dirty: tab.is_dirty(),
            selected: active == Some(id.get()),
        })
        .collect::<Vec<_>>();
    TabSemanticSnapshot {
        count: tabs.len(),
        active: active.map(id::tab_item),
        tabs,
    }
}

/// Small text state useful for an accessibility value node.
pub fn overlay_state_value(state: OverlayState) -> (&'static str, &'static str) {
    (
        if state.is_open() { "true" } else { "false" },
        state.kind().label(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_snapshot_ids_are_not_visual_indices() {
        let mut tabs = TabRegistry::new();
        let first = tabs.add("One", ()).unwrap();
        let second = tabs.add("Two", ()).unwrap();
        tabs.close(first).unwrap();

        let snapshot = tab_snapshot(&tabs);
        assert_eq!(snapshot.tabs[0].id, id::tab_item(second.get()));
        assert_eq!(snapshot.tabs[0].indexed_alias, "tab.item.0");
    }
}
