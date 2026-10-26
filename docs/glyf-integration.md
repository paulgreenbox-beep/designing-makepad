# Gradual GLYF integration plan

Status: proposed navigation/overlay adoption guide. GLYF already consumes shared style and menu helpers; this guide does not claim that every component below has been adopted or behavior-tested in GLYF.

## Recommended first move: adopt navigation only

Start with tabs because `TabRegistry<T>` has no document-domain dependency. In a consuming workspace, add a dependency pinned to the release tag or a reviewed immutable commit:

```toml
[dependencies]
designing-makepad = { git = "https://github.com/paulgreenbox-beep/designing-makepad.git", tag = "v0.1.0" }
```

Then keep GLYF's real document/session state as the generic payload:

```rust
use designing_makepad::{DocumentTabFrame, TabRegistry};

struct GlyfTabPayload {
    document_id: GlyfDocumentId,
    // Existing GLYF session/document state stays owned by GLYF.
}

type GlyfTabId = designing_makepad::TabId<GlyfTabPayload>;

struct GlyfShell {
    tabs: TabRegistry<GlyfTabPayload>,
}
```

The shell projects that model before drawing:

```rust
let active = self.tabs.active();
let frames = self.tabs.tabs().map(|(id, tab)| DocumentTabFrame {
    id: id.get(),
    title: tab.title().to_owned(),
    dirty: tab.is_dirty(),
    closable: tab.is_closable(),
    active: active == Some(id),
}).collect();
```

And maps widget actions back into the GLYF command layer:

```rust
match action {
    DocumentTabStripAction::SelectTab(raw) => {
        if let Some(id) = self.tabs.id_from_raw(raw) {
            self.tabs.activate(id)?;
        }
    }
    DocumentTabStripAction::CloseTab(raw) => {
        if let Some(id) = self.tabs.id_from_raw(raw) {
            self.request_close_glyf_document(id)?;
        }
    }
    DocumentTabStripAction::NewTab => self.new_glyf_document()?,
    DocumentTabStripAction::None => {}
}
```

Do not make a document title, GLYF path, or a Makepad `LiveId` canonical identity. `TabId` is stable across visual reordering and is the value used for targeted close/select operations.

## Shell placement

In the top-level GLYF window, preserve this nesting/order:

```text
Window (native title bar)
  document tabs: DocumentTabStrip
  menu bar
  toolbar
  main GLYF canvas/content
  status bar
  OverlayHost (root-level sibling above normal shell content)
```

The tab strip stays immediately below the native title bar. It is not a card inside ordinary content and is not a separate pseudo-window.

## Overlay and dialog policy

Use `OverlayHost` for short-lived, mouse-first UI such as application menus and canvas context menus. It owns popup z-order, absolute screen-space placement, outside-click dismissal, Escape dismissal, and scroll blocking.

Keep popups that contain a live `TextInput`, selection handles, IME interaction, or long-form editing in GLYF's normal widget tree. That preserves focus and avoids hiding text editing behind a generic overlay abstraction.

Use Makepad's official `Modal` for application dialogs such as preferences, confirmation, and import/export workflows. Do not replace it with a custom overlay merely for visual consistency.

## Theme adoption

`designing_makepad::theme::TOKENS` is an inventory and naming contract; it maps to existing Makepad theme values. It is safe to adopt names gradually:

1. use the navigation and overlay primitives with their default Makepad theme;
2. map GLYF's existing colors to the documented `background_*`, `text*`, `accent*`, `border`, and spacing roles;
3. only then consolidate duplicate local presets.

No global visual rewrite is required for the first integration phase.

## Semantics and UI verification

The reusable stable IDs are:

```text
tabs
tab.count
tab.active
tab.item.<stable-id>
tab.item.<stable-id>.close
tab.new
overlay.active
overlay.kind
menu.file
menu.item.*
```

The Windows bridge in this lab demonstrates how to publish those semantics through `toolkit-windows-automation`. It should be copied/adapted only at the GLYF app boundary, where GLYF already owns its window lifecycle and action dispatch. The component itself remains platform-neutral.

A GLYF smoke scenario should assert the semantic state instead of screen coordinates:

1. launch GLYF and wait for `tabs`;
2. create a document with `tab.new`;
3. select and close a known stable tab ID;
4. open `menu.file` and execute a semantic menu action;
5. open and dismiss an overlay with Escape or outside click;
6. capture a screenshot and export evidence.

## Deferred work

The first phase deliberately excludes canvas selection, annotations, source clipping, GLYF import/export, and drag-reorder persistence. Those remain GLYF-domain decisions. Add them only after navigation adoption is stable and separately tested.
