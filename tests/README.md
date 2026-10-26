# Retained UI scenarios

The `.aiui.json` files are historical scenario specifications retained with the
source. They are not executed by the Rust CI workflow and are not evidence of a
current canary run. Some operations belong to the older test runner.

The normal public binary excludes the optional Toolkit Windows UIA bridge.
Use the separate local integration host and an approved, process-bound
`toolkit-gui-remote-canary` profile for future GUI work. Do not silently run a
legacy GUI backend as a fallback. These historical scenarios need an explicit
compatibility review before any current automation run.

Rust unit tests live beside their implementation modules under `src/` and run
with `cargo test --locked`.
