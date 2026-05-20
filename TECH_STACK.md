# pictura — Full Tech Stack & Project Infrastructure

## 1. The Tool Itself (Runtime)

| Component | Candidate A | Candidate B | Notes |
|---|---|---|---|
| **Language** | Rust | — | Memory safety, no GC, single binary, cross-compile |
| **GUI Framework** | egui (immediate mode) | GTK4 (retained mode) | ADR-001 required. egui = pure Rust, simpler. GTK4 = native, a11y |
| **Image Decoding** | `image` crate (pure Rust) | `libvips` (C binding) | ADR-002 required. image = safe/simple, libvips = fast/large files |
| **Config** | `serde` + `toml` | `serde` + `ron` | TOML is human-friendly, standard in Rust ecosystem |
| **File Walking** | `walkdir` crate | `ignore` crate | `walkdir` is simpler, `ignore` respects .gitignore |
| **Threading** | `rayon` (data parallel) | `std::thread` + channels | rayon for image preloading, channels for async decode |
| **CLI args** | `clap` | — | Standard, derive-based, self-documenting |
| **Logging** | `tracing` | `log` + `env_logger` | `tracing` is modern, async-aware, structured |
| **Error Handling** | `anyhow` + `thiserror` | — | anyhow for app-level, thiserror for library-level |
| **Unicode** | Rust native (UTF-8) | — | Already handled, but must test |

**Decision shortcut if you want speed:** egui + `image` crate + `walkdir` + `toml` + `clap` + `tracing`. Pure Rust, zero system deps, `cargo build` just works.

---

## 2. Development Environment

| Need | Tool | Why |
|---|---|---|
| **Build system** | `cargo` | Rust's native, no alternatives needed |
| **Formatter** | `rustfmt` | Enforce consistent style, zero config needed |
| **Linter** | `clippy` | Catches idiomatic mistakes, `-D warnings` in CI |
| **Documentation** | `rustdoc` | Generates docs from doc comments, `cargo doc --open` |
| **Editor support** | rust-analyzer (LSP) | Any editor: VS Code, Helix, Zed, Neovim |
| **Pre-commit hooks** | `pre-commit` framework | Secrets scan (WARDEN), fmt check, clippy |
| **Git config** | `.gitignore` + `.gitattributes` | Standard Rust template |
| **Dev shell** | `nix-shell` or `docker-compose` | Reproducible dev environment (optional, overkill for solo) |
| **Hot reload** | `cargo watch` | `cargo watch -x run` for dev loop |

---

## 3. Testing Infrastructure

| Layer | Tool | What It Tests |
|---|---|---|
| **Unit tests** | `cargo test` (built-in) | Individual functions, edge cases |
| **Integration tests** | `cargo test` (tests/ dir) | Module interactions |
| **Snapshot tests** | `insta` crate | Rendered output, config parsing |
| **Property-based** | `proptest` crate | Fuzz-like: "for any image dimensions, zoom doesn't crash" |
| **GUI tests** | — tricky in Rust | Use headless Xvfb + screenshot comparison OR test logic separately from UI |
| **Benchmarks** | `criterion` crate | Decode time, slideshow memory, startup with 10k images |
| **Coverage** | `cargo-llvm-cov` or `tarpaulin` | Line/branch coverage, CI gate at 80% |
| **Test fixtures** | `tests/fixtures/` dir | Valid PNG, valid JPEG, truncated, corrupt, 0-byte, huge, Unicode-named |
| **CI test matrix** | GitHub Actions | Test on ubuntu-latest, multiple Rust versions (stable, MSRV) |

---

## 4. CI/CD Pipeline

| Stage | Tool | Gate |
|---|---|---|
| **Trigger** | PR opened / push to main | — |
| **Format** | `cargo fmt --check` | Must pass |
| **Lint** | `cargo clippy -- -D warnings` | Must pass |
| **Build** | `cargo build --release` | Must pass |
| **Test** | `cargo test` | Must pass |
| **Coverage** | `cargo-llvm-cov` | ≥80% on new code |
| **Security audit** | `cargo audit` (RustSec DB) | No critical/high vulns |
| **License check** | `cargo deny` | No GPL-incompatible deps |
| **Benchmark** | `cargo bench` | No regression >10% without approval |
| **Release build** | `cargo build --release` + strip | Optimized binary |
| **Artifact** | Upload binary to GitHub Releases | Tagged release |
| **Platform** | GitHub Actions (free for public) | ubuntu-latest runner |

---

## 5. Packaging & Distribution

| Target | Method | Tool |
|---|---|---|
| **Source** | crates.io | `cargo publish` |
| **Linux (generic)** | AppImage | `linuxdeploy` + `appimagetool` |
| **Arch Linux** | AUR package | PKGBUILD in separate repo |
| **Ubuntu/Debian** | PPA or .deb | `cargo-deb` crate |
| **Fedora** | COPR or .rpm | `cargo-rpm` crate |
| **Flatpak** | Flathub | Flatpak manifest (sandboxed) |
| **Snap** | Snapcraft | snapcraft.yaml |
| **Nix** | nixpkgs | Nix derivation |
| **Homebrew** (macOS) | Homebrew core | Formula — stretch goal |
| **Windows** | MSI / winget | Cross-compile — stretch goal |

**MVP distribution:** AppImage + crates.io. Covers 90% of Linux users with one file.

---

## 6. Observability & Telemetry

| Need | Tool | Notes |
|---|---|---|
| **Crash reporting** | Write to `~/.pictura/crashes/` | Backtrace + OS info + version. User can review before sending. |
| **Error logging** | `tracing` → file | `~/.pictura/logs/` with rotation |
| **Usage stats** (opt-in) | None initially | GDPR minefield. Add later if users want it. |
| **Performance tracing** | `tracing-chrome` | Generate Chrome trace JSON for debugging perf |
| **Memory profiling** | `dhat` crate | Heap profiling for leak detection in CI |

---

## 7. Project Management & Docs

| Need | Tool | Why |
|---|---|---|
| **Source control** | Git + GitHub/GitLab | Standard |
| **ADR storage** | `docs/adr/` in repo | Markdown, numbered, with status |
| **README** | `README.md` | Screenshot, install, features, keybindings |
| **Contributing** | `CONTRIBUTING.md` | Code style, PR process, commit format |
| **Changelog** | `CHANGELOG.md` or GitHub Releases | Keep a Changelog format |
| **License** | `LICENSE` file | MIT or GPL-3.0 (ADR-003) |
| **Issue templates** | `.github/ISSUE_TEMPLATE/` | Bug report, feature request |
| **PR template** | `.github/pull_request_template.md` | Checklist: tested, formatted, changelog |
| **Roadmap** | GitHub Projects or `ROADMAP.md` | What's next |

---

## 8. Security & Compliance

| Need | Tool | When |
|---|---|---|
| **Secret scanning** | `detect-secrets` or `gitleaks` | Pre-commit + CI |
| **Dependency audit** | `cargo audit` | Every CI run + daily cron |
| **License audit** | `cargo deny` | Every CI run |
| **SBOM generation** | `cargo cyclonedx` or `cargo sbom` | On release |
| **Supply chain** | `cargo vet` (Mozilla) | Audit third-party deps |
| **Static analysis** | `clippy` + `cargo geiger` (unsafe count) | CI gate |
| **Code signing** | GPG sign release artifacts | Before public release |

---

## 9. Community & Contribution

| Need | Tool | Notes |
|---|---|---|
| **Discussion** | GitHub Discussions or Discord | Community support |
| **Donations** | GitHub Sponsors / Ko-fi | If project gains traction |
| **Translations** | `rust-i18n` or Fluent | i18n for UI strings (future) |
| **Code of Conduct** | `CODE_OF_CONDUCT.md` | Contributor Covenant |

---

## Summary: Minimal MVP Stack

If you just want to build it without over-engineering the project infra:

```
pictura/
├── src/                    # Rust source
│   ├── main.rs            # Entry point
│   ├── app.rs             # Main loop, window, fullscreen
│   ├── viewer.rs          # Image display, zoom, pan
│   ├── loader.rs          # File I/O, decoding, format detection
│   ├── config.rs          # TOML config, slideshow timer
│   └── navigation.rs      # File list, next/prev, slideshow logic
├── tests/
│   └── fixtures/          # Test images (valid, corrupt, edge cases)
├── docs/
│   └── adr/               # Architecture Decision Records
├── .github/workflows/
│   └── ci.yml             # GitHub Actions: fmt, clippy, test, audit
├── Cargo.toml
├── README.md
├── LICENSE                # MIT
├── CHANGELOG.md
└── ROADMAP.md
```

**Dependencies (Cargo.toml):**
```toml
[dependencies]
egui = "0.29"              # GUI — if ADR-001 picks egui
eframe = "0.29"            # egui framework runner
image = "0.25"             # PNG, JPEG decoding — if ADR-002 picks image crate
walkdir = "2"              # Recursive directory walking
serde = { version = "1", features = ["derive"] }
toml = "0.8"               # Config file format
clap = { version = "4", features = ["derive"] }  # CLI args
tracing = "0.1"            # Structured logging
tracing-subscriber = "0.3" # Log output
anyhow = "1"               # Error handling
thiserror = "2"            # Custom error types

[dev-dependencies]
insta = "1"                # Snapshot testing
proptest = "1"             # Property-based testing
criterion = "0.5"          # Benchmarking
```

**CI (ci.yml) — complete:**
```yaml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo build --release
      - run: cargo test
      - run: cargo audit
      - run: cargo deny check
```

---

## What You Don't Need (Yet)

| Skipped | Why |
|---|---|
| Database | Read-only file viewer, no state beyond config |
| Web server / API | Desktop app |
| Docker | `cargo build` is self-contained |
| Kubernetes | Come on |
| CDN | No web assets |
| Auth system | No users |
| Analytics | Privacy-first, opt-in only later |
| CI for macOS/Windows | Stretch goal, Linux-first |
