//! Executable showcase for the reusable Designing Makepad desktop UI layer.

use designing_makepad::{
    DesktopMenu, DocumentTabFrame, DocumentTabStrip, DocumentTabStripAction, MenuChainState,
    MenuItem, MenuRect, OverlayHost, OverlayHostAction, OverlayKind, OverlayState, StyleProfile,
    StyleTuning, TabId, TabRegistry, place_submenu, semantics, theme,
};
use makepad_widgets::*;

// The showcase-only UIA adapter lives with testing/semantics concerns.
#[path = "testing/windows_uia.rs"]
#[cfg(all(windows, feature = "toolkit-uia"))]
mod automation;

mod style_lab;
#[cfg(all(windows, feature = "toolkit-uia"))]
use toolkit_windows_automation::{
    AccessibilityAction, AccessibilityActionRequest, AccessibilityActions, AccessibilityNode,
    AccessibilityRect, AccessibilityRole, AccessibilitySelection, AccessibilityTree,
};

const WINDOW_TITLE: &str = "Designing Makepad";

app_main!(App);

#[derive(Debug)]
struct DemoDocument {
    number: u64,
}

type DemoTabId = TabId<DemoDocument>;

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    tabs: TabRegistry<DemoDocument>,
    #[rust]
    next_number: u64,
    #[rust]
    overlay: OverlayState,
    #[rust]
    menu_chain: MenuChainState,
    #[rust]
    recent_documents: Vec<String>,
    #[rust]
    active_menu: String,
    #[rust]
    overlay_pos: Option<DVec2>,
    #[rust]
    overlay_size: DVec2,
    #[rust]
    status_note: String,
    #[rust]
    active_tooltip: String,
    #[rust]
    style_profile: StyleProfile,
    #[rust]
    style_tuning: StyleTuning,
    #[rust]
    style_note: String,
    #[rust]
    style_revision: u64,
}

impl App {
    fn add_named_tab(&mut self, title: impl Into<String>, dirty: bool) -> DemoTabId {
        self.next_number += 1;
        let id = self
            .tabs
            .add(
                title,
                DemoDocument {
                    number: self.next_number,
                },
            )
            .expect("tab IDs cannot be exhausted by the showcase");
        self.tabs.set_dirty(id, dirty).expect("new tab is live");
        id
    }

    fn new_tab(&mut self) {
        let number = self.next_number + 1;
        self.add_named_tab(format!("untitled-{number}.glyf"), false);
        self.status_note = format!("Created document tab {number}");
    }

    fn frames(&self) -> Vec<DocumentTabFrame> {
        let active = self.tabs.active();
        self.tabs
            .tabs()
            .map(|(id, tab)| DocumentTabFrame {
                id: id.get(),
                title: tab.title().to_owned(),
                dirty: tab.is_dirty(),
                closable: tab.is_closable(),
                active: active == Some(id),
            })
            .collect()
    }

    fn refresh(&mut self, cx: &mut Cx) {
        if let Some(mut strip) = self
            .ui
            .widget(cx, ids!(tabs))
            .borrow_mut::<DocumentTabStrip>()
        {
            strip.set_tabs(cx, self.frames());
        }
        let content = self.tabs.active_entry().map_or_else(
            || "No tabs are open. Use + to create a document.".to_owned(),
            |(id, tab)| {
                format!(
                    "Document {}\nStable identity: tab:{}\nDirty: {}\n\nThe shell owns the model; DocumentTabStrip only renders it and emits generic actions.",
                    tab.value().number,
                    id.get(),
                    if tab.is_dirty() { "yes" } else { "no" }
                )
            },
        );
        self.ui.label(cx, ids!(content_body)).set_text(cx, &content);
        self.ui
            .label(cx, ids!(tab_count))
            .set_text(cx, &format!("{} open document tab(s)", self.tabs.len()));
        self.ui.label(cx, ids!(token_summary)).set_text(
            cx,
            &format!("{} central design tokens", theme::TOKENS.len()),
        );
        self.ui.label(cx, ids!(menu_caption)).set_text(
            cx,
            if self.active_menu.is_empty() {
                "Menu"
            } else {
                &self.active_menu
            },
        );
        let file_menu = self.file_menu_model();
        let cascade_count = file_menu
            .iter()
            .filter(|item| matches!(item, MenuItem::Submenu { .. }))
            .count();
        self.ui.label(cx, ids!(menu_model_summary)).set_text(
            cx,
            &format!("Shared MenuItem tree · {cascade_count} cascades"),
        );
        self.ui
            .button(cx, ids!(menu_open_recent))
            .set_text(cx, "Open Recent");
        self.ui.button(cx, ids!(menu_export)).set_text(cx, "Export");
        for (index, id) in [
            ids!(menu_recent_0),
            ids!(menu_recent_1),
            ids!(menu_recent_2),
        ]
        .into_iter()
        .enumerate()
        {
            self.ui.button(cx, id).set_text(
                cx,
                self.recent_documents
                    .get(index)
                    .map(String::as_str)
                    .unwrap_or(""),
            );
        }
        self.ui
            .button(cx, ids!(menu_recent_clear))
            .set_text(cx, "Clear Recent");
        let (open, kind) = semantics::overlay_state_value(self.overlay);
        self.ui.label(cx, ids!(status)).set_text(
            cx,
            &format!(
                "{} tab(s) | active: {} | overlay: {} ({}){}",
                self.tabs.len(),
                self.tabs
                    .active()
                    .map_or_else(|| "none".to_owned(), |id| format!("tab:{}", id.get())),
                open,
                kind,
                if self.status_note.is_empty() {
                    String::new()
                } else {
                    format!(" | {}", self.status_note)
                },
            ),
        );
        #[cfg(all(windows, feature = "toolkit-uia"))]
        self.refresh_accessibility(cx);
    }

    fn root_menu_size(&self) -> DVec2 {
        dvec2(252.0, 260.0)
    }

    fn menu_viewport(&self, cx: &Cx) -> Rect {
        let rect = self.ui.widget(cx, ids!(shell)).area().rect(cx);
        if rect.size.x > 2.0 && rect.size.y > 2.0 {
            rect
        } else {
            self.ui.widget(cx, ids!(workspace)).area().rect(cx)
        }
    }

    fn position_menu_panel(&mut self, cx: &mut Cx, id: &[LiveId], pos: DVec2) {
        if let Some(mut panel) = self.ui.widget(cx, id).borrow_mut::<View>() {
            panel.walk.abs_pos = Some(pos);
            panel.redraw(cx);
        }
    }

    fn root_menu_pos(&self, cx: &Cx) -> DVec2 {
        self.overlay_pos.unwrap_or_else(|| {
            let rect = self.menu_button_rect(cx, DesktopMenu::File);
            dvec2(rect.pos.x, rect.pos.y + rect.size.y)
        })
    }

    fn submenu_size(&self, key: &str) -> DVec2 {
        match key {
            "recent" if self.recent_documents.is_empty() => dvec2(300.0, 50.0),
            "recent" => dvec2(
                300.0,
                25.0 + self.recent_documents.len().min(3) as f64 * 31.0 + 46.0,
            ),
            "export" => dvec2(220.0, 118.0),
            _ => dvec2(220.0, 60.0),
        }
    }

    fn submenu_parent_rect(&self, cx: &Cx, key: &str, root_pos: DVec2) -> Rect {
        let id = match key {
            "recent" => id!(menu_open_recent),
            "export" => id!(menu_export),
            _ => id!(menu_open_recent),
        };
        let rect = self.ui.button(cx, &[id]).area().rect(cx);
        let matches_root = rect.size.x > 2.0
            && rect.size.y > 2.0
            && (rect.pos.x - root_pos.x).abs() < 80.0
            && (rect.pos.y - root_pos.y).abs() < 240.0;
        if matches_root {
            rect
        } else {
            let y = match key {
                "recent" => root_pos.y + 66.0,
                "export" => root_pos.y + 98.0,
                _ => root_pos.y + 66.0,
            };
            Rect {
                pos: dvec2(root_pos.x + 8.0, y),
                size: dvec2(self.root_menu_size().x - 16.0, 29.0),
            }
        }
    }

    fn submenu_pos(&self, cx: &Cx, key: &str, root_pos: DVec2) -> DVec2 {
        let parent = self.submenu_parent_rect(cx, key, root_pos);
        let viewport = self.menu_viewport(cx);
        let size = self.submenu_size(key);
        let placement = place_submenu(
            MenuRect::new(parent.pos.x, parent.pos.y, parent.size.x, parent.size.y),
            (size.x, size.y),
            MenuRect::new(
                viewport.pos.x,
                viewport.pos.y,
                viewport.size.x,
                viewport.size.y,
            ),
            4.0,
        );
        dvec2(placement.rect.x, placement.rect.y)
    }

    fn submenu_is_open(&self, key: &str) -> bool {
        self.overlay.kind() == OverlayKind::MainMenu
            && self.menu_chain.root() == Some(DesktopMenu::File)
            && self.menu_chain.active_submenu() == Some(key)
    }

    fn file_menu_model(&self) -> Vec<MenuItem> {
        let recent_children = if self.recent_documents.is_empty() {
            vec![MenuItem::Disabled {
                id: "recent.empty".to_owned(),
                label: "No recent files".to_owned(),
            }]
        } else {
            let mut children = self
                .recent_documents
                .iter()
                .enumerate()
                .map(|(index, title)| MenuItem::Command {
                    id: format!("recent.item.{index}"),
                    label: title.clone(),
                    icon: Some("file".to_owned()),
                })
                .collect::<Vec<_>>();
            children.push(MenuItem::Separator);
            children.push(MenuItem::Command {
                id: "recent.clear".to_owned(),
                label: "Clear Recent".to_owned(),
                icon: Some("trash".to_owned()),
            });
            children
        };
        vec![
            MenuItem::Command {
                id: "file.new".to_owned(),
                label: "New document tab".to_owned(),
                icon: Some("file-plus".to_owned()),
            },
            MenuItem::Submenu {
                id: "file.open_recent".to_owned(),
                label: "Open Recent".to_owned(),
                icon: Some("history".to_owned()),
                children: recent_children,
            },
            MenuItem::Submenu {
                id: "file.export".to_owned(),
                label: "Export".to_owned(),
                icon: Some("export".to_owned()),
                children: vec![
                    MenuItem::Command {
                        id: "export.png".to_owned(),
                        label: "PNG".to_owned(),
                        icon: Some("image".to_owned()),
                    },
                    MenuItem::Command {
                        id: "export.svg".to_owned(),
                        label: "SVG".to_owned(),
                        icon: Some("vector".to_owned()),
                    },
                    MenuItem::Command {
                        id: "export.pdf".to_owned(),
                        label: "PDF".to_owned(),
                        icon: Some("file".to_owned()),
                    },
                ],
            },
            MenuItem::Separator,
            MenuItem::Command {
                id: "file.dirty".to_owned(),
                label: "Toggle active dirty".to_owned(),
                icon: Some("sliders".to_owned()),
            },
            MenuItem::Command {
                id: "file.close".to_owned(),
                label: "Close active tab".to_owned(),
                icon: Some("close".to_owned()),
            },
        ]
    }

    fn sync_overlay(&mut self, cx: &mut Cx) {
        let menu_visible =
            self.overlay.kind() == OverlayKind::MainMenu && self.menu_chain.is_open();
        let file_menu_visible = menu_visible && self.menu_chain.root() == Some(DesktopMenu::File);
        let recent_visible = file_menu_visible && self.submenu_is_open("recent");
        let export_visible = file_menu_visible && self.submenu_is_open("export");
        self.ui
            .view(cx, ids!(menu_panel))
            .set_visible(cx, menu_visible);
        self.ui
            .view(cx, ids!(context_panel))
            .set_visible(cx, self.overlay.kind() == OverlayKind::ContextMenu);
        self.ui
            .view(cx, ids!(menu_open_recent_row))
            .set_visible(cx, file_menu_visible);
        self.ui
            .view(cx, ids!(menu_export_row))
            .set_visible(cx, file_menu_visible);
        self.ui
            .view(cx, ids!(recent_submenu))
            .set_visible(cx, recent_visible);
        self.ui
            .view(cx, ids!(export_submenu))
            .set_visible(cx, export_visible);
        self.ui
            .widget(cx, ids!(recent_empty))
            .set_visible(cx, recent_visible && self.recent_documents.is_empty());
        for (index, id) in [
            ids!(menu_recent_0),
            ids!(menu_recent_1),
            ids!(menu_recent_2),
        ]
        .into_iter()
        .enumerate()
        {
            self.ui
                .widget(cx, id)
                .set_visible(cx, recent_visible && index < self.recent_documents.len());
        }
        let show_recent_controls = recent_visible && !self.recent_documents.is_empty();
        self.ui
            .widget(cx, ids!(recent_separator))
            .set_visible(cx, show_recent_controls);
        self.ui
            .widget(cx, ids!(menu_recent_clear))
            .set_visible(cx, show_recent_controls);
    }

    fn menu_button_rect(&self, cx: &Cx, menu: DesktopMenu) -> Rect {
        let button = match menu {
            DesktopMenu::File => id!(menu_file),
            DesktopMenu::Edit => id!(menu_edit),
            DesktopMenu::View => id!(menu_view),
            DesktopMenu::Window => id!(menu_window),
            DesktopMenu::Language => id!(menu_language),
            DesktopMenu::Help => id!(menu_help),
        };
        self.ui.button(cx, &[button]).area().rect(cx)
    }
    fn layout_menu_chain(&mut self, cx: &mut Cx) {
        if self.overlay.kind() != OverlayKind::MainMenu || !self.menu_chain.is_open() {
            return;
        }
        let root_pos = self.root_menu_pos(cx);
        self.position_menu_panel(cx, ids!(menu_panel), root_pos);
        if self.menu_chain.root() == Some(DesktopMenu::File)
            && let Some(key) = self.menu_chain.active_submenu().map(str::to_owned)
        {
            let submenu_pos = self.submenu_pos(cx, &key, root_pos);
            let panel = match key.as_str() {
                "recent" => ids!(recent_submenu),
                "export" => ids!(export_submenu),
                _ => ids!(recent_submenu),
            };
            self.position_menu_panel(cx, panel, submenu_pos);
        }
        if let Some(mut host) = self
            .ui
            .widget(cx, ids!(shell_overlay))
            .borrow_mut::<OverlayHost>()
        {
            host.show_chain(cx, false);
        }
    }

    fn open_main_menu(&mut self, cx: &mut Cx, menu: DesktopMenu, pos: DVec2) {
        self.menu_chain.open_root(menu);
        self.overlay.open(OverlayKind::MainMenu);
        self.active_menu = if menu == DesktopMenu::File {
            "File · MenuItem cascade".to_owned()
        } else {
            format!("{} menu", menu.label())
        };
        self.overlay_pos = Some(pos);
        self.overlay_size = self.root_menu_size();
        self.sync_overlay(cx);
        self.layout_menu_chain(cx);
        self.play_style_popup_open(cx, ids!(menu_panel), "Menu");
        self.status_note = format!("{} opened", self.active_menu);
        self.refresh(cx);
    }

    fn toggle_menu(&mut self, cx: &mut Cx, menu: DesktopMenu, button: LiveId) {
        if self.menu_chain.root() == Some(menu) {
            self.close_overlay(cx, "Menu closed");
            return;
        }
        let rect = self.ui.button(cx, &[button]).area().rect(cx);
        self.open_main_menu(cx, menu, dvec2(rect.pos.x, rect.pos.y + rect.size.y));
    }

    fn open_file_at_right_edge(&mut self, cx: &mut Cx) {
        let anchor = self.ui.button(cx, ids!(toolbar_flip)).area().rect(cx);
        let viewport = self.menu_viewport(cx);
        let x =
            (viewport.pos.x + viewport.size.x - self.root_menu_size().x - 8.0).max(viewport.pos.x);
        self.open_main_menu(
            cx,
            DesktopMenu::File,
            dvec2(x, anchor.pos.y + anchor.size.y),
        );
    }

    fn open_submenu(&mut self, cx: &mut Cx, key: &str) {
        if self.menu_chain.root() != Some(DesktopMenu::File)
            || self.overlay.kind() != OverlayKind::MainMenu
        {
            let rect = self.menu_button_rect(cx, DesktopMenu::File);
            self.open_main_menu(
                cx,
                DesktopMenu::File,
                dvec2(rect.pos.x, rect.pos.y + rect.size.y),
            );
        }
        if !self.menu_chain.open_submenu(key) {
            return;
        }
        self.active_menu = "File · MenuItem cascade".to_owned();
        self.sync_overlay(cx);
        self.layout_menu_chain(cx);
        let popup = if key == "recent" {
            ids!(recent_submenu)
        } else {
            ids!(export_submenu)
        };
        self.play_style_popup_open(cx, popup, "Submenu");
        self.status_note = format!(
            "File → {} opened",
            if key == "recent" {
                "Open Recent"
            } else {
                "Export"
            }
        );
        self.refresh(cx);
    }

    fn open_submenu_on_hover(&mut self, cx: &mut Cx, event: &Event) {
        if self.menu_chain.root() != Some(DesktopMenu::File)
            || self.overlay.kind() != OverlayKind::MainMenu
        {
            return;
        }
        let Event::MouseMove(pointer) = event else {
            return;
        };
        let root_pos = self.root_menu_pos(cx);
        for key in ["recent", "export"] {
            if self
                .submenu_parent_rect(cx, key, root_pos)
                .contains(pointer.abs)
            {
                self.open_submenu(cx, key);
                return;
            }
        }
        if self.menu_chain.active_submenu().is_some()
            && (Rect {
                pos: root_pos,
                size: self.root_menu_size(),
            })
            .contains(pointer.abs)
        {
            self.menu_chain.close_submenus();
            self.sync_overlay(cx);
            self.layout_menu_chain(cx);
            self.refresh(cx);
        }
    }

    fn clear_showcase_recent(&mut self, cx: &mut Cx) {
        if self.recent_documents.is_empty() {
            self.status_note = "No recent files to clear".to_owned();
            self.refresh(cx);
            return;
        }
        self.recent_documents.clear();
        self.menu_chain.open_submenu("recent");
        self.sync_overlay(cx);
        self.layout_menu_chain(cx);
        self.status_note = "Recent files cleared".to_owned();
        self.refresh(cx);
    }

    fn open_showcase_recent(&mut self, cx: &mut Cx, index: usize) {
        let Some(title) = self.recent_documents.get(index).cloned() else {
            return;
        };
        let existing = self
            .tabs
            .tabs()
            .find(|(_, tab)| tab.title() == title)
            .map(|(id, _)| id);
        if let Some(id) = existing {
            self.tabs
                .activate(id)
                .expect("recent document tab is still live");
            self.status_note = format!("Activated already-open recent tab: {title}");
        } else {
            self.add_named_tab(title.clone(), false);
            self.status_note = format!("Opened recent document: {title}");
        }
        self.close_overlay(cx, self.status_note.clone());
    }

    fn export_showcase(&mut self, cx: &mut Cx, format: &str) {
        self.close_overlay(
            cx,
            format!("Export command completed for {format} (showcase)"),
        );
    }

    fn open_context_menu(&mut self, cx: &mut Cx, pos: DVec2) {
        self.menu_chain.close();
        self.overlay.open(OverlayKind::ContextMenu);
        self.active_menu = "Context menu".to_owned();
        self.overlay_pos = Some(pos);
        self.overlay_size = dvec2(236.0, 176.0);
        self.sync_overlay(cx);
        if let Some(mut host) = self
            .ui
            .widget(cx, ids!(shell_overlay))
            .borrow_mut::<OverlayHost>()
        {
            host.show_at(cx, pos, self.overlay_size, false);
        }
        self.play_style_popup_open(cx, ids!(context_panel), "Context menu");
        self.status_note = "Context menu opened by right click".to_owned();
        self.refresh(cx);
    }

    fn close_overlay(&mut self, cx: &mut Cx, note: impl Into<String>) {
        self.menu_chain.close();
        self.overlay.close();
        self.active_menu.clear();
        self.overlay_pos = None;
        self.sync_overlay(cx);
        if let Some(mut host) = self
            .ui
            .widget(cx, ids!(shell_overlay))
            .borrow_mut::<OverlayHost>()
        {
            host.hide(cx);
        }
        self.status_note = note.into();
        self.refresh(cx);
    }

    /// Owns the physical pointer lifecycle for root-level desktop menus.
    ///
    /// `OverlayHost` owns paint/z-order only. The app compares the pointer
    /// against the same explicit popup geometry used for layout, allowing an
    /// outside click to close while another menu heading receives its event.
    fn dismiss_overlay_on_outside_pointer(&mut self, cx: &mut Cx, event: &Event) -> bool {
        if !self.overlay.is_open() {
            return false;
        }
        let pointer_abs = match event {
            Event::MouseDown(pointer) => Some(pointer.abs),
            Event::MouseUp(pointer) => Some(pointer.abs),
            _ => None,
        };
        let Some(pointer_abs) = pointer_abs else {
            return false;
        };
        let Some(pos) = self.overlay_pos else {
            return false;
        };
        let root_popup = Rect {
            pos,
            size: self.overlay_size,
        };
        let contains_submenu = self
            .menu_chain
            .active_submenu()
            .map(|key| {
                let submenu_pos = self.submenu_pos(cx, key, pos);
                Rect {
                    pos: submenu_pos,
                    size: self.submenu_size(key),
                }
                .contains(pointer_abs)
            })
            .unwrap_or(false);
        if root_popup.contains(pointer_abs) || contains_submenu {
            return false;
        }
        if self.overlay.kind() == OverlayKind::MainMenu
            && DesktopMenu::ALL
                .iter()
                .any(|menu| self.menu_button_rect(cx, *menu).contains(pointer_abs))
        {
            return false;
        }
        self.close_overlay(cx, "Overlay dismissed by outside pointer");
        true
    }
    fn active_dirty(&mut self) {
        if let Some(id) = self.tabs.active() {
            let dirty = self.tabs.get(id).expect("active tab is live").is_dirty();
            self.tabs.set_dirty(id, !dirty).expect("active tab is live");
            self.status_note = if dirty {
                "Active tab marked clean".to_owned()
            } else {
                "Active tab marked dirty".to_owned()
            };
        }
    }

    fn close_active(&mut self, cx: &mut Cx) {
        self.status_note = match self.tabs.close_active() {
            Ok(Some(_)) => "Closed active tab".to_owned(),
            Ok(None) => "No active tab to close".to_owned(),
            Err(error) => error.to_string(),
        };
        self.refresh(cx);
    }

    fn cycle(&mut self, cx: &mut Cx, reverse: bool) {
        if reverse {
            self.tabs.previous();
        } else {
            self.tabs.next();
        }
        self.status_note = if reverse {
            "Selected previous tab".to_owned()
        } else {
            "Selected next tab".to_owned()
        };
        self.refresh(cx);
    }

    fn handle_tab_action(&mut self, cx: &mut Cx, action: DocumentTabStripAction) {
        match action {
            DocumentTabStripAction::NewTab => self.new_tab(),
            DocumentTabStripAction::SelectTab(raw) => {
                self.status_note = match self.tabs.id_from_raw(raw) {
                    Some(id) => match self.tabs.activate(id) {
                        Ok(()) => format!("Selected tab:{raw}"),
                        Err(error) => error.to_string(),
                    },
                    None => format!("tab:{raw} is no longer live"),
                };
            }
            DocumentTabStripAction::CloseTab(raw) => {
                self.status_note = match self.tabs.id_from_raw(raw) {
                    Some(id) => match self.tabs.close(id) {
                        Ok(_) => format!("Closed tab:{raw}"),
                        Err(error) => error.to_string(),
                    },
                    None => format!("tab:{raw} is no longer live"),
                };
            }
            DocumentTabStripAction::None => return,
        }
        self.refresh(cx);
    }

    #[cfg(all(windows, feature = "toolkit-uia"))]
    fn refresh_accessibility(&self, cx: &Cx) {
        automation::publish(WINDOW_TITLE, |screen, window_bounds, generation| {
            self.accessibility_tree(cx, screen, window_bounds, generation)
        });
    }

    #[cfg(all(windows, feature = "toolkit-uia"))]
    fn button_node(
        id: impl Into<String>,
        name: impl Into<String>,
        bounds: AccessibilityRect,
        visible: bool,
    ) -> AccessibilityNode {
        let mut node = AccessibilityNode::new(id, AccessibilityRole::Button, name, bounds);
        node.visible = visible;
        node.actions = AccessibilityActions {
            invoke: true,
            focus: true,
            ..Default::default()
        };
        node
    }

    #[cfg(all(windows, feature = "toolkit-uia"))]
    fn accessibility_tree(
        &self,
        cx: &Cx,
        screen: &automation::ScreenTransform,
        window_bounds: AccessibilityRect,
        generation: u64,
    ) -> Result<AccessibilityTree, String> {
        use designing_makepad::semantics::id as semantic_id;

        let mut root = AccessibilityNode::new(
            semantic_id::APP,
            AccessibilityRole::Window,
            WINDOW_TITLE,
            window_bounds,
        );

        let strip_widget = self.ui.widget(cx, ids!(tabs));
        let fallback_tab_rect = strip_widget.area().rect(cx);
        let strip = strip_widget.borrow::<DocumentTabStrip>();
        let bar_rect = strip
            .as_ref()
            .map(|strip| strip.bar_rect(cx))
            .unwrap_or(fallback_tab_rect);
        let mut tab_list = AccessibilityNode::new(
            semantic_id::TABS,
            AccessibilityRole::Tab,
            "Document tabs",
            screen.rect(bar_rect),
        );
        tab_list.selection = Some(AccessibilitySelection {
            can_select_multiple: false,
            is_selection_required: false,
        });
        for (id, tab) in self.tabs.tabs() {
            let rect = strip
                .as_ref()
                .and_then(|strip| strip.tab_rect(cx, id.get()))
                .unwrap_or(bar_rect);
            let mut tab_node = AccessibilityNode::new(
                semantic_id::tab_item(id.get()),
                AccessibilityRole::TabItem,
                tab.title(),
                screen.rect(rect),
            );
            tab_node.selected = Some(self.tabs.active() == Some(id));
            tab_node.actions = AccessibilityActions {
                select: true,
                focus: true,
                ..Default::default()
            };
            if tab.is_closable() {
                let close_rect = strip
                    .as_ref()
                    .and_then(|strip| strip.tab_close_rect(cx, id.get()))
                    .unwrap_or(rect);
                tab_node.children.push(Self::button_node(
                    semantic_id::tab_close(id.get()),
                    format!("Close {}", tab.title()),
                    screen.rect(close_rect),
                    true,
                ));
            }
            tab_list.children.push(tab_node);
        }
        let new_rect = strip
            .as_ref()
            .and_then(|strip| strip.new_tab_rect(cx))
            .unwrap_or(bar_rect);
        tab_list.children.push(Self::button_node(
            semantic_id::TAB_NEW,
            "New document tab",
            screen.rect(new_rect),
            true,
        ));
        root.children.push(tab_list);

        let mut count = AccessibilityNode::new(
            semantic_id::TAB_COUNT,
            AccessibilityRole::Text,
            "Document tab count",
            screen.rect(bar_rect),
        );
        count.value = Some(self.tabs.len().to_string());
        root.children.push(count);

        let mut active = AccessibilityNode::new(
            semantic_id::TAB_ACTIVE,
            AccessibilityRole::Text,
            "Active document tab",
            screen.rect(bar_rect),
        );
        active.value = self.tabs.active().map(|id| semantic_id::tab_item(id.get()));
        root.children.push(active);

        let menu_rect = self.ui.view(cx, ids!(menu_bar)).area().rect(cx);
        let mut menu_bar = AccessibilityNode::new(
            "menu.bar",
            AccessibilityRole::MenuBar,
            "Application menu",
            screen.rect(menu_rect),
        );
        for (id, name, widget) in [
            (semantic_id::MENU_FILE, "File", id!(menu_file)),
            (semantic_id::MENU_EDIT, "Edit", id!(menu_edit)),
            (semantic_id::MENU_VIEW, "View", id!(menu_view)),
            (semantic_id::MENU_WINDOW, "Window", id!(menu_window)),
            (semantic_id::MENU_LANGUAGE, "Language", id!(menu_language)),
            (semantic_id::MENU_HELP, "Help", id!(menu_help)),
        ] {
            menu_bar.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                true,
            ));
        }
        root.children.push(menu_bar);

        let toolbar_rect = self.ui.view(cx, ids!(toolbar)).area().rect(cx);
        let mut toolbar = AccessibilityNode::new(
            "toolbar.main",
            AccessibilityRole::ToolBar,
            "Showcase actions",
            screen.rect(toolbar_rect),
        );
        for (id, name, widget) in [
            ("toolbar.new", "New tab", id!(toolbar_new)),
            ("toolbar.dirty", "Toggle active dirty", id!(toolbar_dirty)),
            ("toolbar.overlay", "Open overlay menu", id!(toolbar_overlay)),
            ("toolbar.modal", "Open settings modal", id!(toolbar_modal)),
            ("toolbar.add_many", "Add eight tabs", id!(toolbar_many)),
            (
                "toolbar.flip",
                "Open File menu at right edge",
                id!(toolbar_flip),
            ),
        ] {
            toolbar.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                true,
            ));
        }
        root.children.push(toolbar);

        let style_lab_rect = self.ui.view(cx, ids!(style_lab)).area().rect(cx);
        let mut style_lab = AccessibilityNode::new(
            semantic_id::STYLE_LAB,
            AccessibilityRole::Pane,
            "Style Lab",
            screen.rect(style_lab_rect),
        );
        for (id, name, widget) in [
            (
                semantic_id::STYLE_PALETTE,
                "Cycle palette",
                id!(style_palette),
            ),
            (semantic_id::STYLE_SHAPE, "Cycle shape", id!(style_shape)),
            (
                semantic_id::STYLE_SURFACE,
                "Cycle surface",
                id!(style_surface),
            ),
            (semantic_id::STYLE_MOTION, "Cycle motion", id!(style_motion)),
            (
                semantic_id::STYLE_PRESET,
                "Cycle style preset",
                id!(style_preset),
            ),
            (
                semantic_id::STYLE_RESET,
                "Reset production profile",
                id!(style_reset),
            ),
            (
                semantic_id::STYLE_MOTION_PROBE,
                "Run native motion probe",
                id!(style_motion_probe),
            ),
        ] {
            style_lab.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                true,
            ));
        }
        let mut style_profile = AccessibilityNode::new(
            semantic_id::STYLE_PROFILE,
            AccessibilityRole::Text,
            "Current style profile",
            screen.rect(style_lab_rect),
        );
        style_profile.value = Some(self.style_profile.summary());
        style_lab.children.push(style_profile);
        let tokens = self.style_profile.tokens(self.style_tuning);
        let mut style_motion = AccessibilityNode::new(
            semantic_id::STYLE_MOTION_STATE,
            AccessibilityRole::Text,
            "Current motion state",
            screen.rect(style_lab_rect),
        );
        style_motion.value = Some(format!(
            "{}; hover token {:.0} ms; popup token {:.0} ms",
            self.style_profile.motion.label(),
            tokens.motion.duration_fast * 1000.0,
            tokens.motion.duration_popup * 1000.0,
        ));
        style_lab.children.push(style_motion);
        let mut style_note = AccessibilityNode::new(
            semantic_id::STYLE_NOTE,
            AccessibilityRole::Text,
            "Last Style Lab interaction",
            screen.rect(style_lab_rect),
        );
        style_note.value = Some(self.style_note.clone());
        style_lab.children.push(style_note);
        root.children.push(style_lab);
        let context_rect = self.ui.view(cx, ids!(context_area)).area().rect(cx);
        let mut context = AccessibilityNode::new(
            "context.area",
            AccessibilityRole::Pane,
            "Context menu test area",
            screen.rect(context_rect),
        );
        context.actions.invoke = true;
        root.children.push(context);

        let overlay_rect = self.ui.widget(cx, ids!(shell_overlay)).area().rect(cx);
        let (overlay_open, overlay_kind) = semantics::overlay_state_value(self.overlay);
        let mut overlay_active = AccessibilityNode::new(
            semantic_id::OVERLAY_ACTIVE,
            AccessibilityRole::Text,
            "Overlay active",
            screen.rect(overlay_rect),
        );
        overlay_active.value = Some(overlay_open.to_owned());
        root.children.push(overlay_active);
        let mut overlay_kind_node = AccessibilityNode::new(
            semantic_id::OVERLAY_KIND,
            AccessibilityRole::Text,
            "Overlay kind",
            screen.rect(overlay_rect),
        );
        overlay_kind_node.value = Some(overlay_kind.to_owned());
        root.children.push(overlay_kind_node);

        let mut menu_active_node = AccessibilityNode::new(
            semantic_id::MENU_ACTIVE,
            AccessibilityRole::Text,
            "Active desktop menu",
            screen.rect(menu_rect),
        );
        menu_active_node.value = Some(
            self.menu_chain
                .root()
                .map(|menu| menu.key().to_owned())
                .unwrap_or_else(|| "closed".to_owned()),
        );
        root.children.push(menu_active_node);

        let menu_visible =
            self.overlay.kind() == OverlayKind::MainMenu && self.menu_chain.is_open();
        let file_menu_visible = menu_visible && self.menu_chain.root() == Some(DesktopMenu::File);
        for (id, name, widget) in [
            ("menu.item.new_tab", "New document tab", id!(menu_new_tab)),
            ("menu.item.dirty", "Toggle active dirty", id!(menu_dirty)),
            ("menu.item.close", "Close active tab", id!(menu_close_tab)),
            ("menu.item.dismiss", "Dismiss menu", id!(menu_dismiss)),
        ] {
            root.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                menu_visible,
            ));
        }
        for (id, name, widget) in [
            (
                "menu.item.open_recent",
                "Open Recent",
                id!(menu_open_recent),
            ),
            ("menu.item.export", "Export", id!(menu_export)),
        ] {
            root.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                file_menu_visible,
            ));
        }

        let recent_visible = self.submenu_is_open("recent");
        let export_visible = self.submenu_is_open("export");
        let mut recent_submenu = AccessibilityNode::new(
            "menu.submenu.recent",
            AccessibilityRole::Pane,
            "Open Recent submenu",
            screen.rect(self.ui.view(cx, ids!(recent_submenu)).area().rect(cx)),
        );
        recent_submenu.visible = recent_visible;
        root.children.push(recent_submenu);
        let mut export_submenu = AccessibilityNode::new(
            "menu.submenu.export",
            AccessibilityRole::Pane,
            "Export submenu",
            screen.rect(self.ui.view(cx, ids!(export_submenu)).area().rect(cx)),
        );
        export_submenu.visible = export_visible;
        root.children.push(export_submenu);
        for (index, widget) in [
            ids!(menu_recent_0),
            ids!(menu_recent_1),
            ids!(menu_recent_2),
        ]
        .into_iter()
        .enumerate()
        {
            root.children.push(Self::button_node(
                format!("menu.recent.{index}"),
                self.recent_documents
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| format!("Recent document {index}")),
                screen.rect(self.ui.button(cx, widget).area().rect(cx)),
                recent_visible && index < self.recent_documents.len(),
            ));
        }
        let mut recent_empty = AccessibilityNode::new(
            "menu.recent.empty",
            AccessibilityRole::Text,
            "No recent files",
            screen.rect(self.ui.label(cx, ids!(recent_empty)).area().rect(cx)),
        );
        recent_empty.visible = recent_visible && self.recent_documents.is_empty();
        root.children.push(recent_empty);
        root.children.push(Self::button_node(
            "menu.recent.clear",
            "Clear Recent",
            screen.rect(self.ui.button(cx, ids!(menu_recent_clear)).area().rect(cx)),
            recent_visible && !self.recent_documents.is_empty(),
        ));
        for (id, name, widget) in [
            ("menu.export.png", "Export PNG", id!(menu_export_png)),
            ("menu.export.svg", "Export SVG", id!(menu_export_svg)),
            ("menu.export.pdf", "Export PDF", id!(menu_export_pdf)),
        ] {
            root.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                export_visible,
            ));
        }
        let context_visible = self.overlay.kind() == OverlayKind::ContextMenu;
        for (id, name, widget) in [
            (
                "context.item.new_tab",
                "New document tab",
                id!(context_new_tab),
            ),
            (
                "context.item.dirty",
                "Toggle active dirty",
                id!(context_dirty),
            ),
            (
                "context.item.dismiss",
                "Dismiss context menu",
                id!(context_dismiss),
            ),
        ] {
            root.children.push(Self::button_node(
                id,
                name,
                screen.rect(self.ui.button(cx, &[widget]).area().rect(cx)),
                context_visible,
            ));
        }

        let modal_open = self.ui.modal(cx, ids!(settings_modal)).is_open();
        root.children.push(Self::button_node(
            "modal.settings.close",
            "Close settings modal",
            screen.rect(self.ui.button(cx, ids!(settings_close)).area().rect(cx)),
            modal_open,
        ));

        let status_rect = self.ui.label(cx, ids!(status)).area().rect(cx);
        let mut status = AccessibilityNode::new(
            "status",
            AccessibilityRole::StatusBar,
            "Showcase status",
            screen.rect(status_rect),
        );
        status.value = Some(self.status_note.clone());
        root.children.push(status);

        AccessibilityTree::new(root)
            .map_err(|error| error.to_string())
            .map(|tree| tree.with_generation(generation))
    }

    #[cfg(all(windows, feature = "toolkit-uia"))]
    fn handle_accessibility_action(&mut self, cx: &mut Cx, request: AccessibilityActionRequest) {
        use designing_makepad::semantics::id as semantic_id;

        let AccessibilityActionRequest { node_id, action } = request;
        if !matches!(
            action,
            AccessibilityAction::Invoke | AccessibilityAction::Select
        ) {
            return;
        }

        if let Some(raw) = node_id
            .strip_prefix("tab.item.")
            .and_then(|value| value.strip_suffix(".close"))
            .and_then(|value| value.parse::<u64>().ok())
        {
            self.handle_tab_action(cx, DocumentTabStripAction::CloseTab(raw));
            return;
        }
        if let Some(raw) = node_id
            .strip_prefix("tab.item.")
            .and_then(|value| value.parse::<u64>().ok())
        {
            self.handle_tab_action(cx, DocumentTabStripAction::SelectTab(raw));
            return;
        }
        if let Some(index) = node_id
            .strip_prefix("menu.recent.")
            .and_then(|value| value.parse::<usize>().ok())
        {
            self.open_showcase_recent(cx, index);
            return;
        }

        match node_id.as_str() {
            semantic_id::TAB_NEW | "toolbar.new" | "menu.item.new_tab" | "context.item.new_tab" => {
                self.new_tab()
            }
            "toolbar.dirty" | "menu.item.dirty" | "context.item.dirty" => self.active_dirty(),
            "toolbar.add_many" => {
                for _ in 0..8 {
                    self.new_tab();
                }
                self.status_note = "Added 8 tabs; TabBar scrolls horizontally".to_owned();
            }
            "toolbar.flip" => {
                self.open_file_at_right_edge(cx);
                return;
            }
            "toolbar.modal" => {
                self.ui.modal(cx, ids!(settings_modal)).open(cx);
                self.status_note = "Settings-style Modal opened".to_owned();
            }
            "modal.settings.close" => {
                self.ui.modal(cx, ids!(settings_modal)).close(cx);
                self.status_note = "Settings-style Modal closed".to_owned();
            }
            "menu.item.open_recent" => {
                self.open_submenu(cx, "recent");
                return;
            }
            "menu.item.export" => {
                self.open_submenu(cx, "export");
                return;
            }
            "menu.recent.clear" => {
                self.clear_showcase_recent(cx);
                return;
            }
            "menu.export.png" => {
                self.export_showcase(cx, "PNG");
                return;
            }
            "menu.export.svg" => {
                self.export_showcase(cx, "SVG");
                return;
            }
            "menu.export.pdf" => {
                self.export_showcase(cx, "PDF");
                return;
            }
            "menu.item.close" => {
                self.close_active(cx);
                self.close_overlay(cx, "Closed active tab from menu");
                return;
            }
            "menu.item.dismiss" | "context.item.dismiss" => {
                self.close_overlay(cx, "Overlay dismissed");
                return;
            }
            "context.area" => {
                let rect = self.ui.view(cx, ids!(context_area)).area().rect(cx);
                self.open_context_menu(cx, rect.pos);
                return;
            }
            semantic_id::STYLE_PALETTE => self.cycle_style_palette(cx),
            semantic_id::STYLE_SHAPE => self.cycle_style_shape(cx),
            semantic_id::STYLE_SURFACE => self.cycle_style_surface(cx),
            semantic_id::STYLE_MOTION => self.cycle_style_motion(cx),
            semantic_id::STYLE_PRESET => self.cycle_style_preset(cx),
            semantic_id::STYLE_RESET => self.reset_style_profile(cx),
            semantic_id::STYLE_MOTION_PROBE => self.run_style_motion_probe(cx),
            semantic_id::MENU_FILE => self.toggle_menu(cx, DesktopMenu::File, id!(menu_file)),
            semantic_id::MENU_EDIT => self.toggle_menu(cx, DesktopMenu::Edit, id!(menu_edit)),
            semantic_id::MENU_VIEW => self.toggle_menu(cx, DesktopMenu::View, id!(menu_view)),
            semantic_id::MENU_WINDOW => self.toggle_menu(cx, DesktopMenu::Window, id!(menu_window)),
            semantic_id::MENU_LANGUAGE => {
                self.toggle_menu(cx, DesktopMenu::Language, id!(menu_language))
            }
            semantic_id::MENU_HELP => self.toggle_menu(cx, DesktopMenu::Help, id!(menu_help)),
            _ => return,
        }
        self.refresh(cx);
    }

    fn context_click(&mut self, cx: &mut Cx, event: &Event) -> bool {
        let Event::MouseDown(pointer) = event else {
            return false;
        };
        let context_rect = self.ui.view(cx, ids!(context_area)).area().rect(cx);
        if pointer.button != MouseButton::SECONDARY || !context_rect.contains(pointer.abs) {
            return false;
        }
        self.open_context_menu(cx, pointer.abs);
        true
    }

    /// Maps visual icon controls to plain-language tooltip text at the shell
    /// edge. The reusable `IconButtonSpec` remains document-agnostic; this
    /// application chooses which concrete controls receive those commands.
    fn icon_tooltip_at(&self, cx: &Cx, abs: DVec2) -> Option<(&'static str, Rect)> {
        for (widget, tooltip) in [
            (id!(toolbar_new), "New document tab"),
            (id!(toolbar_dirty), "Toggle dirty state"),
            (id!(toolbar_overlay), "Open File menu"),
            (id!(toolbar_modal), "Open settings dialog"),
            (id!(toolbar_many), "Create eight tabs"),
            (id!(gallery_file), "New file icon"),
            (id!(gallery_folder), "Open file icon"),
            (id!(gallery_save), "Save icon"),
            (id!(gallery_pack), "Package icon"),
            (id!(gallery_pointer), "Selection icon"),
            (id!(gallery_region), "Region annotation icon"),
            (id!(gallery_crop), "Crop icon"),
            (id!(gallery_layers), "Layers icon"),
            (id!(gallery_image), "Image icon"),
            (id!(gallery_settings), "Settings icon"),
            (id!(gallery_help), "Help icon"),
        ] {
            let rect = self.ui.button(cx, &[widget]).area().rect(cx);
            if rect.contains(abs) {
                return Some((tooltip, rect));
            }
        }
        None
    }

    /// Shows one small tooltip surface for icon-first controls. It is driven
    /// from real physical pointer movement, not accessibility invocation.
    fn update_icon_tooltip(&mut self, cx: &mut Cx, event: &Event) {
        let pointer_abs = match event {
            Event::MouseMove(pointer) => Some(pointer.abs),
            Event::MouseDown(_) | Event::MouseUp(_) | Event::Scroll(_) => None,
            _ => return,
        };
        let target = pointer_abs.and_then(|abs| self.icon_tooltip_at(cx, abs));
        let next = target.map(|(tooltip, _)| tooltip).unwrap_or_default();
        if self.active_tooltip == next {
            return;
        }
        let tooltip = self.ui.tooltip(cx, ids!(tooltip_host));
        if let Some((text, rect)) = target {
            tooltip.show_with_options(cx, dvec2(rect.pos.x, rect.pos.y + rect.size.y + 6.0), text);
        } else {
            tooltip.hide(cx);
        }
        self.active_tooltip = next.to_owned();
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let menu = self.add_named_tab("menu.glyf", false);
        self.add_named_tab("hud.glyf", true);
        self.add_named_tab("notes.glyf", false);
        self.tabs.activate(menu).expect("initial tab is live");
        self.recent_documents = vec![
            "menu.glyf".to_owned(),
            "design-review.glyf".to_owned(),
            "archive-layout.glyf".to_owned(),
        ];
        self.status_note = "Ready - tabs are generic reusable state".to_owned();
        self.sync_overlay(cx);
        self.load_style_profile(cx);
        self.refresh(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        #[cfg(all(windows, feature = "toolkit-uia"))]
        for request in actions
            .iter()
            .filter_map(|action| action.downcast_ref::<automation::AccessibilityUiAction>())
            .map(|action| action.0.clone())
            .collect::<Vec<_>>()
        {
            self.handle_accessibility_action(cx, request);
        }
        if actions
            .iter()
            .filter_map(|action| action.downcast_ref::<OverlayHostAction>())
            .any(|action| {
                matches!(
                    action,
                    OverlayHostAction::DismissedByOutside | OverlayHostAction::DismissedByEscape
                )
            })
        {
            self.menu_chain.close();
            self.overlay.close();
            self.active_menu.clear();
            self.sync_overlay(cx);
            self.status_note = "Overlay dismissed".to_owned();
            self.refresh(cx);
        }

        let strip = self.ui.widget(cx, ids!(tabs));
        for action in actions.filter_widget_actions(strip.widget_uid()) {
            self.handle_tab_action(cx, action.cast());
        }

        for (button, menu) in [
            (id!(menu_file), DesktopMenu::File),
            (id!(menu_edit), DesktopMenu::Edit),
            (id!(menu_view), DesktopMenu::View),
            (id!(menu_window), DesktopMenu::Window),
            (id!(menu_language), DesktopMenu::Language),
            (id!(menu_help), DesktopMenu::Help),
        ] {
            if self.ui.button(cx, &[button]).clicked(actions) {
                self.toggle_menu(cx, menu, button);
            }
        }

        if self.ui.button(cx, ids!(toolbar_new)).clicked(actions) {
            self.new_tab();
            self.refresh(cx);
        }
        if self.ui.button(cx, ids!(toolbar_dirty)).clicked(actions) {
            self.active_dirty();
            self.refresh(cx);
        }
        if self.ui.button(cx, ids!(toolbar_overlay)).clicked(actions) {
            let rect = self.ui.button(cx, ids!(toolbar_overlay)).area().rect(cx);
            self.open_main_menu(
                cx,
                DesktopMenu::File,
                dvec2(rect.pos.x, rect.pos.y + rect.size.y),
            );
        }
        if self.ui.button(cx, ids!(toolbar_many)).clicked(actions) {
            for _ in 0..8 {
                self.new_tab();
            }
            self.status_note = "Added 8 tabs; TabBar scrolls horizontally".to_owned();
            self.refresh(cx);
        }
        if self.ui.button(cx, ids!(toolbar_flip)).clicked(actions) {
            self.open_file_at_right_edge(cx);
        }
        if self.ui.button(cx, ids!(toolbar_modal)).clicked(actions) {
            self.ui.modal(cx, ids!(settings_modal)).open(cx);
            self.status_note = "Settings-style Modal opened".to_owned();
            self.refresh(cx);
        }

        if self.ui.button(cx, ids!(style_palette)).clicked(actions) {
            self.cycle_style_palette(cx);
        }
        if self.ui.button(cx, ids!(style_shape)).clicked(actions) {
            self.cycle_style_shape(cx);
        }
        if self.ui.button(cx, ids!(style_surface)).clicked(actions) {
            self.cycle_style_surface(cx);
        }
        if self.ui.button(cx, ids!(style_motion)).clicked(actions) {
            self.cycle_style_motion(cx);
        }
        if self.ui.button(cx, ids!(style_preset)).clicked(actions) {
            self.cycle_style_preset(cx);
        }
        if self.ui.button(cx, ids!(style_reset)).clicked(actions) {
            self.reset_style_profile(cx);
        }
        if self
            .ui
            .button(cx, ids!(style_motion_probe))
            .clicked(actions)
        {
            self.run_style_motion_probe(cx);
        }
        for (id, key) in [
            (ids!(style_accent), "accent"),
            (ids!(style_radius), "radius"),
            (ids!(style_elevation), "elevation"),
            (ids!(style_opacity), "opacity"),
        ] {
            if let Some(value) = self.ui.slider(cx, id).slided(actions) {
                self.update_style_slider(cx, key, value);
            }
        }
        if self.ui.button(cx, ids!(menu_new_tab)).clicked(actions) {
            self.new_tab();
            self.close_overlay(cx, "Created tab from menu");
        }
        if self.ui.button(cx, ids!(menu_open_recent)).clicked(actions) {
            self.open_submenu(cx, "recent");
        }
        if self.ui.button(cx, ids!(menu_export)).clicked(actions) {
            self.open_submenu(cx, "export");
        }
        for (index, id) in [
            ids!(menu_recent_0),
            ids!(menu_recent_1),
            ids!(menu_recent_2),
        ]
        .into_iter()
        .enumerate()
        {
            if self.ui.button(cx, id).clicked(actions) {
                self.open_showcase_recent(cx, index);
            }
        }
        if self.ui.button(cx, ids!(menu_recent_clear)).clicked(actions) {
            self.clear_showcase_recent(cx);
        }
        if self.ui.button(cx, ids!(menu_export_png)).clicked(actions) {
            self.export_showcase(cx, "PNG");
        }
        if self.ui.button(cx, ids!(menu_export_svg)).clicked(actions) {
            self.export_showcase(cx, "SVG");
        }
        if self.ui.button(cx, ids!(menu_export_pdf)).clicked(actions) {
            self.export_showcase(cx, "PDF");
        }
        if self.ui.button(cx, ids!(menu_dirty)).clicked(actions) {
            self.active_dirty();
            self.close_overlay(cx, "Toggled dirty state from menu");
        }
        if self.ui.button(cx, ids!(menu_close_tab)).clicked(actions) {
            self.close_active(cx);
            self.close_overlay(cx, "Closed active tab from menu");
        }
        if self.ui.button(cx, ids!(menu_dismiss)).clicked(actions) {
            self.close_overlay(cx, "Menu dismissed");
        }

        if self.ui.button(cx, ids!(context_new_tab)).clicked(actions) {
            self.new_tab();
            self.close_overlay(cx, "Created tab from context menu");
        }
        if self.ui.button(cx, ids!(context_dirty)).clicked(actions) {
            self.active_dirty();
            self.close_overlay(cx, "Toggled dirty state from context menu");
        }
        if self.ui.button(cx, ids!(context_dismiss)).clicked(actions) {
            self.close_overlay(cx, "Context menu dismissed");
        }

        if self.ui.button(cx, ids!(settings_close)).clicked(actions) {
            self.ui.modal(cx, ids!(settings_modal)).close(cx);
            self.status_note = "Settings-style Modal closed".to_owned();
            self.refresh(cx);
        }
        if self.ui.modal(cx, ids!(settings_modal)).dismissed(actions) {
            self.status_note = "Settings-style Modal dismissed".to_owned();
            self.refresh(cx);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        designing_makepad::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.update_icon_tooltip(cx, event);
        if let Event::KeyDown(key) = event
            && key.key_code == KeyCode::Escape
            && self.overlay.is_open()
        {
            self.close_overlay(cx, "Overlay dismissed by Escape");
            return;
        }
        if let Event::KeyDown(key) = event
            && key.key_code == KeyCode::Escape
            && self.ui.modal(cx, ids!(settings_modal)).is_open()
        {
            self.ui.modal(cx, ids!(settings_modal)).close(cx);
            self.status_note = "Settings-style Modal dismissed by Escape".to_owned();
            self.refresh(cx);
            return;
        }
        if self.dismiss_overlay_on_outside_pointer(cx, event) {
            return;
        }
        self.open_submenu_on_hover(cx, event);
        if self.context_click(cx, event) {
            return;
        }
        if let Event::KeyDown(key) = event
            && (key.modifiers.control || key.modifiers.logo)
        {
            match key.key_code {
                KeyCode::Tab => {
                    self.cycle(cx, key.modifiers.shift);
                    return;
                }
                KeyCode::KeyW => {
                    self.close_active(cx);
                    return;
                }
                _ => {}
            }
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        // Startup builds the native window after `handle_startup`; publish once
        // more after Makepad has processed the event so UIA can attach to it.
        #[cfg(all(windows, feature = "toolkit-uia"))]
        self.refresh_accessibility(cx);
    }
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.ChromeStyleDocumentTabs
    use mod.widgets.DsButton
    use mod.widgets.DsElevatedPanel
    use mod.widgets.DsIconChevronRight
    use mod.widgets.DsIconButtonCrop
    use mod.widgets.DsIconButtonFileImage
    use mod.widgets.DsIconButtonFilePlus
    use mod.widgets.DsIconButtonFolderOpen
    use mod.widgets.DsIconButtonHelp
    use mod.widgets.DsIconButtonImage
    use mod.widgets.DsIconButtonLayers
    use mod.widgets.DsIconButtonMore
    use mod.widgets.DsIconButtonPackage
    use mod.widgets.DsIconButtonPointer
    use mod.widgets.DsIconButtonRegion
    use mod.widgets.DsIconButtonSave
    use mod.widgets.DsIconButtonSelected
    use mod.widgets.DsIconButtonSettings
    use mod.widgets.DsIconButtonSliders
    use mod.widgets.DsIconButtonWarning
    use mod.widgets.DsInput
    use mod.widgets.DsMenuBar
    use mod.widgets.DsMenuButton
    use mod.widgets.DsMenuItem
    use mod.widgets.DsMenuSeparator
    use mod.widgets.DsMenuSurface
    use mod.widgets.DsMotionProbe
    use mod.widgets.DsPanel
    use mod.widgets.DsPrimaryButton
    use mod.widgets.DsStatusBar
    use mod.widgets.DsToolbar
    use mod.widgets.DsWindow
    use mod.widgets.OverlayHost
    use mod.widgets.Tooltip

    startup() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                window.inner_size: vec2(1120, 720)
                window.title: "Designing Makepad"
                body +: {
                    width: Fill
                    height: Fill
                    flow: Overlay

                    shell := DsWindow {
                        width: Fill
                        height: Fill
                        flow: Down

                        tabs_host := View {
                            width: Fill
                            height: 38
                            flow: Down
                            tabs := ChromeStyleDocumentTabs { width: Fill height: 38 }
                        }

                        menu_bar := DsMenuBar {
                            width: Fill
                            menu_file := DsMenuButton { text: "File" }
                            menu_edit := DsMenuButton { text: "Edit" }
                            menu_view := DsMenuButton { text: "View" }
                            menu_window := DsMenuButton { text: "Window" }
                            menu_language := DsMenuButton { text: "Language" }
                            menu_help := DsMenuButton { text: "Help" }
                        }

                        toolbar := DsToolbar {
                            width: Fill
                            toolbar_new := DsIconButtonFilePlus {}
                            toolbar_dirty := DsIconButtonSliders {}
                            toolbar_overlay := DsIconButtonMore {}
                            toolbar_modal := DsIconButtonSettings {}
                            toolbar_many := DsIconButtonFileImage {}
                            toolbar_separator := View {
                                width: 1
                                height: 22
                                show_bg: true
                                draw_bg.color: theme.color_bevel
                            }
                            Label {
                                text: "Icon-first actions"
                                draw_text.color: theme.color_text_meta
                            }
                            View { width: Fill height: Fill }
                            tab_count := Label { text: "" draw_text.color: theme.color_text_meta }
                            toolbar_flip := DsButton { text: "Edge menu" }
                        }

                        workspace := View {
                            width: Fill
                            height: Fill
                            flow: Down
                            padding: Inset{left: 16 right: 16 top: 16 bottom: 16}

                            workspace_panel := DsPanel {
                                width: Fill
                                height: Fill
                                flow: Right
                                spacing: 16
                                padding: Inset{left: 22 right: 22 top: 20 bottom: 20}

                                overview := View {
                                    width: 348
                                    height: Fill
                                    flow: Down
                                    spacing: 12
                                    Label {
                                        text: "Designing Makepad"
                                        draw_text.color: theme.color_text
                                        draw_text.text_style.font_size: 22
                                    }
                                    Label {
                                        text: "Executable desktop UI laboratory for reusable Makepad editor building blocks."
                                        draw_text.color: theme.color_text_meta
                                    }
                                    content_body := Label {
                                        text: ""
                                        draw_text.color: theme.color_text
                                        draw_text.text_style.font_size: 14
                                    }

                                    context_area := DsElevatedPanel {
                                        width: Fill
                                        height: 112
                                        flow: Down
                                        spacing: 6
                                        padding: Inset{left: 14 right: 14 top: 14 bottom: 14}
                                        Label {
                                            text: "Context Menu"
                                            draw_text.color: theme.color_text
                                            draw_text.text_style.font_size: 16
                                        }
                                        Label {
                                            text: "Right-click this surface to open the same shared MenuSurface."
                                            draw_text.color: theme.color_text_meta
                                        }
                                    }

                                    Label {
                                        text: "Input + primary action"
                                        draw_text.color: theme.color_text
                                    }
                                    catalog_input := DsInput {
                                        width: Fill
                                        empty_text: "Reusable text input"
                                    }
                                    catalog_primary := DsPrimaryButton { text: "Primary action" }
                                    catalog_secondary := DsButton { text: "Secondary action" }
                                    catalog_danger := DsButton { text: "Danger action" }
                                    catalog_checkbox := CheckBox { text: "Selection enabled" active: true }
                                    catalog_toggle := Toggle { text: "Live preview" active: true }
                                    catalog_dropdown := DropDown { width: Fill labels: ["Normal" "Focused" "Disabled"] }
                                    style_canvas_mock := DsElevatedPanel {
                                        width: Fill
                                        height: 74
                                        flow: Down
                                        padding: Inset{left: 12 right: 12 top: 10 bottom: 10}
                                        Label { text: "Canvas / selected region" draw_text.color: theme.color_text }
                                        Label { text: "Accent-safe overlay surface shared with GLYF Canvas." draw_text.color: theme.color_text_meta }
                                    }
                                    Label {
                                        text: "Tabs | Menus | Toolbar | Modal | Context Menu | Theme Tokens"
                                        draw_text.color: theme.color_text_meta
                                    }
                                    token_summary := Label { text: "" draw_text.color: theme.color_text_meta }
                                }

                                style_lab := DsElevatedPanel {
                                    width: 258
                                    height: Fill
                                    flow: Down
                                    spacing: 5
                                    padding: Inset{left: 14 right: 14 top: 14 bottom: 14}
                                    Label {
                                        text: "STYLE LAB"
                                        draw_text.color: theme.color_text
                                        draw_text.text_style.font_size: 17
                                    }
                                    style_profile_summary := Label {
                                        width: Fill
                                        height: Fit
                                        text: ""
                                        draw_text.color: theme.color_text_meta
                                    }
                                    Label { text: "Palette" draw_text.color: theme.color_text_meta }
                                    style_palette := DsButton { width: Fill text: "Dark  v" }
                                    Label { text: "Shape" draw_text.color: theme.color_text_meta }
                                    style_shape := DsButton { width: Fill text: "Rounded  v" }
                                    Label { text: "Surface" draw_text.color: theme.color_text_meta }
                                    style_surface := DsButton { width: Fill text: "Elevated  v" }
                                    Label { text: "Motion" draw_text.color: theme.color_text_meta }
                                    style_motion := DsButton { width: Fill text: "Subtle  v" }
                                    DsMenuSeparator {}
                                    Label { text: "Accent" draw_text.color: theme.color_text_meta }
                                    style_accent := Slider { width: Fill min: 0 max: 1 step: 0.05 default: 0.55 precision: 2 }
                                    Label { text: "Radius" draw_text.color: theme.color_text_meta }
                                    style_radius := Slider { width: Fill min: 0 max: 1 step: 0.05 default: 0.55 precision: 2 }
                                    Label { text: "Elevation" draw_text.color: theme.color_text_meta }
                                    style_elevation := Slider { width: Fill min: 0 max: 1 step: 0.05 default: 0.55 precision: 2 }
                                    Label { text: "Opacity" draw_text.color: theme.color_text_meta }
                                    style_opacity := Slider { width: Fill min: 0.35 max: 1 step: 0.05 default: 0.90 precision: 2 }
                                    DsMenuSeparator {}
                                    Label { text: "Preset" draw_text.color: theme.color_text_meta }
                                    style_preset := DsButton { width: Fill text: "Midnight" }
                                    style_reset := DsButton { width: Fill text: "Reset" }
                                    style_motion_probe := DsMotionProbe { width: Fill text: "Play motion probe" }
                                    style_motion_summary := Label { width: Fill height: Fit text: "" draw_text.color: theme.color_text_meta }
                                    style_token_summary := Label { width: Fill height: Fit text: "" draw_text.color: theme.color_text_meta }
                                    style_note_label := Label { width: Fill height: Fit text: "" draw_text.color: theme.color_text_meta }
                                }
                                icon_catalog := DsElevatedPanel {
                                    width: Fill
                                    height: Fill
                                    flow: Down
                                    spacing: 12
                                    padding: Inset{left: 16 right: 16 top: 16 bottom: 16}
                                    Label {
                                        text: "Curated icon gallery"
                                        draw_text.color: theme.color_text
                                        draw_text.text_style.font_size: 17
                                    }
                                    Label {
                                        text: "Lucide subset — hover any icon button for a tooltip."
                                        draw_text.color: theme.color_text_meta
                                    }
                                    gallery_row_file := View {
                                        width: Fill
                                        height: 32
                                        flow: Right
                                        spacing: 6
                                        gallery_file := DsIconButtonFilePlus {}
                                        gallery_folder := DsIconButtonFolderOpen {}
                                        gallery_save := DsIconButtonSave {}
                                        gallery_pack := DsIconButtonPackage {}
                                        gallery_settings := DsIconButtonSettings {}
                                        gallery_help := DsIconButtonHelp {}
                                    }
                                    gallery_row_canvas := View {
                                        width: Fill
                                        height: 32
                                        flow: Right
                                        spacing: 6
                                        gallery_pointer := DsIconButtonPointer {}
                                        gallery_region := DsIconButtonRegion {}
                                        gallery_crop := DsIconButtonCrop {}
                                        gallery_layers := DsIconButtonLayers {}
                                        gallery_image := DsIconButtonImage {}
                                        gallery_selected := DsIconButtonSelected {
                                            draw_icon.svg: crate_resource("self:resources/icons/eye.svg")
                                        }
                                        gallery_disabled := DsIconButtonWarning { enabled: false }
                                    }
                                    DsMenuSeparator {}
                                    Label {
                                        text: "States: normal · selected · disabled"
                                        draw_text.color: theme.color_text_meta
                                    }
                                    Label {
                                        text: "Use only the icons that carry a real command or semantic meaning."
                                        draw_text.color: theme.color_text_meta
                                    }
                                }
                            }
                        }

                        status_bar := DsStatusBar {
                            width: Fill
                            status := Label { text: "Starting..." draw_text.color: theme.color_text_meta }
                        }
                    }

                    shell_overlay := OverlayHost {
                        content +: {
                            menu_panel := DsMenuSurface {
                                width: 252
                                height: Fit
                                flow: Down
                                spacing: 3
                                padding: Inset{left: 8 right: 8 top: 8 bottom: 8}
                                visible: false
                                menu_caption := Label {
                                    text: "Menu"
                                    draw_text.color: theme.color_text
                                    draw_text.text_style.font_size: 15
                                }
                                menu_model_summary := Label {
                                    text: ""
                                    draw_text.color: theme.color_text_meta
                                }
                                DsMenuSeparator {}
                                menu_new_tab := DsMenuItem { text: "New document tab" }
                                menu_open_recent_row := View {
                                    width: Fill
                                    height: 29
                                    flow: Overlay
                                    align: Align{x: 1.0 y: 0.5}
                                    menu_open_recent := DsMenuItem { text: "Open Recent" }
                                    DsIconChevronRight {
                                        width: 16
                                        height: 16
                                        margin: Inset{left: 0 right: 10 top: 0 bottom: 0}
                                    }
                                }
                                menu_export_row := View {
                                    width: Fill
                                    height: 29
                                    flow: Overlay
                                    align: Align{x: 1.0 y: 0.5}
                                    menu_export := DsMenuItem { text: "Export" }
                                    DsIconChevronRight {
                                        width: 16
                                        height: 16
                                        margin: Inset{left: 0 right: 10 top: 0 bottom: 0}
                                    }
                                }
                                DsMenuSeparator {}
                                menu_dirty := DsMenuItem { text: "Toggle active dirty" }
                                menu_close_tab := DsMenuItem { text: "Close active tab" }
                                DsMenuSeparator {}
                                menu_dismiss := DsMenuItem { text: "Dismiss menu" }
                            }

                            recent_submenu := DsMenuSurface {
                                width: 300
                                height: Fit
                                flow: Down
                                spacing: 3
                                padding: Inset{left: 8 right: 8 top: 8 bottom: 8}
                                visible: false
                                recent_empty := Label {
                                    width: Fill
                                    height: Fit
                                    text: "No recent files"
                                    draw_text.color: theme.color_text_meta
                                }
                                menu_recent_0 := DsMenuItem { text: "" visible: false }
                                menu_recent_1 := DsMenuItem { text: "" visible: false }
                                menu_recent_2 := DsMenuItem { text: "" visible: false }
                                recent_separator := DsMenuSeparator { visible: false }
                                menu_recent_clear := DsMenuItem { text: "Clear Recent" visible: false }
                            }

                            export_submenu := DsMenuSurface {
                                width: 220
                                height: Fit
                                flow: Down
                                spacing: 3
                                padding: Inset{left: 8 right: 8 top: 8 bottom: 8}
                                visible: false
                                menu_export_png := DsMenuItem { text: "PNG" }
                                menu_export_svg := DsMenuItem { text: "SVG" }
                                menu_export_pdf := DsMenuItem { text: "PDF" }
                            }
                            context_panel := DsMenuSurface {
                                width: 236
                                height: Fit
                                flow: Down
                                spacing: 3
                                padding: Inset{left: 8 right: 8 top: 8 bottom: 8}
                                visible: false
                                Label {
                                    text: "Context Menu"
                                    draw_text.color: theme.color_text
                                    draw_text.text_style.font_size: 15
                                }
                                DsMenuSeparator {}
                                context_new_tab := DsMenuItem { text: "New document tab" }
                                context_dirty := DsMenuItem { text: "Toggle active dirty" }
                                DsMenuSeparator {}
                                context_dismiss := DsMenuItem { text: "Dismiss context menu" }
                            }
                        }
                    }

                    settings_modal := Modal {
                        content +: {
                            width: 390
                            height: Fit
                            settings_modal_card := DsPanel {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12
                                padding: Inset{left: 24 right: 24 top: 22 bottom: 22}
                                Label {
                                    text: "Settings-style Modal"
                                    draw_text.color: theme.color_text
                                    draw_text.text_style.font_size: 18
                                }
                                Label {
                                    text: "This uses the official Makepad Modal primitive. Click outside, press Escape, or use Close to dismiss."
                                    draw_text.color: theme.color_text_meta
                                }
                                settings_close := DsPrimaryButton { text: "Close" }
                            }
                        }
                    }

                    tooltip_host := Tooltip {}
                }
            }
        }
    }
}
