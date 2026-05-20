# pictura Post-Mortem: What the Sub-Agents Would Have Caught

## The Reality

We had **4 rounds of fixes** after the initial scaffold. Each round was an issue that a sub-agent would have caught before Alex ever touched the repo.

---

## Round 1: CI/CD Never Actually Ran

**What happened:** I wrote `.github/workflows/ci.yml` but never pushed it to a real repo. When we finally did, it was blocked by a missing `workflow` token scope. Alex had to manually fix permissions.

**Agent that would have caught it:** 🚀 **SHIP**

| Check | Status |
|---|---|
| Deploy pipeline green? | ❌ Never executed |
| Pipeline gates defined? | ⚠️ Defined but not connected to any repo |

**When it would fire:** Immediately on first push. SHIP checks: "is the pipeline running and passing?" If no pipeline exists or it never ran, BLOCK.

**Prevention:** SHIP would have flagged "CI pipeline defined but not activated — no runs detected on main branch" before I declared the project "ready."

---

## Round 2: Missing Bench File Breaks Build

**What happened:** `Cargo.toml` referenced `benches/decode.rs` in `[[bench]]` but the file didn't exist. `cargo build` failed immediately.

**Agent that would have caught it:** ✅ **VERIFY**

| Check | Status |
|---|---|
| Does the build pass? | ❌ `cargo build --release` failed |
| Are all referenced files present? | ❌ `benches/decode.rs` referenced but missing |

**When it would fire:** On any PR or on the CI pipeline's build step. VERIFY runs `cargo build` (or checks CI output). Build failure → BLOCK.

**Prevention:** The CI pipeline has `cargo build --release` as a gate. If the pipeline had been running from the start, it would have caught this. VERIFY would have flagged "build step failed — bench target references missing file."

---

## Round 3: API Deprecation + Type Errors

**What happened:** Three errors and three warnings on first successful build:

| Error/Warning | Root Cause |
|---|---|
| `image::io::Reader` deprecated | image crate 0.25 renamed it to `ImageReader` |
| `Reader::clone()` doesn't exist | ImageReader no longer implements Clone |
| `egui::Key::Question` not found | egui 0.29 changed key enum |
| `SlideshowConfig: Default` missing | Serde `#[serde(default)]` requires `Default` trait |
| `ViewerConfig: Default` missing | Same |
| `GenericImageView` unused import | Leftover from earlier code |
| `MAX_DECODE_BYTES` unused | Constant defined but never referenced |

**Agents that would have caught them:**

| Agent | What it catches |
|---|---|
| ✅ **VERIFY** | Any build failure. CI gate `cargo build` → fail. BLOCK. |
| ⚡ **PERF** / ✅ **VERIFY** | Deprecated API usage. PERF checks dependency compatibility; VERIFY checks that the code uses current APIs. |
| 📖 **DOC** | Unused imports, dead code. DOC's clarity check flags code that doesn't do anything. |

**When they would fire:** VERIFY on build failure (immediate). DOC on dead code scan (can run asynchronously, non-blocking but flags accumulate).

**Prevention:** The CI pipeline's `cargo build` and `cargo clippy -- -D warnings` gates catch all of these before merge. Warnings as errors = no dead code, no deprecated APIs, no missing derives slip through.

---

## Round 4: Root Compromise — No Test Fixtures

**What happened:** The `cargo test` would have failed because `tests/fixtures/valid.png` didn't exist, and the loader tests reference it for the memory budget test.

**Agent that would have caught it:** ✅ **VERIFY**

| Check | Status |
|---|---|
| Do tests pass? | ❌ `cargo test` fails — fixture missing |
| Are test fixtures present? | ❌ Directory exists but empty |

**Prevention:** VERIFY's CI gate `cargo test` catches this. Would have been round 2, not round 4, if pipeline was running.

---

## Round 5: System Crash on Close (NVIDIA Driver)

**What happened:** pictura ran fine but crashed the entire server on app close due to broken NVIDIA kernel module.

**Agent that would have caught it:** 🛡️ **GUARD** + 👁️ **EYE**

| Agent | What it would flag |
|---|---|
| 🛡️ **GUARD** | "No graceful degradation — app close causes system-level crash. Instance should fail gracefully, not take down the host." |
| 👁️ **EYE** | "No crash telemetry. If this happened in production, we'd have no logs to diagnose it." |

**When they would fire:** GUARD during reliability review (post-deploy incident). EYE during observability audit (missing crash handler).

**Prevention for future:** GUARD would recommend a crash handler (`SIGSEGV` handler that writes to `~/.pictura/crashes/` before exit). EYE would require structured logging that captures the shutdown sequence. Neither would prevent the NVIDIA bug, but they'd make diagnosis instant instead of "server crashed, no logs."

---

## Summary: Which Agents Caught What

```
ROUND    ISSUE                              CAUGHT BY           WHEN
─────────────────────────────────────────────────────────────────────────
  1      CI not running on real repo        🚀 SHIP             First push
  2      benches/decode.rs missing          ✅ VERIFY           Build step
  3a     image::io::Reader deprecated       ✅ VERIFY           Build step
  3b     Reader::clone() removed            ✅ VERIFY           Build step
  3c     egui::Key::Question gone           ✅ VERIFY           Build step
  3d     Missing Default derives            ✅ VERIFY           Build step
  3e     Unused imports / dead code         📖 DOC             Clippy scan
  4      Test fixtures missing              ✅ VERIFY           Test step
  5      System crash on close              🛡️ GUARD + 👁️ EYE  Post-incident
```

---

## The Meta Lesson

**VERIFY caught 6 of 9 issues.** This is not a coincidence — the build pipeline is the densest gate. If `cargo build` and `cargo test` run on every push, 6 of our 9 problems never reach a human.

**The CI pipeline IS the sub-agent system for compilation.** We defined it. We just didn't run it. Once pushed, it would have caught everything in rounds 2–4 automatically.

**SHIP caught the meta-issue:** the pipeline itself wasn't connected. This is the one thing no other agent could catch, because every other agent depends on SHIP's pipeline running.

---

## What This Means for the Agent Model

The model **works** — it predicted exactly the categories of issues we hit. The failure was not in the agent definitions but in the execution: the pipeline wasn't live.

For real projects, the agents should be **enforceable gates**, not just theoretical checklists:

| Theoretical | Enforceable |
|---|---|
| "VERIFY checks build passes" | CI runs `cargo build` → red X on PR |
| "SHIP checks pipeline is configured" | Repo requires passing CI before merge |
| "DOC checks dead code" | `cargo clippy -- -D warnings` in CI |
| "WARDEN checks secrets" | Pre-commit hook + CI secret scan |

The agents don't just produce reports — they **block merges** with actual CI checks that the developer can't bypass without an explicit override.
