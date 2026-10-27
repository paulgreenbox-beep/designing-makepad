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

## Stabilization branch, 2026-10-27

Base commit `396865d7a6b20db85186da7becb1674e6432b9af`; changes in
`fix/ui-stabilization` were checked on Windows x86_64 with Rust/Cargo 1.97.1.
These are working-tree results, separate from the previously published commit.

| Check | Result |
| --- | --- |
| `python scripts/validate_resources.py` | Passed with bundled Python: 55 SVG files and 55 literal references |
| `cargo check --all-targets --offline --locked -j 1` | Passed |
| `cargo test --offline --locked -j 1` | Passed: 42 tests |
| `cargo build --offline --locked -j 1` | Passed |
| Fresh public base clone check/test/build | Passed with local Cargo cache |
| Physical GUI / focus / visual and motion acceptance | **NOT RUN** |

Current `toolkit-gui-remote-canary` answers an evidence request but does not
expose `windows_find_window` in this connection and has no admitted owned target.
GUI acceptance is **BLOCKED_ENVIRONMENT**. Motion duration changes apply to the
existing static button probe only; the values displayed for other controls and
popup openings are tokens, not established runtime behavior. TextInput popup
focus and keyboard behavior remain a documented pattern awaiting live proof.
See `reports/FIX_REPORT.md` and local ignored stabilization evidence.