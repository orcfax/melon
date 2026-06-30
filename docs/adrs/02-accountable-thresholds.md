---
title: "Accountable Thresholds"
author: "@waalge"
date: 2026-04-19
tags:
  - cryptography
  - consensus
---

## **Context**

Initial attempts to implement a round-robin proposer selection with a liveness
filter failed because nodes relied on subjective local views. Without shared
state, nodes diverged on the "Live Set," leading to consensus stalls (the
"Entropy Spiral").

Furthermore, Static TSS with a Trusted Dealer is brittle; permanent loss of
\>1/3 of validators causes unrecoverable halts and lacks the per-block
visibility required to synchronize liveness across high-latency links (e.g.,
Patagonia to Siberia).

## **Decision**

We will switch to **Accountable Threshold Signatures** via **Dynamic VK
Aggregation** to provide an objective truth for liveness via the Signer Bitmask.

1. **Registration:** We introduce a new Item for bootstrapping:

```rust
   Registration {
       keys: Keys {
           node_id: ed25519::VerificationKey,
           bls: bls::VerificationKey,
       },
       proof: {
           ed25519: ed25519::Signature,
           provenance: bls::Signature
       }
   }
```

Registration requires Proof-of-Possession (PoP) to prevent rogue key attacks.  
2. **On-the-Fly Aggregation:** Nodes recompute the Group VK for every Quorum
Certificate (QC) by summing Public Keys (PKs) mapped to the included **Signer
Bitmask**.  
3. **Objective Liveness:** The QC bitmask in finalized blocks is the
authoritative source for proposer rotation. This bitmask solves the
desynchronization issues encountered in the initial round-robin
implementation.  
4. **L1 Subtraction Optimization:** Cardano stores a "Base Group VK" (Sum of all
100 PKs). Verification uses:
`vk_effective = vk_base - \sum _{ missing } vk_i` 5. **Canonical View:** The
system will maintain a canonical view of the validator registry including these
node keys. _Note: Full details of the Registry View will be established in a
separate ADR._

## **Dissent, counter, and comments**

- **CPU:** Point addition/subtraction costs \~15μs for 100 nodes; negligible.
- **L1 Cost:** Subtraction is efficient in Plutus but requires the Base VK to
  stay in sync with the L2 registry.
- **Registry Synchronization:** Requires bit-perfect agreement on the validator
  list to ensure aggregated VKs match.

## **Status**

Proposed

## **Consequences**

- **Positive:** Solves round-robin desynchronization; enables automatic recovery
  from churn; eliminates trusted setup.
- **Negative:** Increased complexity in seq logic; requires L1 updates for
  registry changes.
- **Neutral:** Usage on-chain scales in number of missing signatures, rather
  than total.
