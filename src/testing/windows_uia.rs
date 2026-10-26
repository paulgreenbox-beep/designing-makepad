//! Windows-only bridge owner for the Designing Makepad semantic snapshot.

#[cfg(windows)]
use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};
#[cfg(windows)]
use toolkit_windows_automation::{
    AccessibilityActionHandler, AccessibilityActionRequest, AccessibilityRect, AccessibilityTree,
    WindowsAccessibilityBridge,
};
#[cfg(windows)]
use windows_uia::{
    Win32::{
        Foundation::{HWND, POINT, RECT},
        Graphics::Gdi::ClientToScreen,
        UI::{
            HiDpi::GetDpiForWindow,
            WindowsAndMessaging::{FindWindowW, GetWindowRect, GetWindowThreadProcessId},
        },
    },
    core::PCWSTR,
};

#[cfg(windows)]
thread_local! {
    static BRIDGE: RefCell<Option<WindowsAccessibilityBridge>> = const { RefCell::new(None) };
    static GENERATION: Cell<u64> = const { Cell::new(0) };
}

/// An accessibility action marshalled from UIA back to the Makepad UI thread.
#[cfg(windows)]
#[derive(Clone, Debug)]
pub struct AccessibilityUiAction(pub AccessibilityActionRequest);

/// Converts Makepad client-space rectangles to UIA screen-space physical pixels.
#[cfg(windows)]
pub struct ScreenTransform {
    origin: POINT,
    dpi_factor: f64,
}

#[cfg(windows)]
impl ScreenTransform {
    fn for_window(hwnd: HWND) -> Result<Self, String> {
        let mut origin = POINT::default();
        unsafe { ClientToScreen(hwnd, &mut origin) }
            .ok()
            .map_err(|error| error.to_string())?;
        let dpi = unsafe { GetDpiForWindow(hwnd) }.max(96);
        Ok(Self {
            origin,
            dpi_factor: dpi as f64 / 96.0,
        })
    }

    pub fn rect(&self, rect: makepad_widgets::Rect) -> AccessibilityRect {
        AccessibilityRect {
            x: self.origin.x as f64 + rect.pos.x * self.dpi_factor,
            y: self.origin.y as f64 + rect.pos.y * self.dpi_factor,
            width: rect.size.x * self.dpi_factor,
            height: rect.size.y * self.dpi_factor,
        }
    }
}

/// Publishes a fresh tree, attaching the provider to this process's window once.
#[cfg(windows)]
pub fn publish(
    window_title: &str,
    build_tree: impl FnOnce(
        &ScreenTransform,
        AccessibilityRect,
        u64,
    ) -> Result<AccessibilityTree, String>,
) {
    let Ok(hwnd) = find_window(window_title) else {
        return;
    };
    let Ok(screen) = ScreenTransform::for_window(hwnd) else {
        return;
    };
    let Ok(window_bounds) = window_bounds(hwnd) else {
        return;
    };
    let generation = GENERATION.with(|slot| {
        let next = slot.get().wrapping_add(1);
        slot.set(next);
        next
    });
    let Ok(tree) = build_tree(&screen, window_bounds, generation) else {
        return;
    };

    BRIDGE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(bridge) = slot.as_ref() {
            let _ = bridge.update(tree);
            return;
        }
        let handler: AccessibilityActionHandler = Arc::new(|request| {
            makepad_widgets::Cx::post_action(AccessibilityUiAction(request));
            Ok(())
        });
        if let Ok(bridge) = WindowsAccessibilityBridge::attach(hwnd, tree, handler) {
            *slot = Some(bridge);
        }
    });
}

#[cfg(windows)]
fn window_bounds(hwnd: HWND) -> Result<AccessibilityRect, String> {
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect) }.map_err(|error| error.to_string())?;
    Ok(AccessibilityRect {
        x: rect.left as f64,
        y: rect.top as f64,
        width: (rect.right - rect.left).max(0) as f64,
        height: (rect.bottom - rect.top).max(0) as f64,
    })
}

#[cfg(windows)]
fn find_window(title: &str) -> Result<HWND, String> {
    let wide = title
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let hwnd =
        unsafe { FindWindowW(None, PCWSTR(wide.as_ptr())) }.map_err(|error| error.to_string())?;
    let mut process_id = 0;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };
    if process_id != std::process::id() {
        return Err("matched window belongs to another process".into());
    }
    Ok(hwnd)
}
