<div align="center">

```
  ██╗██████╗ ██╗███████╗
  ██║██╔══██╗██║██╔════╝
  ██║██████╔╝██║███████╗
  ██║██╔══██╗██║╚════██║
  ██║██║  ██║██║███████║
  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝
```

### System 14/300 - High-Performance QUIC Tunnel

[![Rust](https://img.shields.io/badge/Rust-1.75+-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![QUIC](https://img.shields.io/badge/Protocol-QUIC-blue?style=flat-square)](https://quicwg.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

**A secure, low-latency tunneling system built on top of Quinn (QUIC implementation).**

</div>

---

## What is IRIS?

IRIS is a Layer 2 tunneling simulation using **QUIC (Quick UDP Internet Connections)**.
It demonstrates:

- **Zero-RTT** connection establishment.
- **TLS 1.3** integration with self-signed certificates.
- **Multiplexed Streams** over a single UDP connection.

## Getting Started

### Prerequisites

- **Rust**: `v1.75` or later.
- **Cargo**: Included with Rust.

### Installation

```bash
git clone https://github.com/DaviBonetto/IRIS-L2-QUIC-Tunnel.git
cd IRIS-L2-QUIC-Tunnel
```

### Running the Simulation

We have included a verified simulation script to test the tunnel:

**PowerShell (Windows)**

```powershell
./simulate_v2.ps1
```

**Bash (Linux/macOS)**

```bash
# You may need to grant execution permissions
chmod +x simulate_tunnel.sh
./simulate_tunnel.sh
```

## Architecture

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    C->>S: QUIC Handshake (TLS 1.3)
    S-->>C: Connection Established (0-RTT)
    C->>S: Open Bi-Directional Stream
    C->>S: Send Payload "Hello from Titan"
    S-->>C: Ack
    C->>S: Close Stream
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
