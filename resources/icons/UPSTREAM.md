# Curated Lucide icon subset

Source repository: <https://github.com/lucide-icons/lucide>

Downloaded: 2026-08-24 from `main/icons/`.

This directory intentionally contains only the 38 icons declared by
`designing_makepad::IconName`; it is not a vendored copy of the full Lucide
catalog. The complete upstream `LICENSE-LUCIDE.txt` is retained verbatim next
to the SVG files. It contains the ISC licence for Lucide and the embedded MIT
notice required for any assets derived from Feather.

The visual API is the document-agnostic `IconName` / `IconButtonSpec` Rust
model plus the `DsIcon*` and `DsIconButton*` Makepad presets. Applications
must provide their own command semantics and accessibility IDs.

## Signal compatibility filenames

For the public source release, the 17 files under `signal/` are copies of corresponding curated Lucide icons. The previous drawings had no recorded provenance and are preserved only in a local, unpublished report archive. The public copies retain Lucide ISC and Feather MIT terms from `LICENSE-LUCIDE.txt`; they are not claimed as original artwork.

| Compatibility name | Lucide source |
| --- | --- |
| signal/signal-new.svg | file-plus.svg |
| signal/signal-open.svg | folder-open.svg |
| signal/signal-save.svg | save.svg |
| signal/signal-image.svg | image.svg |
| signal/signal-select.svg | mouse-pointer-2.svg |
| signal/signal-point.svg | crosshair.svg |
| signal/signal-region.svg | square-dashed-mouse-pointer.svg |
| signal/signal-fit.svg | maximize.svg |
| signal/signal-zoom-in.svg | zoom-in.svg |
| signal/signal-zoom-out.svg | zoom-out.svg |
| signal/signal-layers.svg | layers.svg |
| signal/signal-package.svg | package.svg |
| signal/signal-undo.svg | undo-2.svg |
| signal/signal-redo.svg | redo-2.svg |
| signal/signal-copy.svg | copy.svg |
| signal/signal-cut.svg | scissors.svg |
| signal/signal-paste.svg | clipboard.svg |
