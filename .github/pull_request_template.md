## Description

<!-- What does this PR do? Why? -->

## Agent Review Checklist

Before requesting review, confirm each applicable gate:

- [ ] 🚀 **SHIP**: CI pipeline is configured and running on this branch
- [ ] 🔒 **WARDEN**: No secrets in code. `cargo audit` passes. Dependencies are clean.
- [ ] ✅ **VERIFY**: `cargo build --release` passes. `cargo test` passes. Edge cases handled.
- [ ] 📖 **DOC**: `cargo fmt --check` passes. `cargo clippy -- -D warnings` passes. Code is self-documenting.
- [ ] 📋 **AUDITOR**: `cargo deny check` passes. No license violations.
- [ ] 🧠 **ARCHITECT**: ADR exists if this is a new service/module/dependency. No circular deps.
- [ ] 🛡️ **GUARD**: Timeouts, retries, graceful degradation handled (if applicable).
- [ ] ⚡ **PERF**: No N+1 queries. Pagination enforced. Load test considered (if applicable).
- [ ] 👁️ **EYE**: Logging + metrics present for new services/endpoints.

## Risk Acceptance (if applicable)

<!-- If any gate is being overridden, paste the risk acceptance record here. -->

## Related

<!-- Link issues, ADRs, related PRs -->
