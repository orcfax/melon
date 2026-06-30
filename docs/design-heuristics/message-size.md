# Message Size & Transport

This note considers the effect of message size on propogation times and the
effect of stack on message size.

## Summary

- **Standard Message Target (\< 1.2 KB):** Raw payloads should remain under 1.2
  KB to ensure atomic delivery within a single MTU-bounded packet [^1]
- **Maximum Burst Ceiling (\< 8 KB):** To utilize the initial congestion window
  (initcwnd) without requiring a round-trip acknowledgment, raw payloads must
  not exceed 8 KB [^2]
- **Asynchronous Data Transfer (\> 64 KB):** Payloads exceeding 64 KB should
  utilize a "Claim/Check" architecture. The Gossipsub layer should transmit only
  a 32-byte Content Identifier (CID), with the full CBOR blob retrieved via a
  secondary Request-Response stream.
- **Overhead Allowance:** All bandwidth calculations must include a \~256-byte
  buffer per message for protocol encapsulation.

## Comparating thresholds: 1.2 KB vs. 8 KB Payloads

The following table defines the performance delta between single-packet and
multi-packet transmissions within a gossip mesh.

| Metric              | 1.2 KB Payload (MTU Optimal)                                       | 8 KB Payload (Window Optimal)                                                                     |
| :------------------ | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| **Encapsulation**   | **Atomic.** Message fits within a single 1500-byte Ethernet frame. | **Fragmented.** Requires 6–7 sequential TCP segments.                                             |
| **Error Recovery**  | Low overhead. Single packet loss results in single message loss.   | High overhead. Loss of any segment invalidates the entire 8 KB frame, forcing TCP retransmission. |
| **Propagation**     | **1 RTT.** Theoretical minimum latency for hop-by-hop forwarding.  | **1-2 RTTs.** Subject to receiver ACK frequency and segment reordering delays.                    |
| **Resource Impact** | Negligible buffer pressure.                                        | Measurable. Large concurrent bursts can saturate peer receive buffers.                            |

**Technical Justification for 8 KB:** Modern TCP implementations typically
initialize with an initcwnd of 10 segments (\~14.6 KB) [^2]. An 8 KB payload
plus \~256 bytes of overhead stays within this window, permitting transmission
in a single initial burst.

## Performance Thresholds & Penalties

Thresholds refer to **Raw CBOR Payload size**. The "Stack Tax" is additive
overhead (see below).

| Threshold (Raw)    | Classification | Technical Consequence                                                                                                                   |
| :----------------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| **\~1.2 KB**       | MTU Boundary   | Total size exceeding \~1.4 KB triggers IP fragmentation. Reliability becomes dependent on all constituent fragments arriving [^1].      |
| **16 KB \- 60 KB** | TCP Saturation | Payload exceeds the initial congestion window. Data transmission is throttled by a required Round-Trip Time (RTT) to expand the window. |
| **50 KB+**         | Pipeline Stall | ValidationMode::Strict requirements block the gossip queue. Forwarding is prohibited until full CBOR deserialization and validation.    |
| **\~2 KB**         | Protocol Limit | Default `max\_transmit\_size` [^3].                                                                                                     |

## Stack Tax

The following estimates represent the per-message byte overhead added to the raw
CBOR payload (approx. **150–300 bytes**):

- **Gossipsub Layer:** \~50–100 bytes (Topic strings, sequence numbers, peer
  signatures) [^4].
- **Security Layer (Noise):** \~20–50 bytes (Authentication tags and MACs) [^5].
- **Transport & Multiplexing:** \~52–80 bytes (Yamux frame headers [^6] and
  standard TCP/IP headers [^1]).

## Transport and Security

### TCP vs. QUIC

- **TCP:** Exhibits stream-level Head-of-Line (HoL) blocking. Packet loss on a
  connection stalls all concurrent gossip traffic.
- **QUIC (UDP-based):** Eliminates stream-level HoL blocking. Loss is localized
  to individual message streams, providing superior P99 latency in high-loss
  environments [^7].

### Noise vs. TLS 1.3

- **Noise (IK/XX):** Optimized for P2P. Lower per-packet overhead and faster
  0/1-RTT handshakes [^5].
- **TLS 1.3:** Requires X.509 certificate management. Record headers are
  approximately 5 bytes larger per frame compared to Noise, leading to higher
  cumulative overhead in high-frequency messaging [^8].

[^1]: https://www.rfc-editor.org/rfc/rfc894

[^2]: https://www.cdnplanet.com/blog/tune-tcp-initcwnd-for-optimum-performance/

[^3]:
    https://docs.rs/libp2p-gossipsub/latest/libp2p_gossipsub/struct.ConfigBuilder.html#method.max_transmit_size

[^4]:
    https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/gossipsub-v1.1.md

[^5]: https://noiseprotocol.org/noise.html

[^6]: https://github.com/libp2p/specs/tree/master/yamux

[^7]: https://datatracker.ietf.org/doc/html/rfc9000

[^8]: https://datatracker.ietf.org/doc/html/rfc8446
