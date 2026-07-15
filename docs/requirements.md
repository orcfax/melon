# Consensus requirements

## Context

More general context is found in the [context document](./context.md).

## Design assumptions

The framing of the requirements is partly based on assumptions on the form of
the output.

- The software is, or mimics, a service on a computer.
- Each participant is running an instance of the software.
- State must be communicated between participants.
- If relevant, assume participants can communicate via TCP or UDP based.
- A participant is the holder of a private key of a key pair.
- Identity is established, at least initially, via the public key communicated
  out-of-band.
- Any participant may introduce data sourced externally. The atomic unit of
  this external data is called an atom.
- The state around which we must find consensus is essentially a sequence of
  atoms, perhaps together with auxiliary state supporting the overall system.
- Honest participants have a shared understanding of what validity checks
  should be run on the receipt of data (atoms, _etc_).
- Assume that a super majority of participants are honest and competent.

If these assumptions turn out to be inappropriate, then accommodations should be
made.

## Requirements

Must haves for Catalyst Milestone (parameters outlined below):

1. Participants efficiently communicate their state
1. Participants run validity checks on any data they receive. If appropriate,
   they will stop communicating with a participant providing bad data.
1. N participants reach consensus.
1. In normal conditions, time < T seconds between an atom being sent from an
   external source to it being included in consensus by most participants. If
   finality is not a property of the consensus mechanism, then replace this with
   an analogous condition (eg high probability _etc_).
1. In normal conditions, throughput averaging > V statements per second

Ballpark parameters:

- N = 100 number of nodes in the network
- K = 30 number of malicious or incompetent nodes
- T = 60 time of delivery
- V = 10 velocity of statements

## Notes

This document is a working document. It is based on our [context](./context.md).
