# Settlement time estimation

This note hypothesises ballpark settlement time for sub-1.3 KB atomic messages.
It considers validation and optimistic aggregation.

For 100 nodes with a mesh network connection of 6 active peers, the majority of
messages will arrive within three hops. We assume 100ms for avergae hop time.

| Stage                        | Duration (ms) | Cumulative (ms) | Driving Factors                                               |
| :--------------------------- | :------------ | :-------------- | :------------------------------------------------------------ |
| 1\. Propose & Handoff        | 25ms          | 25ms            | Memory access, CBOR encoding, BLS signing, and stack handoff. |
| 2\. Proposal Broadcast       | 300ms         | 325ms           | 3 hops at 100ms                                               |
| 4\. Validation & Endorsement | 25ms          | 350ms           | Application-layer proposal check before node endorsement.     |
| 5\. Endorsement Broadcast    | 300ms         | 650ms           | 3 hops at 100ms                                               |
| 6\. Aggregate & Verify       | 25ms          | 650ms           | Optimistic path : Single BLS aggregate verification.          |
