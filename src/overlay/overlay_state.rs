//! Pure overlay lifecycle state, independent of concrete menu content.

/// A small finite set of mouse-first transient overlay kinds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OverlayKind {
    /// No transient overlay is visible.
    #[default]
    None,
    /// An application menu such as File or Edit.
    MainMenu,
    /// A context-sensitive menu opened by a secondary click.
    ContextMenu,
    /// A lightweight floating panel. Focus-sensitive editors should instead
    /// remain normal widgets, and application dialogs should use `Modal`.
    FloatingPanel,
}

impl OverlayKind {
    /// Human-readable state for status text and semantic snapshots.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MainMenu => "main-menu",
            Self::ContextMenu => "context-menu",
            Self::FloatingPanel => "floating-panel",
        }
    }
}

/// State owner for a single active overlay in a desktop shell.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlayState {
    kind: OverlayKind,
}

impl OverlayState {
    /// Opens (or replaces) the active overlay kind.
    pub fn open(&mut self, kind: OverlayKind) {
        self.kind = kind;
    }

    /// Explicit alias that documents replacement of an already-open overlay.
    pub fn switch(&mut self, kind: OverlayKind) {
        self.open(kind);
    }

    /// Closes the active overlay. Returns whether state changed.
    pub fn close(&mut self) -> bool {
        let was_open = self.is_open();
        self.kind = OverlayKind::None;
        was_open
    }

    /// Closes a menu due to a pointer outside its owned geometry.
    pub fn dismiss_outside(&mut self) -> bool {
        self.close()
    }

    /// Closes a menu due to Escape.
    pub fn dismiss_escape(&mut self) -> bool {
        self.close()
    }

    /// Whether any transient overlay is visible.
    pub const fn is_open(&self) -> bool {
        !matches!(self.kind, OverlayKind::None)
    }

    /// Returns the currently visible overlay kind.
    pub const fn kind(&self) -> OverlayKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_closed_and_opens_a_kind() {
        let mut state = OverlayState::default();
        assert!(!state.is_open());
        state.open(OverlayKind::MainMenu);
        assert!(state.is_open());
        assert_eq!(state.kind(), OverlayKind::MainMenu);
    }

    #[test]
    fn opening_a_second_kind_replaces_the_first() {
        let mut state = OverlayState::default();
        state.open(OverlayKind::MainMenu);
        state.switch(OverlayKind::ContextMenu);
        assert_eq!(state.kind(), OverlayKind::ContextMenu);
    }

    #[test]
    fn outside_click_closes_once() {
        let mut state = OverlayState::default();
        state.open(OverlayKind::FloatingPanel);
        assert!(state.dismiss_outside());
        assert!(!state.dismiss_outside());
        assert_eq!(state.kind(), OverlayKind::None);
    }

    #[test]
    fn replacement_and_repeated_dismissal_report_state_transitions() {
        let mut state = OverlayState::default();
        assert!(!state.dismiss_escape());
        state.open(OverlayKind::MainMenu);
        state.switch(OverlayKind::ContextMenu);
        assert_eq!(state.kind(), OverlayKind::ContextMenu);
        assert!(state.dismiss_outside());
        assert!(!state.dismiss_escape());
        state.open(OverlayKind::FloatingPanel);
        assert!(state.dismiss_escape());
        assert!(!state.close());
        assert_eq!(state.kind(), OverlayKind::None);
    }

    #[test]
    fn escape_closes_the_active_overlay() {
        let mut state = OverlayState::default();
        state.open(OverlayKind::ContextMenu);
        assert!(state.dismiss_escape());
        assert!(!state.is_open());
    }
}
