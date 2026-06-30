---
title: "Preamble and Genesis"
author: "@you"
date: 2026-05-03
tags:
  - genesis
---

## Context

To launch the Melon network, we need a deterministic starting point that
initializes the geological timeline, the state root, and the initial validator
set. Rather than creating a unique "Genesis Block" format, we want to utilize
the existing period_block structure to simplify the codebase.

## Decision

The network's birth is defined as the finalization of Period 0, Height 0. This
event is governed by a Genesis Manifest.

1. The Genesis Period Block. The very first block in the ledger history is a
   period_block with the following coordinates:

- Coordinate: pair 0, 0 (Period 0, Height 0)
- Parent ID: 0x00...00 (The null hash)
- Body: A period_body containing the initial state_root and the Genesis
  Manifest.

2. Initialization Sequence. A node boots into the Genesis state by performing
   the following steps:

- Manifest Loading: The node loads a local or L1-provided manifest file.
- Clock Alignment: The node waits until the wall-clock time matches the
  GENESIS_TIME defined in the manifest.
- State Hydration: The node initializes its local KV store to match the
  state_root provided in the Genesis block (often containing the initial
  distribution of assets).
- First Proposer: The validator set defined in INITIAL_LIVE_SET begins the
  consensus process for Standard Block (0, 0, 1, 1)—the first block of Period 0,
  Epoch 0, Slot 1, Height 1.

3. The Genesis Manifest (Extensible). For the launch of the network, the
   manifest must contain:

- `PROTOCOL_VERSION`: 0
- `GENESIS_TIME`: The agreed-upon launch timestamp.
- `SLOT_DURATION`: e.g., 4000 (4 seconds).
- `EPOCH_LENGTH`: e.g., 21600 (number of slots in 24 hours).
- `SIGNING_SCHEME`: 1 (BLS12-381).
- `INITIAL_LIVE_SET`: The public keys of the founding validators.
- `NETWORK_ID`: A unique identifier to prevent cross-chain replay attacks.
  Mainnet is set to `0`.

## Status

Proposed

## Consequences

- Positive: No special "Genesis" code path; the first block is just a standard
  period_block.
- Positive: Enables "Re-Genesis" in the future by simply dropping a new Period
  block with a new state root.
- Negative: Validators must be perfectly time-synced to avoid missing the first
  slots of the new network.
