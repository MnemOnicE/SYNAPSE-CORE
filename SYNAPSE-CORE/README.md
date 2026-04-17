# Project SYNAPSE 🧠

> "A standalone cybernetic message bus and highly modular distributed architecture."

**Project SYNAPSE** elevates a simple publish/subscribe loop into a professional, distributed nervous system capable of running across servers, laptops, microcontrollers, and mobile devices (via Termux).

By decoupling *Sensors* (Publishers) from *Motors* (Subscribers) using a strictly authenticated, robust JSON schema over a Redis message broker, adding a new hardware module or software integration is as simple as dropping a single script into the `nodes/` folder.

---

## 🏛️ Architecture

The codebase is organized entirely by the role of the node:

- `core/`: The nervous system's spine. Handles the Redis connection, standardized error logging, and JSON payload building/parsing.
- `nodes/publishers/`: **Sensory Nodes (The Eyes/Ears)**
  - Scripts that observe their environment (e.g., tailing logs, reading serial ports) and SHOUT events to the bus. They contain zero logic about *what* should happen next.
- `nodes/subscribers/`: **Motor Nodes (The Hands/Voice)**
  - Scripts that LISTEN for specific intents and trigger physical or digital reactions (e.g., flashing LEDs, API webhooks, phone vibrations). They contain zero logic about *why* the intent was sent.
- `nodes/hybrids/`: **Logic Nodes (The Brain)**
  - Scripts that do BOTH. They listen for sensory events, process conditional logic or AI reasoning, and publish motor commands back to the bus.

## 📦 The Synapse Schema

All nodes **must** speak the exact same language. The bus enforces a strict JSON schema. If a module wants to broadcast, it uses `PayloadManager.build_payload()` to create this envelope:

```json
{
  "timestamp": 1713387343,
  "source_node": "ubuntu_auth_watchdog",
  "event_category": "security",
  "action_intent": "alert",
  "data": {
    "ip_address": "192.168.1.45",
    "severity": 8
  }
}
```

### Dealing with Raw Bytes
JSON does not natively support raw byte arrays (e.g., raw serial sensor data). To prevent serialization crashes, use the included Base64 helpers in `core/utils.py`:
- `PayloadManager.encode_bytes(b'\x01\x02')` -> `"AQI="`
- `PayloadManager.decode_bytes("AQI=") -> b'\x01\x02'`

## 🚀 Quickstart & Deployment

1. **Configure Environment**
   Update the `.env` file with your secure Redis parameters:
   ```env
   REDIS_HOST=localhost
   REDIS_PORT=6379
   REDIS_PASSWORD=supersecret_synapse_password
   ```

2. **Launch the Nervous System**
   The deployment script will automatically create a Python virtual environment, install dependencies, spin up the Redis broker via Docker Compose, and launch all nodes in the background.
   ```bash
   cd deployment/
   ./launch_nodes.sh
   ```

3. **Bridging to Mobile (Termux)**
   To treat a mobile phone as a node, bridge the Redis port over an SSH tunnel or `socat` from your Ubuntu server to the device, and run a subscriber script natively in Termux pointing to `localhost:6379`.

---
*Generated via Axion Core logic. Architecture: Distributed.*
