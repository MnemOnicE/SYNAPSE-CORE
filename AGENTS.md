# Project Synapse Guidelines

- All Python scripts must include a plain-text YAML frontmatter block at the top mapping its ontology (Tags, Parents, Children).
- For tests, follow standard pytest conventions and mock the Redis client.

## Coding Squad Architectural Contract

The following laws are strictly enforced for all future Coding Squad interactions regarding the "True Membrane Architecture" running within the Termux mobile environment:

- **The Membrane Law**: Agents must never route raw hardware ingress directly through Redis. All raw physical data MUST pass through the Rust UDS socket first.
- **The Concurrency Law**: Agents must never implement mmap or standard shared memory for cross-language telemetry due to Android/GIL constraints. Enforce the local UDP heartbeat with Exponential Moving Average (EMA) recovery.
- **The Zero-Copy Law**: Any modifications to the Rust ingress router must strictly utilize the `bytes` crate and `serde_json::value::RawValue`. DOM heap allocations for routing are strictly prohibited.
- **The Permissions Law**: UDS sockets must be locked to 0600 user-space permissions to prevent Android SELinux violations.
