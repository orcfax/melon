---
title: "Hardening Round-Robin via Route Throttling and Private IPv6"
author: "@waalge"
date: 2026-04-18
tags:
  - security
  - infrastructure
  - networking
  - ddos-mitigation
---

## **Context**

The network utilizes a deterministic round-robin proposer schedule. This
predictability creates a vulnerability where attackers can identify and DDoS the
upcoming proposer, effectively halting the chain.

Key challenges:

1. **Public Visibility:** Proposers must be reachable to receive statements and
   endorsements, but public exposure invites targeted flooding.
2. **Resource Exhaustion:** Traditional application-layer filtering is
   insufficient if the network interface or kernel is saturated by a volumetric
   DDoS.
3. **Global Latency:** Any mitigation must not introduce significant "hops" or
   processing delays that exceed the 500ms global latency budget.

## **Decision**

We will implement a dual-layer infrastructure defense to secure the predictable
proposer rotation without requiring anonymity protocols like VRFs.

1. **Kernel-Level Dynamic Whitelisting:**
   - We will use nftables to maintain a dynamic "Priority Set" containing the IP
     addresses of the 100 current validators.
   - Traffic from non-whitelisted IPs will be strictly throttled using Linux
     Traffic Control (tc) with a Hierarchical Token Bucket (HTB).
   - Unknown traffic will be relegated to a "Junk" queue with a low bandwidth
     ceiling, ensuring validator-to-validator packets bypass congestion during
     an attack.
2. **Semi-Private IPv6 Channels:**
   - Every node will bind a unique, non-published IPv6 address from its
     allocated prefix for each of its 100 peers.
   - These "Semi-private" addresses will not be broadcast in public discovery
     mechanisms (Kademlia DHT).
   - Peer-to-peer connections will prioritize these direct IPv6 routes, creating
     a "Virtual Backbone" that remains invisible to standard network scanners
     and botnet harvesters.
3. **Inbound Connection Limiting:**
   - We will enforce strict connlimit rules for non-validator IPs to prevent TCP
     state exhaustion, while allowing unlimited concurrent connections for
     verified validator PeerIDs.

## **Dissent, counter, and comments**

- **Operational Overhead:** Managing OS-level firewall rules from within the
  Rust binary requires elevated privileges (CAP_NET_ADMIN) and complicates
  containerized deployments (Docker/Kubernetes).
- **IP Spoofing:** While tc prioritizes IP addresses, sophisticated attackers
  might attempt to spoof validator IPs. However, the requirement for an
  established libp2p noise-encrypted handshake mitigates the impact of spoofed
  packets reaching the application layer.
- **IPv6 Availability:** Some data centers or regional ISPs may have
  inconsistent IPv6 routing, potentially isolating nodes that rely solely on
  private IPv6 channels.

## **Status**

Proposed

## **Consequences**

- **Positive:** Protects the proposer's "Surface Window" without adding the
  complexity of Secret Leader Election (SLE); keeps the verification logic on
  Cardano simple by retaining the deterministic schedule.
- **Negative:** Requires more complex DevOps and "bare metal" or "raw socket"
  access; introduces potential for accidental self-isolation if the whitelist
  synchronization fails.
- **Neutral:** Shifts the security burden from the cryptographer to the system
  administrator.
