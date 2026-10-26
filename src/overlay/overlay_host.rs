//! A generic root-level host for transient mouse-first overlays.

use makepad_widgets::*;

/// Reason an [`OverlayHost`] dismissed itself.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum OverlayHostAction {
    /// A pointer landed outside the popup geometry.
    DismissedByOutside,
    /// The user pressed Escape while the overlay was active.
    DismissedByEscape,
    /// No operation.
    #[default]
    None,
}

/// A normal root-level Makepad view that draws popup content above shell UI.
///
/// Menus and context menus live in `content`, whose absolute position is owned
/// by the shell. Text-editing popups are deliberately not forced through this
/// primitive: normal widgets are more reliable for `TextInput` focus.
#[derive(Script, ScriptHook, Widget)]
pub struct OverlayHost {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    opened: bool,
    #[rust]
    dismiss_on_outside: bool,
    #[rust]
    popup_pos: Vec2d,
    #[rust]
    popup_size: Vec2d,
    #[rust]
    chain_mode: bool,
}

impl Widget for OverlayHost {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.opened {
            return;
        }

        self.view.handle_event(cx, event, scope);

        if let Event::KeyDown(key) = event
            && key.key_code == KeyCode::Escape
        {
            self.hide(cx);
            Cx::post_action(OverlayHostAction::DismissedByEscape);
            return;
        }

        if !self.dismiss_on_outside {
            return;
        }
        let bg_view = self.view.widget(cx, ids!(bg_view));
        let popup_rect = Rect {
            pos: self.popup_pos,
            size: self.popup_size,
        };
        let outside_click = match event.hits(cx, bg_view.area()) {
            Hit::FingerDown(pointer) => !popup_rect.contains(pointer.abs),
            Hit::FingerUp(pointer) => !popup_rect.contains(pointer.abs),
            _ => false,
        };
        if outside_click {
            self.hide(cx);
            Cx::post_action(OverlayHostAction::DismissedByOutside);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.opened {
            return DrawStep::done();
        }
        let step = self.view.draw_walk(cx, scope, walk);
        let content = self.view.widget(cx, ids!(content));
        cx.block_scrolling_except_within(content.area());
        step
    }
}

impl OverlayHost {
    /// Shows content above normal layout at a shell-owned absolute position.
    pub fn show_at(&mut self, cx: &mut Cx, pos: Vec2d, size: Vec2d, dismiss_on_outside: bool) {
        self.popup_pos = pos;
        self.popup_size = size;
        self.dismiss_on_outside = dismiss_on_outside;
        self.chain_mode = false;
        self.opened = true;
        if let Some(mut content) = self.view.widget(cx, ids!(content)).borrow_mut::<View>() {
            content.walk.abs_pos = Some(pos);
            content.walk.width = Size::fit();
            content.walk.height = Size::fit();
            content.redraw(cx);
        }
        self.view.redraw(cx);
    }

    /// Shows a cascade container whose child menu surfaces own their own
    /// absolute positions. Applications still own the chain geometry and
    /// outside-click policy, so this remains useful for nested menus and
    /// complex document shells without introducing a second event model.
    pub fn show_chain(&mut self, cx: &mut Cx, dismiss_on_outside: bool) {
        self.dismiss_on_outside = dismiss_on_outside;
        self.chain_mode = true;
        self.opened = true;
        if let Some(mut content) = self.view.widget(cx, ids!(content)).borrow_mut::<View>() {
            content.walk.abs_pos = None;
            content.walk.width = Size::fill();
            content.walk.height = Size::fill();
            content.redraw(cx);
        }
        self.view.redraw(cx);
    }

    /// Hides content and releases the overlay's scrolling block.
    pub fn hide(&mut self, cx: &mut Cx) {
        if !self.opened {
            return;
        }
        self.opened = false;
        self.dismiss_on_outside = false;
        self.chain_mode = false;
        self.view.redraw(cx);
        cx.unblock_scrolling();
    }

    /// Whether this host is currently visible.
    pub const fn is_open(&self) -> bool {
        self.opened
    }
}

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.OverlayHostBase = #(OverlayHost::register_widget(vm))
    mod.widgets.OverlayHost = mod.std.set_type_default() do mod.widgets.OverlayHostBase{
        width: Fill
        height: Fill
        flow: Overlay
        align: Align {x: 0.0 y: 0.0}

        // A transparent real Fill child keeps hit testing in the normal widget
        // tree rather than relying on a DrawList portal.
        bg_view := View {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg +: {
                color: #x00000000
                pixel: fn() {
                    return self.color
                }
            }
        }

        content := View {
            width: Fit
            height: Fit
            flow: Overlay
            align: Align {x: 0.0 y: 0.0}
            clip_x: false
            clip_y: false
        }
    }
}
