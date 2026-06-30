# Capacity & Bottlenecks

This considers the capacity and bottlenecks of a 100-node network using 1.3 KB
atomic messages.

## Summary

The process is likely Egress bound. Future iterations may consider message
aggregation, despite the likely increase in propogation times.

## Egress Bandwidth

Data per Item Lifecycle (Per Node):

- Proposal Forwarding: (1.3 KB \+ 250 bytes) x 6 peers ≈ 9.3 KB
- Endorsement Forwarding (100 nodes): (0.15 KB \+ 250 bytes) x 100 msgs x 6
  peers ≈ 242.4 KB
- **Total:** \~252 KB

Egress bandwidth to throughput is linear. Example figures using the above
estimate: 2 Mbps corresponds to 1 Items/s

## CPU Capacity

### Optimistic Execution Path

Nodes assume all incoming signatures are valid, performing aggregation prior to
a single verification step.

Computation per Proposal:

- 1 Proposal Verification (BLS): 2ms
- Aggregation Math (Summing points): \< 1ms
- 1 Aggregate Verification: 2ms
- Total CPU Time: \~5ms

Throughput Limit (Single Core): \~200 Proposals/sec.

### Fallback Inspection Path (Failure Recovery)

If the aggregate verification fails, the node reverts to a logarithmic search to
identify the malformed signature(s). This should be a rare event, and results in
the exclusion of a bad node from the network.
