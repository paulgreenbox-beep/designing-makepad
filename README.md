# Designing Makepad

A standalone Makepad UI lab with reusable Rust models and widgets for desktop editor shells. The project is independent of GLYF and does not include its document model, persistence, canvas, or application commands.

## What is implemented

- **Tabs:** `TabRegistry<T>` provides stable `TabId<T>` identity, ordering, active selection, dirty and closable state, and targeted operations. `DocumentTabStrip` renders application-provided frames and emits generic select, close, and new-tab actions.
- **Overlays:** `OverlayState` and `OverlayHost` provide a root-level, mouse-first transient popup host with outside-click and Escape dismissal. Menu models, cascaded menu state, and submenu placement helpers support the showcase. This is not a text-dialog or text-editing abstraction; keep live text inputs in the normal widget tree and use Makepad `Modal` for application dialogs.
- **Icons and signals:** `IconName`, `IconButtonSpec`, and Signal icon models describe the curated 38 Lucide SVG assets and the playground's Signal icon set. Applications supply their own command meanings and accessibility IDs.
- **Theme and style:** `theme::TOKENS` supplies a shared token vocabulary mapped onto Makepad values. `StyleProfile` is a reusable style model; the Style Lab showcase is showcase wiring, not a public style widget.
- **Semantic snapshots:** `semantics` provides shared semantic snapshots and stable IDs for integrations. The Windows UIA bridge source is optional local integration code and is not part of the default public application build.

The showcase wires these pieces into a native Makepad window. It is an example application, not a claim that every pattern is a finished product widget.

## Requirements and platform status

Ordinary library and GUI builds use the pinned Makepad Git dependency. Windows is the primary checked platform. The publication verification record is maintained in [`VERIFICATION.md`](VERIFICATION.md). Linux and macOS have not been verified. Runtime GUI acceptance for this publication is **NOT RUN**.

## Use

The public manifest pins Makepad to an immutable revision. Toolkit is not a dependency of ordinary public builds and is not published by this project; it is future external tooling. Developers with a known Toolkit checkout can use the separate local integration template described in [`docs/public-development.md`](docs/public-development.md).

On Windows, install the Rust MSVC toolchain, Visual Studio C++ Build Tools, the Windows SDK and Git. Then run from the checkout:

```powershell
cargo check --locked --all-targets
cargo test --locked
cargo build --locked
cargo run --locked
```

To consume the library from another Cargo project, use release tag `v0.1.0`, or pin a reviewed immutable repository commit:

```toml
[dependencies]
designing-makepad = { git = "https://github.com/paulgreenbox-beep/designing-makepad", tag = "v0.1.0" }
```

Applications can pin a specific commit with `rev = "<commit-sha>"` instead. This repository's MIT license applies to its own code. Bundled icons retain their upstream license terms in [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md); Makepad retains its upstream terms.

## Development

See [`docs/public-development.md`](docs/public-development.md) for dependency pinning, local customization, and GUI verification boundaries. The existing [GLYF integration guide](docs/glyf-integration.md) is a proposal for gradual adoption, not a claim that GLYF has integrated these components.
