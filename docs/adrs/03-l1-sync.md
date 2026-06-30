---
title: "L1 Sync"
author: "@waalge"
date: 2026-04-19
tags:
  - consensus
  - l1-settlement
  - governance
---

## **Context**

Cardano (L1) consumes L2 state but updates are expensive. We want both a cheap,
high-frequency "Fast-Path" verification and reliable, lower-frequency "Total
State" synchronization.

## **Decision**

We will implement a tiered **L1 Synchronization** protocol for verifying L2 data
based on L1 state staleness.

1. **On-Demand Sync:** Updates are incentive-driven. Any user can pay to sync
   the L1 to a recent L2 state by providing a valid **State Update Proof**.
2. **Update Proof Requirement:** The L1 Plutus script updates its BaseVK and
   RegistryRoot only when receiving:
   - The **New Base VK** and **New Registry Root**.
   - A **Signer Bitmask** and **Aggregate Signature** from the currently stored
     L1 validator set (min 67%).
3. **Tiered Verification Paths:**
   - **Fast-Path (Limited State):** Uses **Subtraction Optimization** for
     near-unanimous signatures:
     `effective_vk = base_vk - missing_pks.iter().sum()` Requires the L1 to have
     a recently updated BaseVK.
   - **Slow-Path (Total State):** Results in a **Canonical View** anchored on
     the L1, merklizing:
     - Full validator keys.
     - **Total execution state** (including all processed items and current
       state machine variables).  
       Enables cheap Merkle proofs of inclusion after the initial root anchor.
4. **Continuous L2 Evolution:** New Registration items are used for L2 consensus
   after a lookback window but only become "Settlement Capable" on the L1 after
   a State Update transaction.

## **Disaster Recovery**

A future ADR will consider recovery for broken cryptographic chains of trust.

## **Dissent, counter, and comments**

- **Sync Drift:** Fast-Path availability degrades as the L2 evolves ahead of the
  L1.
- **Incentives:** High-frequency users are incentivized to fund L1 updates to
  maintain cheap Fast-Path access.

## **Status**

Proposed

## **Consequences**

- **Positive:** Decouples L2 speed from L1 cost; provides cheap settlement with
  a reliable fallback.
- **Negative:** Increased client-side complexity to track L1 staleness and
  construct proofs.
- **Neutral:** L1 remains a passive observer, moving its anchor only when
  provably directed.
