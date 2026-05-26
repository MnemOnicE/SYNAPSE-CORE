# Project SYNAPSE: True Membrane Architecture

## Abstract

Project SYNAPSE is a zero-latency, congestion-proof cybernetic message bus explicitly designed for mobile architectures within the Termux environment. It acts as the critical neurological link between raw hardware streams and high-level agentic logic, ensuring fault-tolerant routing on constrained Android devices.

## The Architecture (Ingress/Egress Split)

The architecture is built on a strict boundary between performance-critical ingestion and stateful logic:

```mermaid
flowchart TD
    %% Styling Definitions
    classDef hardware fill:#e2e8f0,stroke:#64748b,stroke-width:2px,color:#0f172a
    classDef rust fill:#fef08a,stroke:#ca8a04,stroke-width:2px,color:#854d0e
    classDef uds fill:#fecdd3,stroke:#e11d48,stroke-width:2px,color:#9f1239,stroke-dasharray: 5 5
    classDef broker fill:#bfdbfe,stroke:#2563eb,stroke-width:2px,color:#1e3a8a
    classDef python fill:#bbf7d0,stroke:#16a34a,stroke-width:2px,color:#14532d
    classDef storage fill:#fed7aa,stroke:#ea580c,stroke-width:2px,color:#9a3412
    classDef logic fill:#e9d5ff,stroke:#9333ea,stroke-width:2px,color:#581c87

    %% External Sources
    subgraph External["External Hardware / Publishers"]
        direction LR
        S1[Serial Data] --> S3
        S2[Sensors] --> S3
        S3[Raw Byte Stream]:::hardware
    end

    %% The Membrane Boundary
    UDS{{"Unix Domain Socket<br>(~/.synapse/ingress.sock)"}}:::uds
    S3 --> |Zero-Hop| UDS

    %% Rust Ingress Engine
    subgraph RustEngine["Rust Gatekeeper (Ingress Engine)"]
        direction TB
        Ingress[Data Ingestion<br>Zero-copy Parsing]:::rust
        UDS --> Ingress

        Eval{"Horizon Evaluation<br>(dQ/dt Thresholds)"}:::rust
        Ingress --> Eval

        WAL[("Memory-Mapped<br>Ring Buffer (WAL)")]:::storage
        Ingress --> |Raw Payload| WAL

        Telemetry[UDP Telemetry<br>Heartbeat]:::rust
        Eval -.- Telemetry

        Eval --> |Viable Signal| Proj[Holographic Projection]:::rust
    end

    %% The Broker
    subgraph MessageBroker["Local Broker"]
        Redis[(Redis)]:::broker
        Proj --> |HolographicHeader| Redis
    end

    %% Python Egress Loop
    subgraph PythonAgent["Python Agentic Loop (SYNAPSE-CORE)"]
        direction TB
        Egress[Stateful Logic Consumer]:::python
        Redis --> |Consume Headers| Egress

        Router{"Logic Router"}:::logic
        Egress --> Router

        Sub1[Desk LED Flasher]:::python
        Sub2[Termux Haptic Engine]:::python

        Router --> |Action Route| Sub1
        Router --> |Action Route| Sub2
    end
```


1. **Rust Gatekeeper (Ingress):** Hardware streams feed directly into the Rust engine via Unix Domain Sockets (UDS), specifically located at `~/.synapse/ingress.sock`. The Rust layer performs critical $dQ/dt$ Schwarzschild threshold evaluation to determine signal viability without dropping frames.
2. **Holographic Projection:** Once evaluated, the Rust engine projects a lightweight `HolographicHeader` via a local Redis broker.
3. **Python Agent (Egress):** The `SYNAPSE-CORE` Python agentic loop consumes these HolographicHeaders to execute high-level orchestration, ensuring that heavy computational logic never blocks raw physical data ingestion.

## Deployment

To launch the system natively within an Android Termux environment, use the following explicit bootstrap sequence.

### 1. Compile and Start the Rust Engine

```bash
cd synapse
cargo build --release
./target/release/synapse
```

### 2. Initialize the Python Agentic Loop

Open a new Termux session or run alongside the backgrounded Rust engine:

```bash
cd SYNAPSE-CORE
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python src/main.py
```
