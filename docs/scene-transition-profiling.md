# Scene Transition Profiling

Use this workflow when a UI scene transition stalls or shows a black frame before the new scene appears.

## Quick capture

```cmd
scripts\profile-scene.cmd --scene last-beacon/main_menu
```

The script runs the game with Bevy's `bevy/trace_chrome` and `bevy/debug` features using the optimized `foundation-test` profile. It also enables slow-step logs by default:

- `FOUNDATION_BSN_PROFILE_MS=1` logs slow Foundation `.bsn` resolve/apply steps.
- `LAST_BEACON_WIDGET_PROFILE_MS=1` logs slow nested Last Beacon widget resolve/apply steps.

Raise either threshold to reduce log noise, for example:

```cmd
set FOUNDATION_BSN_PROFILE_MS=10
set LAST_BEACON_WIDGET_PROFILE_MS=10
scripts\profile-scene.cmd --scene last-beacon/ui_playground
```

## Reading the results

Open the Chrome trace JSON produced by Bevy in <https://ui.perfetto.dev/> or Chrome's tracing viewer. The added spans to look for are:

- `foundation_bsn_instance`
- `foundation_bsn_apply`
- `last_beacon_bsn_widget`
- `last_beacon_bsn_widget_apply`

If the visible stall lines up with `foundation_bsn_apply`, the bottleneck is the synchronous `ScenePatch::apply` path that builds the scene's ECS hierarchy on the main thread. If widget spans are small but the Foundation apply span is large, nested widgets are not the primary cause.

## Current finding

The previous investigation isolated the large transition spike to Foundation's temporary reflection-based `.bsn` bridge applying a complete scene patch synchronously. Readiness gating and font ordering fixed visible pop-in/font swaps, but they do not reduce the main-thread cost of constructing a large BSN scene in one frame.
