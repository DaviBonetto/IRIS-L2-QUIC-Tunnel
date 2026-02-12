<div align="center">

```
  â–ˆâ–ˆâ•—â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ•— â–ˆâ–ˆâ•—â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ•—
  â–ˆâ–ˆâ•‘â–ˆâ–ˆâ•”â•â•â–ˆâ–ˆâ•—â–ˆâ–ˆâ•‘â–ˆâ–ˆâ•”â•â•â•â•â•
  â–ˆâ–ˆâ•‘â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ•”â•â–ˆâ–ˆâ•‘â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ•—
  â–ˆâ–ˆâ•‘â–ˆâ–ˆâ•”â•â•â–ˆâ–ˆâ•—â–ˆâ–ˆâ•‘â•šâ•â•â•â•â–ˆâ–ˆâ•‘
  â–ˆâ–ˆâ•‘â–ˆâ–ˆâ•‘  â–ˆâ–ˆâ•‘â–ˆâ–ˆâ•‘â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ•‘
  â•šâ•â•â•šâ•â•  â•šâ•â•â•šâ•â•â•šâ•â•â•â•â•â•â•
```

### ðŸ›¡ï¸ System 14/300 â€” High-Performance QUIC Tunnel

[![Rust](https://img.shields.io/badge/Rust-1.75+-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![QUIC](https://img.shields.io/badge/Protocol-QUIC-blue?style=flat-square)](https://quicwg.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

**A secure, low-latency tunneling system built on top of Quinn (QUIC implementation).**

</div>

---

## ðŸ“– What is IRIS?

IRIS is a Layer 2 tunneling simulation using **QUIC (Quick UDP Internet Connections)**.
It demonstrates:

- ðŸš€ **Zero-RTT** connection establishment.
- ðŸ”’ **TLS 1.3** integration with self-signed certificates.
- ðŸ“¡ **Multiplexed Streams** over a single UDP connection.

## ðŸš€ Getting Started

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

## ðŸ—ï¸ Architecture

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

## ðŸ“„ License

Distributed under the MIT License. See `LICENSE` for more information.
