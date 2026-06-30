---
title: "Slots and timing"
author: "@waalge"
date: 2026-04-19
tags:
  - consensus
---

## Context

Melon consensus is a form of pipelined BFT. The chain evolves by adding blocks:
each block is proposed, then endorsed to produce a Quorum Certificate (QC).
Block `n + 1` includes the QC for block `n`.

We use accountable signatures: the QC bitmask doubles as a proof of liveness.
The live set for slot `n` is derived from the QC in block `n`.

Assumptions:

- delivery probabilities (one honest node to another): p50 < 400ms; p95 < 700ms;
  p99: < 1s.
- Clock drift < 100ms.

## Decision

Block production is slot-based. The proposer is selected by round robin over the
live set. Slot length is a fixed network parameter: **4000ms**. Every node
awaits the full slot - well-configured nodes are not penalised by geography.
Blocks are lightweight: a header and a vec of blake3 hashes.

The proposer of block `n` aggregates the QC for block `n-1`, collecting
endorsements throughout slot `n` and cutting off at `3900ms`. Since all
endorsements are broadcast, every node can independently reconstruct the QC; the
proposer is not privileged. The canonical QC and live set are derived from chain
state - always deterministic, never ambiguous.

### Slot structure

```
0 - 100ms       Proposer broadcasts proposal for block n
100 - 1500ms    Peers receive proposal and endorse if received before cutoff
                Endorsements broadcast immediately; overlap with proposal delivery
1500ms          Proposal receipt cutoff - late proposals are not endorsed
~2500ms         p99 for last valid endorsements to have propagated
2500 - 3900ms   Network quiet: no new endorsements expected
3900ms          Proposer assembles block n with QC for block n-1
4000ms          Slot boundary. Next slot begins.
```

The 1500ms cutoff accounts for 100ms broadcast window, p99 delivery, and 100ms
clock skew. It prevents a late proposal triggering a last-minute endorsement
storm. The quiet period is a natural health signal: persistent endorsement
traffic after ~2500ms indicates a degraded network or an ignored late proposal.

### Fork prevention

If a peer can construct a valid QC for height `h`, it must not endorse any
competing proposal at height `h` that omits it - doing so would ratify a fork.
This is locally enforceable; the proposer's intent is irrelevant.

A valid QC requires 2/3+ of the live set to have broadcast endorsements. Since
endorsements propagate to all, if one node can construct a QC then with
probability ~`1 - (0.01)^(2N/3)` so can the supermajority. A competing proposal
is self-defeating: it will not reach threshold.

Caveat: this assumes independent delivery. A regional partition could prevent a
subset of nodes from receiving endorsements while still receiving a competing
proposal. Forks remain unlikely but are most plausible under correlated network
failures.

### Proposal failure

If no valid proposal is received, or the proposal fails to reach 2/3 threshold,
the next proposer builds at the same height `h` reusing the last agreed QC. A
persistently absent node falls out of the live set within 5 slots.

### Genesis

Block 0 is produced by node 0. TBC.

## Decent, counter, and comments

No within-slot fallback: a dead slot is unrecoverable. This trades liveness for
simplicity; revisit if empty slots become a throughput concern.

**Open concerns:**

- **Cutoff observability.** A misconfigured node silently fails to endorse with
  no in-protocol signal. Participation rate per node should be monitored
  operationally.
- **QC reachability.** The next proposer should monitor their endorsement count
  against threshold in real time; if short of 2/3 at 3900ms the slot is lost
  with no recourse.
- **Silent nodes.** A node seeing no proposal is almost certainly witnessing a
  failed slot, not a local fault. It should seek better peers.

## Status

Proposed.

## Consequences

4s slot time gives modest but predictable time-to-finality. A failed slot costs
4s with no height advance. The design is geographically fair, fork-resistant by
construction, and produces a deterministic live set from chain state. The main
operational burden is monitoring endorsement participation to catch
misconfigured nodes that silently underperform.
