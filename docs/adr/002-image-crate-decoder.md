# ADR-002: Use `image` Crate for Image Decoding

## Status
Accepted

## Context
pictura needs to decode PNG and JPEG images for display. Requirements:
- Support PNG and JPEG (baseline + progressive)
- Handle metadata (dimensions before full decode for memory budgeting)
- Graceful handling of corrupted/truncated files
- Zero or minimal system dependencies for easy maintenance

## Decision
Use the **`image`** crate (pure Rust) for all image decoding.

## Alternatives Considered

| Decoder | Pros | Cons | Verdict |
|---|---|---|---|
| **`image` crate** | Pure Rust, well-maintained (3k+ stars), supports PNG/JPEG/GIF/BMP/WebP/TIFF, metadata extraction before full decode, no system libs | Slower than native C decoders on very large files (>100MP), higher peak memory on large images | **Chosen** |
| libvips (via `vips` crate) | Extremely fast, streaming pipeline = low memory, handles massive files efficiently | Requires system `libvips` installation + C toolchain, C binding complexity, distro packaging inconsistency, overkill for a viewer | Rejected — maintenance burden |
| ImageMagick (via `magick_rust`) | Battle-tested, supports everything | GPL-style license concerns, system dependency, security vulnerability history, complex API | Rejected — license + security |
| `jpeg-decoder` + `png` crates separately | More granular control | Two crates to maintain, `image` already wraps both, no benefit | Rejected — unnecessary |

## Consequences

- **Positive:** Zero system dependencies. `cargo build` downloads and compiles everything. No "install libjpeg-dev first."
- **Positive:** `image` crate has `image::io::Reader::with_guessed_format()` — auto-detects format from magic bytes. Format detection is a one-liner.
- **Positive:** Dimensions available without full decode: `reader.into_dimensions()` gives width/height before allocating pixel buffer. This enables the memory budget guardrail.
- **Positive:** Adding WebP/GIF/BMP support later is trivial — `image` already supports them.
- **Negative:** Decoding a 20000×20000 PNG allocates ~1.6GB (RGBA u8). Mitigation: pre-check dimensions, refuse or downscale if exceeding memory budget (e.g., 500MB cap).
- **Negative:** Slower than libvips on benchmark — but for a user-facing viewer displaying one image at a time, this is imperceptible. The bottleneck is human perception, not decode speed.
- **Risk:** If users demand support for >100MP images, libvips can be added as an optional backend behind a feature flag without changing the public API. The `viewer` module would talk to a `Decoder` trait, not `image` directly. This ADR does not preclude that future.
