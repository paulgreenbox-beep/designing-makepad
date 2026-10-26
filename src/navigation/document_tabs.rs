//! Browser/editor-style tabs rendered with Makepad's scrollable `TabBar`.
//!
//! `LiveId` only identifies an ephemeral renderer instance. The shell keeps
//! the canonical document ID and receives it back through the generic action.

use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

/// One visual document tab supplied by an application shell.
#[derive(Clone, Debug, PartialEq)]
pub struct DocumentTabFrame {
    /// Stable application-owned identity, never a display index.
    pub id: u64,
    /// Human-readable document title. It may be duplicated.
    pub title: String,
    /// Whether the document has unsaved changes.
    pub dirty: bool,
    /// Whether the component exposes the close affordance.
    pub closable: bool,
    /// Whether this document is selected by the shell.
    pub active: bool,
}

/// Actions emitted by the visual document-tab strip.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum DocumentTabStripAction {
    /// No tab operation.
    #[default]
    None,
    /// Select a stable application document ID.
    SelectTab(u64),
    /// Close a stable application document ID.
    CloseTab(u64),
    /// Ask the shell to create a new document tab.
    NewTab,
}

/// A browser/editor-style document tab row with built-in horizontal scrolling.
#[derive(Script, ScriptHook, Widget)]
pub struct DocumentTabStrip {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[redraw]
    #[live]
    tab_bar: TabBar,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
    #[rust]
    frame: Vec<DocumentTabFrame>,
}

impl DocumentTabStrip {
    /// Replaces the visual projection and schedules a redraw.
    pub fn set_tabs(&mut self, cx: &mut Cx, frame: Vec<DocumentTabFrame>) {
        self.frame = frame;
        self.tab_bar.redraw(cx);
    }

    /// Returns the visible rectangle for a stable application tab ID.
    pub fn tab_rect(&self, cx: &Cx, id: u64) -> Option<Rect> {
        self.tab_bar.tab_rect(cx, Self::renderer_key(id))
    }

    /// Returns the deliberately generous close target that this wrapper owns.
    ///
    /// Makepad's private close-button geometry is not exposed by `TabBar`; the
    /// wrapper therefore uses this same explicit hit target for both physical
    /// clicks and accessibility geometry.
    pub fn tab_close_rect(&self, cx: &Cx, id: u64) -> Option<Rect> {
        self.frame
            .iter()
            .find(|tab| tab.id == id && tab.closable)
            .and_then(|_| self.tab_rect(cx, id))
            .map(Self::close_rect_for_tab)
    }

    /// Returns the visible rectangle of the new-tab control.
    pub fn new_tab_rect(&self, cx: &Cx) -> Option<Rect> {
        self.tab_bar.tab_rect(cx, Self::new_tab_key())
    }

    /// Returns the complete horizontal-scroll viewport rectangle.
    pub fn bar_rect(&self, cx: &Cx) -> Rect {
        self.tab_bar.bar_rect(cx)
    }

    /// Maps a stable document ID to the renderer key used only by Makepad.
    pub fn renderer_key(id: u64) -> LiveId {
        LiveId::from_str_num("designing-makepad-document-tab", id)
    }

    fn new_tab_key() -> LiveId {
        LiveId::from_str_num("designing-makepad-new-tab", 1)
    }

    fn close_rect_for_tab(tab_rect: Rect) -> Rect {
        Rect {
            pos: dvec2(tab_rect.pos.x + 4.0, tab_rect.pos.y + 3.0),
            size: dvec2(
                tab_rect.size.x.min(20.0).max(1.0),
                (tab_rect.size.y - 6.0).max(1.0),
            ),
        }
    }

    fn action_at(&self, cx: &Cx, abs: DVec2) -> Option<DocumentTabStripAction> {
        if self.new_tab_rect(cx).is_some_and(|rect| rect.contains(abs)) {
            return Some(DocumentTabStripAction::NewTab);
        }
        for tab in &self.frame {
            if tab.closable
                && self
                    .tab_close_rect(cx, tab.id)
                    .is_some_and(|rect| rect.contains(abs))
            {
                return Some(DocumentTabStripAction::CloseTab(tab.id));
            }
            if self
                .tab_rect(cx, tab.id)
                .is_some_and(|rect| rect.contains(abs))
            {
                return Some(DocumentTabStripAction::SelectTab(tab.id));
            }
        }
        None
    }

    fn id_for_renderer_key(&self, key: LiveId) -> Option<u64> {
        self.frame
            .iter()
            .find_map(|tab| (Self::renderer_key(tab.id) == key).then_some(tab.id))
    }

    fn emit(&self, cx: &mut Cx, action: DocumentTabStripAction) {
        cx.widget_action_with_data(&self.action_data, self.uid, action);
    }

    fn label(tab: &DocumentTabFrame) -> String {
        if tab.dirty {
            format!("{}  •", tab.title)
        } else {
            tab.title.clone()
        }
    }
}

impl Widget for DocumentTabStrip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // The stock Tab close glyph is intentionally private. Routing physical
        // primary-button interaction through our documented geometry keeps the
        // visual hit target and automation bounds in sync.
        if let Event::MouseDown(pointer) = event
            && pointer.button == MouseButton::PRIMARY
            && let Some(action) = self.action_at(cx, pointer.abs)
        {
            self.emit(cx, action);
            return;
        }

        for action in cx.capture_actions(|cx| self.tab_bar.handle_event(cx, event, scope)) {
            match action.as_widget_action().cast() {
                TabBarAction::TabWasPressed(key) if key == Self::new_tab_key() => {
                    self.emit(cx, DocumentTabStripAction::NewTab);
                }
                TabBarAction::TabWasPressed(key) => {
                    if let Some(id) = self.id_for_renderer_key(key) {
                        self.emit(cx, DocumentTabStripAction::SelectTab(id));
                    }
                }
                TabBarAction::TabCloseWasPressed(key) => {
                    if let Some(id) = self.id_for_renderer_key(key) {
                        if self.frame.iter().any(|tab| tab.id == id && tab.closable) {
                            self.emit(cx, DocumentTabStripAction::CloseTab(id));
                        }
                    }
                }
                TabBarAction::ShouldTabStartDrag(_) | TabBarAction::None => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let active_index = self.frame.iter().position(|tab| tab.active);
        self.tab_bar.begin(cx, active_index, walk);
        for tab in &self.frame {
            let template = if tab.closable {
                id!(CloseableTab)
            } else {
                id!(PermanentTab)
            };
            self.tab_bar
                .draw_tab(cx, Self::renderer_key(tab.id), &Self::label(tab), template);
        }
        self.tab_bar
            .draw_tab(cx, Self::new_tab_key(), "+", id!(PermanentTab));
        self.tab_bar.end(cx);
        DrawStep::done()
    }
}

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.DocumentTabStripBase = #(DocumentTabStrip::register_widget(vm))
    mod.widgets.DocumentTabStrip = mod.std.set_type_default() do mod.widgets.DocumentTabStripBase {
        width: Fill
        height: 38
        tab_bar: TabBarFlat {
            height: 38
            // ScrollBarsTabs keeps a large document set usable without an
            // application-specific overflow implementation.
            CloseableTab +: {
                padding: Inset{left: 4 right: 8 top: 0 bottom: 0}
                icon_walk: Walk{width: 13 height: 13}
                draw_icon.svg: crate_resource("self:resources/icons/file-image.svg")
                close_button +: {
                    width: 20
                    height: 30
                    margin: Inset{left: 0 right: 4 top: 0 bottom: 0}
                }
            }
            PermanentTab +: {
                padding: Inset{left: 10 right: 10 top: 0 bottom: 0}
            }
            draw_bg +: {
                color: theme.color_bg_even
            }
        }
    }

    mod.widgets.ChromeStyleDocumentTabs = mod.widgets.DocumentTabStrip{}
}
