# Public development and verification

## Public dependency

Ordinary library and GUI builds require only the public Makepad Git dependency, pinned to immutable revision `105158e08c89d47f57af4621420ed68eb3a147ce` from `https://github.com/makepad/makepad`.

Rust Makepad Toolkit is not a root manifest dependency, is not required for ordinary builds, and is not published by this project. It is future external tooling. The shared semantic helpers and IDs remain part of the public source. Windows UIA bridge sources are kept behind the optional local `toolkit-uia` integration host and are not included in the default public application build. The local host manifest example is `integrations/toolkit-uia/Cargo.toml.example`; it is for developers who already have a known Toolkit checkout. The root manifest does not define a public `toolkit-uia` feature or promise a default feature for it.

For local UIA development, copy `integrations/toolkit-uia/Cargo.toml.example` to `Cargo.toml` in the same directory, set its Toolkit crate path, and run `cargo build --manifest-path integrations/toolkit-uia/Cargo.toml`. This separate host defines the integration feature. Do not add it to the ordinary public dependency path. The optional host is not part of publication verification.

The pinned source identifies the intended public Makepad commit. This document does not claim that fresh dependency resolution or the locked commands have been validated for publication. Review the final manifest and `Cargo.lock` together before release.

## Local development

A developer may use an untracked Cargo configuration or a local manifest override for isolated experimentation. Keep such customization on the developer's machine, outside committed publication files. Do not alter the committed dependency revisions or `Cargo.lock` as an incidental effect of local overrides. Review any lockfile change explicitly before proposing it for the public repository.

The repository ignores local Cargo configuration at `.cargo/config.toml`, along with common editor state, caches, secrets, logs, and temporary files. The ignore rules do not exclude source, tests, or resources.

## GUI verification boundary

GUI development and inspection use an admitted `toolkit-gui-remote-canary` route after checking current capabilities and an approved Designing Makepad target/profile. A connection or evidence query alone does not establish application launch or Remote compatibility. Without public `windows_find_window` and the approved target/profile, record `BLOCKED_ENVIRONMENT` and do not invent window IDs or reuse identifiers from another connection.

Do not fall back to the legacy GUI MCP, direct UIA scripts, private HTTP endpoints, or synthetic screenshots. Historical AI UI scenarios and prior captures are historical records, not current GUI acceptance. Real runtime GUI acceptance for this publication is **NOT RUN**. Static checks, CI, or unit tests do not replace that separate integration evidence.

This repository's README records the publication verification status in `VERIFICATION.md`; that record is maintained with the publication review.

## Text-editing popup pattern and limits

Use a normal root-level widget subtree with a `TextInput` inside a visible panel; do not place a live editor in `OverlayHost` just to obtain z-order. Keep the panel and its input in the normal event and draw tree. The shell owns visibility, placement, outside-click policy, and whether edits are accepted or cancelled. `Modal` is the separate option for an application dialog.

On opening, show the panel and request a redraw. Once the input has a valid drawn area, the shell can request focus with `self.ui.text_input(cx, ids!(popup_input)).set_key_focus(cx)`. The exact point at which a newly visible input has a valid area depends on the shell's draw lifecycle; an immediate focus request before its first draw should not be assumed to work. Forward normal events to the widget tree, use `changed`, `escaped`, or `key_focus_lost` actions as appropriate, and decide explicitly whether focus loss commits, cancels, or does neither. On close, hide the panel and return focus to a still-visible, valid initiating control when appropriate; do not restore a stale area. Avoid treating focus loss caused by an internal click or IME composition as an automatic commit without application-specific handling.

The pinned Makepad source exposes `WidgetRef::set_key_focus` and `TextInputRef` action helpers; this is a proposed integration pattern, not evidence that focus, typing, Escape, outside clicks, selection, IME, or restoration work in a particular shell. Validate those interactions physically through an admitted GUI route. Current GUI acceptance: **NOT RUN**.

## Motion scope

The Style Lab motion probe is a Button with statically declared Animator states. Its None/Reduced/Subtle/Smooth transitions use `duration_normal` policy values (instant/80/160/220 ms); the static declarations must be kept in sync with `MotionTokens`. The hover and popup values shown in the lab are policy tokens, not evidence of applied timings. The probe does not set the built-in hover/focus timings of other controls and `play_style_popup_open` does not animate popup opening. Dynamic construction of Animator values is not assumed safe.
