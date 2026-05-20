# ADR-001: Use egui for GUI Framework

## Status
Accepted

## Context
pictura needs a GUI framework for Linux (X11/Wayland) supporting:
- Window management, fullscreen toggle
- Image rendering with zoom/pan
- Keyboard + mouse input
- Minimal dependencies for easy maintenance

## Decision
Use **egui** (immediate mode GUI) via `eframe` runner.

## Alternatives Considered

| Framework | Pros | Cons | Verdict |
|---|---|---|---|
| **egui** | Pure Rust, no system deps, simple API, `cargo build` just works, cross-compiles trivially | Non-native look, no accessibility (yet), immediate mode = redraws every frame | **Chosen** |
| GTK4 | Native look on Linux, full accessibility, retained mode = efficient for static UIs | Requires system GTK4 libs + C toolchain, complex build, version mismatches across distros, harder to cross-compile, gtk4-rs binding lag | Rejected — maintenance burden too high |
| Iced | Pure Rust, retained mode, Elm-inspired architecture | Less mature, fewer widgets, smaller community, breaking changes between versions | Rejected — not stable enough for a tool meant to be maintained long-term |
| Slint | Declarative UI language, native look | Requires Slint DSL + compiler, separate license for non-GPL, smaller ecosystem | Rejected — adds a DSL learning curve + licensing complexity |

## Consequences

- **Positive:** Single `cargo build` on any machine with Rust. No "install these 15 system packages first." Cross-compilation to macOS/Windows is straightforward if we expand later.
- **Positive:** egui's immediate mode simplifies state management — no MVC/MVVM ceremony. State is data, UI is a function of state.
- **Negative:** Non-native look. egui looks like egui. This is acceptable for a utility tool; it's not trying to blend into GNOME or KDE.
- **Negative:** No screen reader support. Accessibility is a known gap in egui. Acceptable for v1.0; tracked as future work.
- **Risk:** egui is actively developed. API may change between minor versions. Mitigation: pin exact version in Cargo.toml, review changelog on upgrades.
