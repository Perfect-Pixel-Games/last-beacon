# Performance Profiling

Last Beacon ships two complementary tools for diagnosing performance problems:
a live in-game overlay for quick checks, and a Tracy capture for finding
exactly which system is expensive.

## Quick check: the `stat.perf` overlay

Foundation's runtime always includes a performance-stat overlay, toggled from
the in-game console:

```text
stat.perf
```

It shows FPS, frame time, CPU process time, CPU/GPU render time, memory, and
entity count, each with a current value and a running 1-second average, plus a
frame-time history strip so short stutters are visible even when the average
looks fine. Use `stat.perf.dump` to write a one-line snapshot of every stat to
the log, useful for pasting into a bug report without a screenshot.

This overlay is enough to tell *whether* there's a problem and roughly *where*
(CPU game logic vs. CPU render vs. GPU render). It cannot tell you *which
system* inside "CPU Process" is slow — that's what Tracy is for.

## Deep dive: Tracy

[Tracy](https://github.com/wolfpld/tracy) is a free real-time frame profiler.
Bevy has first-class support for it: every system in every schedule
automatically gets a timed span with no extra code required, once the feature
is enabled.

### 1. Get the Tracy profiler app

Tracy is a separate desktop application, not a Rust crate, so it is not
vendored in this repository. Download a prebuilt `tracy-profiler` (or
`tracy.exe` on Windows) from the
[Tracy releases page](https://github.com/wolfpld/tracy/releases) that matches
the Tracy protocol version Bevy 0.19 depends on. Keep it running; it listens
for a connection from the game.

### 2. Build and run the game with the `profiling` feature

```cmd
cargo run --manifest-path game/Cargo.toml --features profiling
```

`profiling` is opt-in (not part of `default-features`) because the Tracy
tracing layer has real overhead and is only useful with a Tracy client
attached. It enables Bevy's `trace_tracy` feature, which pulls in `trace` and
`debug` so span names are meaningful.

### 3. Capture and read a session

With the Tracy app open and the game running, it should connect automatically
(look for the game process in Tracy's connection list if not). Record a few
seconds covering the slow behavior, then stop. The flamegraph shows every
system's per-frame cost; sort by self time to find the actual bottleneck
rather than guessing from aggregate frame time.

Cross-reference what Tracy shows with the `stat.perf` overlay's rows: if
Tracy's game-logic systems are cheap but `CPU Render`/`GPU Render` in the
overlay are high, the bottleneck is in rendering, not gameplay code, and a
GPU frame capture tool (RenderDoc for Vulkan, PIX for DX12) would be the next
step — not currently wired into this project, but the overlay's per-pass
breakdown at least narrows which pass to investigate first.

## Common cause: an unoptimized dev build

Before profiling, make sure you're not measuring an unoptimized build. Both
`game/Cargo.toml` and `engine/Cargo.toml` set `[profile.dev] opt-level = 1`
and `[profile.dev.package."*"] opt-level = 3`, so a plain `cargo run` already
compiles dependencies (Bevy, glam, wgpu, ...) with optimizations — leaving
`opt-level = 0` for dependencies, even in a dev build, can cost several times
the frame budget on its own and will dominate any profiling session until
fixed.
