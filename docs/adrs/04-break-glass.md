---
title: "Break-Glass Disaster Recovery"
author: "@waalge"
date: 2026-04-19
tags:
  - governance
  - recovery
  - l1-settlement
---

## **Context**

Standard L1 state updates require a 67% threshold. If \>33% of validators are
lost, the L1 is "bricked." A recovery mechanism is needed to restart the network
with a new validator set when the cryptographic chain of trust is broken.

## **Decision**

We will implement a **Time-Decayed Registration of Intent** using a liveness
bitmask on the Cardano L1.

The following parameters are part of the global **Network Configuration**:

- `RECOVERY_HALF_LIFE`: The time constant governing the speed of threshold
  decay.
- `LAST_ORDERS`: The mandatory quiet period required before execution.

1. **Initiation:** Any validator from the last L1-recognized set can initiate
   recovery by registering a "Restart Intent." This records a start_timestamp
   and initializes a LivenessBitmask with the initiator's bit flipped to 1\.
2. **Aggregation (Liveness Flipping):** Other validators join the intent by
   submitting their own "Liveness Registration" to the L1. Each successful
   registration flips the corresponding bit in the bitmask. The recovery becomes
   "Eligible" when the number of flipped bits (nodes registered as alive) meets
   the required_threshold for the current time.
3. **Exponential Decay Function:** The threshold for eligibility follows a
   doubling-time rule. As the number of available nodes halves, the required
   wait time for eligibility doubles.  
   Let `t` be hours passed since initiation, then

```math

   threshold(t) = total_keys * (RECOVERY_HALF_LIFE / (t + RECOVERY_HALF_LIFE))
```

**Example (100 Nodes, `RECOVERY_HALF_LIFE = 4 hours`):**

- **T=4h:** 50 nodes (50% of set) can recover.
- **T=12h:** 25 nodes (25% of set) can recover.
- **T=28h:** 12 nodes (12.5% of set) can recover.

4. **Last orders:** Once the LivenessBitmask meets the threshold(t), the
   proposal enters the last orders phase.
   - This window resets if a new node registers (flips a bit), ensuring all
     honest nodes have time to be represented in the new set.
   - The recovery can only be executed if the window passes without a newer L2
     block being posted or a new node registering.
5. **Liveness Preemption:** A standard 67% State Update always takes priority.
   If processed, it voids all active "Restart Intents" and the associated
   bitmasks. This is the primary defense against a partition-induced recovery
   when the network is actually healthy.
6. **Slashing (TBC):** Deferred. Rejection via preemption serves as a diagnostic
   signal for partitioned nodes.

## **Status**

Proposed

## **Consequences**

- **Positive:** Bitmask provides a clear audit trail of who survived; the
  `LAST_ORDERS` prevents a "rush" execution and gives the live chain a fair
  window to preempt.
- **Negative:** Increased L1 transaction count as each node must flip its own
  bit (though these can be batched).
- **Neutral:** The "New Validator Set" is defined precisely by the bits flipped
  in the winning intent's mask.

## **Dissent, counter, and comments**

- **L1 Congestion:** If Cardano is congested, `LAST_ORDERS` must be large enough
  to allow the "Live Chain" (67%) to land a transaction and kill the recovery.
- **Batching:** To save fees, multiple nodes should be able to flip their bits
  in a single L1 transaction using a batched signature.
