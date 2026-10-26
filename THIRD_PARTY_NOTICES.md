# Third-party notices

## Lucide icons

The curated SVG subset under `resources/icons/` is sourced from Lucide. Lucide is distributed under the ISC License. The upstream license text, including its notice for icons derived from Feather, is preserved in [`resources/icons/LICENSE-LUCIDE.txt`](resources/icons/LICENSE-LUCIDE.txt).

## Feather-derived icons

The Lucide upstream notice identifies the listed Feather-derived icons and includes the MIT License, Copyright (c) 2013-present Cole Bemis. That notice is reproduced in the retained Lucide license file referenced above.

## Makepad

The ordinary public application depends on Makepad from its public Git repository, pinned to the revision in `Cargo.toml`. Its license remains the upstream project's; this repository's MIT license does not relicense Makepad.

## Rust Makepad Toolkit

Rust Makepad Toolkit is future external tooling and is not a dependency of ordinary public builds or published by this project. Developers may use it in a separate local integration host when they have a known Toolkit checkout. Any such tooling retains its own upstream license terms; this repository's MIT license does not relicense it.

The public Signal compatibility SVGs are copies of the curated Lucide assets; see `resources/icons/UPSTREAM.md` for the exact mapping. Previous unverified-provenance drawings are excluded and retained only locally.

## Dependency metadata review

The 154 external packages in this source release's lockfile were checked by cached package declarations and exact-version registry metadata. 152 have explicit permissive declarations (including Apache-only packages and Unicode-3.0 additional terms). Two Makepad crates, `makepad-regex` and `makepad-latex-math`, omit the Cargo license field; they are distributed within the pinned upstream repository under its root MIT license, with no separate package override found. Their files and fonts are not copied into this repository. This declaration review is not a binary/font redistribution audit.
