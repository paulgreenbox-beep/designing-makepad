# Publication verification

Checked on 2026-10-27 (Moscow date), Windows x86_64-pc-windows-msvc,
with rustc 1.97.1 and Cargo 1.97.1. The checks used the committed public
Makepad Git revision and lockfile, without Toolkit or a local Makepad override.

| Command | Result |
| --- | --- |
| `cargo check --locked --all-targets -j1` | Passed |
| `cargo test --locked -j1` | Passed: 38 library tests; binary/doc-test targets had no tests |
| `cargo build --locked -j1` | Passed |

The Makepad checkout was verified at
`105158e08c89d47f57af4621420ed68eb3a147ce` with no tracked source changes.
Cargo reports duplicate upstream package warnings for vendored bitflags and
cfg-if; the showcase also has an unused WINDOW_TITLE warning when UIA is off.
These warnings did not prevent checking, testing or linking the executable.

All 55 SVG resources parse and referenced paths exist. The 17 Signal
compatibility files in this source release are documented Lucide copies;
original drawings of unrecorded provenance are excluded and retained locally.
No captured screenshots, user settings, executable bundle or font bundle is
included in the source publication.

The optional local Toolkit host and live GUI interaction were NOT RUN for this
publication. Source models, Rust tests, compilation and legacy `.aiui.json`
scenarios do not establish physical input, popup/focus, DPI or GPU acceptance.
Linux and macOS builds were not tested. Fresh-clone and GitHub Actions results
are recorded separately against the published commit after the initial push.
