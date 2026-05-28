🧪 [Testing] Add TTL cull logic tests for HorizonBus

🎯 **What:** The `HorizonBus::should_unfold` TTL cull logic previously had no automated unit tests, leading to a testing gap for an important feature that culls stale or future-dated holographic payloads.

📊 **Coverage:** The new tests cover:
- `test_should_unfold_fresh_frame`: Ensuring fresh frames (`age < max_age_ms` and `age > 0`) are successfully verified for unfolding.
- `test_should_unfold_stale_frame`: Ensuring stale frames (`age > max_age_ms`) return false and are dropped as Hawking radiation.
- `test_should_unfold_future_frame`: Ensuring future-dated frames (`age < 0`) correctly fail validation.
- `test_should_unfold_edge_cases`: Testing timestamps right near the 0 age and the `max_age_ms` threshold.

✨ **Result:** Test coverage for `synapse/src/horizon/bus.rs` is significantly improved. A deterministic `std::sync::OnceLock` singleton `HorizonBus` wrapper was implemented to cleanly provide state without binding errors on multiple `AgentTelemetry` instantiation instances across test runners.
