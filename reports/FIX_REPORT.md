# Stabilization fix report

Base: `396865d7a6b20db85186da7becb1674e6432b9af` on `fix/ui-stabilization`.
Windows x86_64-pc-windows-msvc. GUI acceptance remains separate from source tests.

| ID | Files / functions | Priority | Finding and change | Verification / remaining risk |
| --- | --- | --- | --- | --- |
| B1 | showcase, Toolkit canary | P1 | Physical GUI smoke test requires an admitted owned target and current canary discovery/input tools. | **BLOCKED_ENVIRONMENT**: current canary catalog has no `windows_find_window`; no GUI steps or screenshots executed. |
| B2 | `src/menu.rs:409,425,440`; `src/overlay/overlay_state.rs:106` | P2 | Add regression coverage for cascade replacement, close and repeated dismissal; preserve OverlayHost implementation. | 42 tests passed, including menu and overlay state. Physical routing, pointer corridor, resize and DPI remain **NOT RUN**, not fixed or reproduced. |
| B3 | `docs/public-development.md:27` (Text-editing popup pattern) | P2 | Document normal widget-tree TextInput, redraw before requesting focus, cancellation and focus restoration boundaries. | Proposed pattern only: physical typing, IME, Enter/Escape, outside click and compatibility with GLYF are **NOT RUN**. No generic TextPopupHost is claimed. |
| B4 | `src/theme.rs:381`; `src/style_lab.rs:111,324` | P2 | Align the existing static motion probe timings with motion tokens and label displayed values as policy tokens. | Static source change only; runtime transitions and standard-control, popup, tab and focus motion are **NOT RUN**. The probe does not implement animation on every control. |
| B5 | Style Lab and showcase | P2 | Existing presets retained without speculative visual changes. | Visual presets, 125% scaling, font sizes and density **NOT RUN**. Enum presence is not visual proof. |
| C1 | `scripts/validate_resources.py:9`; `.github/workflows/ci.yml` | P2 | Add XML and literal SVG resource-reference validation to Windows CI. | Local validator passed for 55 SVGs and 55 references using bundled Python; candidate Windows CI passed: https://github.com/paulgreenbox-beep/designing-makepad/actions/runs/40557306444. |

Fresh public `main` was checked, tested and built separately at the base commit using the local Cargo cache. Physical GUI acceptance remains blocked.
Tested candidate commit: 6d4b76566ac8068c963fbabac3f963d287d7ccdf; the local tree-matched commit is 04e4f63426b6fb8edd2fd610e9934aee844cfcdc.
