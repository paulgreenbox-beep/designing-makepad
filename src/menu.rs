//! Pure state and geometry for reusable desktop application menus.

/// Data model for a reusable menu tree. Applications retain ownership of the
/// commands behind these items; the model only describes structure and intent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MenuItem {
    Command {
        id: String,
        label: String,
        icon: Option<String>,
    },
    Separator,
    Submenu {
        id: String,
        label: String,
        icon: Option<String>,
        children: Vec<MenuItem>,
    },
    Disabled {
        id: String,
        label: String,
    },
}

/// The interaction role of a [`MenuItem`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuItemKind {
    Command,
    Separator,
    Submenu,
    Disabled,
}

impl MenuItem {
    pub const fn kind(&self) -> MenuItemKind {
        match self {
            Self::Command { .. } => MenuItemKind::Command,
            Self::Separator => MenuItemKind::Separator,
            Self::Submenu { .. } => MenuItemKind::Submenu,
            Self::Disabled { .. } => MenuItemKind::Disabled,
        }
    }

    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Command { id, .. } | Self::Submenu { id, .. } | Self::Disabled { id, .. } => {
                Some(id)
            }
            Self::Separator => None,
        }
    }
}

/// A heading in a conventional desktop menu bar.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DesktopMenu {
    File,
    Edit,
    View,
    Window,
    Language,
    Help,
}

impl DesktopMenu {
    /// Stable order used by menu bars and component catalogs.
    pub const ALL: &[Self] = &[
        Self::File,
        Self::Edit,
        Self::View,
        Self::Window,
        Self::Language,
        Self::Help,
    ];

    /// Machine-friendly key shared with applications' semantic IDs.
    pub const fn key(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Edit => "edit",
            Self::View => "view",
            Self::Window => "window",
            Self::Language => "language",
            Self::Help => "help",
        }
    }

    /// Default English label. Applications may localize this at their edge.
    pub const fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Window => "Window",
            Self::Language => "Language",
            Self::Help => "Help",
        }
    }

    /// Resolves only documented menu keys, never arbitrary overlay names.
    pub fn from_key(key: &str) -> Option<Self> {
        Some(match key {
            "file" => Self::File,
            "edit" => Self::Edit,
            "view" => Self::View,
            "window" => Self::Window,
            "language" => Self::Language,
            "help" => Self::Help,
            _ => return None,
        })
    }
}

/// Result of a menu-bar transition. The shell uses this to decide whether to
/// redraw and re-anchor a root overlay; command execution remains application-owned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuTransition {
    Unchanged,
    Opened(DesktopMenu),
    Switched { from: DesktopMenu, to: DesktopMenu },
    Closed(DesktopMenu),
}

/// Canonical state for exactly one visible desktop menu.
///
/// It deliberately has no geometry or widget references. The UI shell owns
/// anchoring, drawing, input routing and commands; this type makes it
/// impossible for two top-level menus to be open at once.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MenuState {
    active: Option<DesktopMenu>,
}

impl MenuState {
    /// Returns the only open heading, if there is one.
    pub const fn active(self) -> Option<DesktopMenu> {
        self.active
    }

    /// Whether any desktop menu surface is currently visible.
    pub const fn is_open(self) -> bool {
        self.active.is_some()
    }

    /// Opens a heading or replaces the active heading in one transition.
    pub fn open(&mut self, menu: DesktopMenu) -> MenuTransition {
        match self.active {
            Some(active) if active == menu => MenuTransition::Unchanged,
            Some(from) => {
                self.active = Some(menu);
                MenuTransition::Switched { from, to: menu }
            }
            None => {
                self.active = Some(menu);
                MenuTransition::Opened(menu)
            }
        }
    }

    /// A second click on the current heading closes it; another heading
    /// switches directly without a transient closed state.
    pub fn toggle(&mut self, menu: DesktopMenu) -> MenuTransition {
        if self.active == Some(menu) {
            self.close()
        } else {
            self.open(menu)
        }
    }

    /// Closes the active menu after a command, outside click or Escape.
    pub fn close(&mut self) -> MenuTransition {
        match self.active.take() {
            Some(menu) => MenuTransition::Closed(menu),
            None => MenuTransition::Unchanged,
        }
    }
}

/// Shared state for one root menu and its visible cascade.
///
/// The path deliberately contains string keys rather than widget IDs: views
/// can be recreated during a redraw while the semantic menu state stays valid.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MenuChainState {
    root: MenuState,
    submenu_path: Vec<String>,
}

impl MenuChainState {
    pub const fn root(&self) -> Option<DesktopMenu> {
        self.root.active()
    }

    pub const fn is_open(&self) -> bool {
        self.root.is_open()
    }

    pub fn submenu_path(&self) -> &[String] {
        &self.submenu_path
    }

    pub fn active_submenu(&self) -> Option<&str> {
        self.submenu_path.last().map(String::as_str)
    }

    pub fn open_root(&mut self, menu: DesktopMenu) -> MenuTransition {
        let transition = self.root.open(menu);
        if transition != MenuTransition::Unchanged {
            self.submenu_path.clear();
        }
        transition
    }

    pub fn toggle_root(&mut self, menu: DesktopMenu) -> MenuTransition {
        let transition = self.root.toggle(menu);
        if transition != MenuTransition::Unchanged {
            self.submenu_path.clear();
        }
        transition
    }

    /// Opens or replaces a first-level submenu below the current root.
    pub fn open_submenu(&mut self, submenu: impl Into<String>) -> bool {
        if !self.is_open() {
            return false;
        }
        let submenu = submenu.into();
        if self.submenu_path.len() == 1 && self.submenu_path[0] == submenu {
            return false;
        }
        self.submenu_path.clear();
        self.submenu_path.push(submenu);
        true
    }

    /// Adds a child submenu to the existing cascade.
    pub fn open_nested_submenu(&mut self, submenu: impl Into<String>) -> bool {
        if !self.is_open() {
            return false;
        }
        let submenu = submenu.into();
        if self.active_submenu() == Some(submenu.as_str()) {
            return false;
        }
        self.submenu_path.push(submenu);
        true
    }

    /// Closes all descendants while keeping the root menu visible.
    pub fn close_submenus(&mut self) -> bool {
        if self.submenu_path.is_empty() {
            return false;
        }
        self.submenu_path.clear();
        true
    }

    /// Closes the entire cascade after a command, outside click or Escape.
    pub fn close(&mut self) -> MenuTransition {
        self.submenu_path.clear();
        self.root.close()
    }
}

/// A platform-neutral menu rectangle in screen coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MenuRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl MenuRect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(self) -> f64 {
        self.y + self.height
    }
}

/// Direction selected for a visible submenu surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmenuDirection {
    Right,
    Left,
}

/// Result of placing a submenu beside a parent menu item.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SubmenuPlacement {
    pub rect: MenuRect,
    pub direction: SubmenuDirection,
}

/// Places a submenu to the right when possible, otherwise falls back to the
/// left and clamps it vertically into the viewport.
pub fn place_submenu(
    parent: MenuRect,
    submenu_size: (f64, f64),
    viewport: MenuRect,
    gap: f64,
) -> SubmenuPlacement {
    let (width, height) = submenu_size;
    let right_x = parent.right() + gap;
    let left_x = parent.x - gap - width;
    let fits_right = right_x + width <= viewport.right();
    let direction = if fits_right {
        SubmenuDirection::Right
    } else {
        SubmenuDirection::Left
    };
    let x = if direction == SubmenuDirection::Right {
        right_x
    } else {
        left_x
    }
    .clamp(viewport.x, (viewport.right() - width).max(viewport.x));
    let y = parent
        .y
        .clamp(viewport.y, (viewport.bottom() - height).max(viewport.y));
    SubmenuPlacement {
        rect: MenuRect::new(x, y, width, height),
        direction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_heading_switch_keeps_exactly_one_menu_open() {
        let mut state = MenuState::default();
        assert_eq!(
            state.open(DesktopMenu::File),
            MenuTransition::Opened(DesktopMenu::File)
        );
        assert_eq!(
            state.open(DesktopMenu::Edit),
            MenuTransition::Switched {
                from: DesktopMenu::File,
                to: DesktopMenu::Edit,
            }
        );
        assert_eq!(state.active(), Some(DesktopMenu::Edit));
    }

    #[test]
    fn toggle_and_command_close_are_idempotent() {
        let mut state = MenuState::default();
        state.toggle(DesktopMenu::View);
        assert_eq!(
            state.toggle(DesktopMenu::View),
            MenuTransition::Closed(DesktopMenu::View)
        );
        assert_eq!(state.close(), MenuTransition::Unchanged);
    }

    #[test]
    fn only_documented_keys_resolve_to_menu_headings() {
        assert_eq!(
            DesktopMenu::from_key("language"),
            Some(DesktopMenu::Language)
        );
        assert_eq!(DesktopMenu::from_key("settings"), None);
    }

    #[test]
    fn cascade_closes_descendants_on_root_switch_and_close() {
        let mut state = MenuChainState::default();
        state.open_root(DesktopMenu::File);
        assert!(state.open_submenu("recent"));
        assert_eq!(state.active_submenu(), Some("recent"));
        state.open_root(DesktopMenu::Edit);
        assert_eq!(state.root(), Some(DesktopMenu::Edit));
        assert!(state.submenu_path().is_empty());
        state.open_submenu("history");
        state.close();
        assert!(!state.is_open());
        assert!(state.submenu_path().is_empty());
    }

    #[test]
    fn cascade_replaces_first_level_and_supports_nested_children() {
        let mut state = MenuChainState::default();
        state.open_root(DesktopMenu::File);
        assert!(state.open_submenu("recent"));
        assert!(state.open_submenu("export"));
        assert_eq!(state.submenu_path(), &["export"]);
        assert!(state.open_nested_submenu("pdf"));
        assert_eq!(state.submenu_path(), &["export", "pdf"]);
        assert!(state.close_submenus());
        assert_eq!(state.active_submenu(), None);
    }

    #[test]
    fn closed_chain_rejects_submenus_and_reopen_has_no_stale_path() {
        let mut state = MenuChainState::default();
        assert!(!state.open_submenu("recent"));
        assert!(!state.open_nested_submenu("child"));
        assert!(!state.close_submenus());
        state.open_root(DesktopMenu::File);
        assert!(state.open_submenu("recent"));
        assert!(state.open_nested_submenu("child"));
        assert_eq!(state.close(), MenuTransition::Closed(DesktopMenu::File));
        assert_eq!(state.close(), MenuTransition::Unchanged);
        state.open_root(DesktopMenu::Edit);
        assert_eq!(state.root(), Some(DesktopMenu::Edit));
        assert!(state.submenu_path().is_empty());
    }

    #[test]
    fn unchanged_root_keeps_cascade_but_toggle_closes_it() {
        let mut state = MenuChainState::default();
        state.open_root(DesktopMenu::File);
        assert!(state.open_submenu("recent"));
        assert!(!state.open_submenu("recent"));
        assert!(state.open_nested_submenu("child"));
        assert!(!state.open_nested_submenu("child"));
        assert_eq!(state.open_root(DesktopMenu::File), MenuTransition::Unchanged);
        assert_eq!(state.submenu_path(), &["recent", "child"]);
        assert_eq!(state.toggle_root(DesktopMenu::File), MenuTransition::Closed(DesktopMenu::File));
        assert!(!state.is_open());
        assert!(state.submenu_path().is_empty());
    }

    #[test]
    fn replacing_first_level_submenu_discards_nested_descendants() {
        let mut state = MenuChainState::default();
        state.open_root(DesktopMenu::View);
        state.open_submenu("one");
        state.open_nested_submenu("child");
        assert!(state.open_submenu("two"));
        assert_eq!(state.submenu_path(), &["two"]);
        assert!(state.close_submenus());
        assert!(!state.close_submenus());
        assert_eq!(state.root(), Some(DesktopMenu::View));
    }

    #[test]
    fn outside_click_can_close_the_entire_cascade() {
        let mut state = MenuChainState::default();
        state.open_root(DesktopMenu::File);
        state.open_submenu("recent");
        state.close();
        assert_eq!(state.root(), None);
        assert!(state.submenu_path().is_empty());
    }
    #[test]
    fn submenu_placement_prefers_right_and_flips_left() {
        let viewport = MenuRect::new(0.0, 0.0, 800.0, 600.0);
        let right = place_submenu(
            MenuRect::new(80.0, 40.0, 220.0, 28.0),
            (200.0, 180.0),
            viewport,
            4.0,
        );
        assert_eq!(right.direction, SubmenuDirection::Right);
        assert_eq!(right.rect.x, 304.0);

        let left = place_submenu(
            MenuRect::new(650.0, 40.0, 140.0, 28.0),
            (200.0, 180.0),
            viewport,
            4.0,
        );
        assert_eq!(left.direction, SubmenuDirection::Left);
        assert_eq!(left.rect.x, 446.0);
    }

    #[test]
    fn submenu_placement_clamps_to_viewport_vertically() {
        let viewport = MenuRect::new(10.0, 20.0, 400.0, 300.0);
        let placement = place_submenu(
            MenuRect::new(100.0, 290.0, 120.0, 28.0),
            (180.0, 160.0),
            viewport,
            4.0,
        );
        assert_eq!(placement.rect.y, 160.0);
        assert!(placement.rect.bottom() <= viewport.bottom());
    }

    #[test]
    fn menu_item_carries_nested_structure_and_kind() {
        let item = MenuItem::Submenu {
            id: "recent".into(),
            label: "Open Recent".into(),
            icon: Some("history".into()),
            children: vec![MenuItem::Disabled {
                id: "recent.empty".into(),
                label: "No recent files".into(),
            }],
        };
        assert_eq!(item.kind(), MenuItemKind::Submenu);
        assert_eq!(item.id(), Some("recent"));
    }
}
