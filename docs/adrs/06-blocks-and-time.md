---
title: "Blocks and Time"
author: "@waalge"
date: 2026-05-03
tags:
  - consensus
  - architecture
  - cbor
  - time-tracking
---

## Context

The Melon network requires a deterministic way to track time and protocol
evolution across genesis, hard forks, and regular validator rotations. Standard
block validation is optimized for speed. Structural transitions require
different data and handling.

## Decision

We define a distinct block types and use a coordinate system of time. This
structure is codified in the protocol's CBOR/CDDL specification and ensures
absolute temporal and cryptographic continuity.

### Coordinates

The network tracks growth via a hierarchical coordinate system.

| Coordinate | Scope              | Behavior                                                           |
| :--------- | :----------------- | :----------------------------------------------------------------- |
| Period `P` | Protocol Lifecycle | Incremented by a Period Block. Identifies the current rule set.    |
| Epoch `E`  | Operational        | Incremented by an Epoch Block. Defines the validator set rotation. |
| Slot `S`   | Temporal           | Absolute time units since the start of the epoch                   |
| Height `H` | Cumulative         | Total count of all blocks since Period 0\. Never resets.           |

### Block Variants

The protocol implements three distinct block types, each with a specific header
and body structure.

Each header includes the (relevant) coordinate, `parent_id` and signature.

#### Period

- Coordinate: pair `(P, H)`
- Body:
  - state_root: Definitive state anchor for the previous Period's tail.
  - manifest: A map of protocol-defining parameters.
- Function: Hard forks, protocol upgrades, and disaster recovery.

#### Standard

- Coordinate: quad `(P, E, S, H)`
- Body: A vector of item IDs (transactions/certs).
- Function: Standard ledger growth.

#### Epoch

- Coordinate: triple `(P, E, H)`
- Body:
  - state_root: Execution layer checkpoint.
  - validator_set_root: Cryptographic anchor for the new validator set.
- Function: Validator set rotation and state checkpointing.

### Period Manifest

A Period Block's manifest defines the fundamental parameters for the duration of
the Period. While the manifest is designed to be extensible to accommodate
future protocol features, its initial structure includes:

- `PROTOCOL_VERSION`: The logic set required to process this Period.
- `GENESIS_TIME`: UNIX timestamp for Slot 0 of this Period.
- `SLOT_DURATION`: Milliseconds per slot (e.g., 4000ms).
- `EPOCH_LENGTH`: Number of slots before a mandatory Epoch Block.
- `SIGNING_SCHEME`: The cryptographic algorithm used for QCs (e.g., BLS12-381).
- `INITIAL_LIVE_SET`: List of public keys authorized for the start of the
  Period.

### Implementation Details

- CBOR Encoding: Using variable-length integers, the coordinates add \~10–15
  bytes of overhead.
- State Continuity: Every block (including Period blocks) must reference a
  `parent_id`. The `state_root` in Epoch and Period blocks ensures the Execution
  Layer and Sequence Layer remain synchronized.
- Implicit Timing: Slot duration is fixed per Period. A node calculates the
  current slot.

## Status

Proposed

## Consequences

- Positive: Provides a unique, absolute index for every block via; simplifies
  hard-forks via the Period primitive; enables light clients to verify state via
  Epoch checkpoints.
- Neutral: The ledger is organized into Periods, subdivided into Epochs.
