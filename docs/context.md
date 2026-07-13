# Context

This document aims to provide a basis from which to frame the requirements and
other documentation.

It describes something between what currently exist and where we might want to
be.

## Oracles

An oracle is a service that makes information available to the context of code
executing within the blockchain. This information is used to determine what of
transactions are deemed valid. Thus those effected by such transactions need to
have trust in the oracle system. By its nature the information from an oracle
does not come with the guarantees of the blockchain, and so to be useful the
oracle must present its own integrity model - the justification of why anyone
should have confidence the oracle service provider is accurate and reliable.

## Integrity models

In any context where one party is to be dependent on another for accurate and
reliable data provision, there is a question of integrity.

**By authority**. This is where a single centralised authority provides
information. The integrity model argues that the combination of legal
repercussions, and financial and reputation damage ensures the data provider is
highly motivated to report information accurately and reliably.

An advantage of this approach is that it is relatively simple. A disadvantage of
approach is that it has a single point of corruption and or failure. It relies
solely on a single entity to be honest and competent.

Blockchains are designed specifically on how to organise
[_"Computer Systems Established, Maintained, and Trusted by Mutually Suspicious Groups_](https://nakamotoinstitute.org/library/computer-systems-by-mutually-suspicious-groups/).
A blockchain native approach assumes that most but not all participants are
honest and competent while some participants of the network may be malicious or
incompetent.

**By consensus**. If a decentralised set of participants are to provide a
service such as an oracle service, they must first agree on the information they
provide. That is, they must find consensus.

## Orcfax

Orcfax is an aspiring decentralised oracle on the Cardano blockchain.

The data it provides is arranged into feeds. A feed is generally a time series
data. For example the ADA/USD exchange rate.

A feed can be thought of as data pipeline. Data is periodically collected from
numerous sources. This is input to a normalization and aggregation process which
outputs a statement belonging to a feed. Each execution of the data pipeline
results in a statement.

Currently the Orcfax is centralised. The goal is to make it decentralised. The
decentralised Orcfax network is EchoNet; it's participants 'Orcfax validators'
These validators are recognized as being the owners of specific assets on the
Cardano L1.

A non-exhaustive list of tasks that validators must do includes:

- running the existing collectors and feeds
- ensuring maintenance of existing code base
- ensuring health of data sources / definitions of feeds
- sunsetting and discontinuing underused feeds
- defining and developing new feeds
- ensuring the health of the network (removing bad validators and collectors)
- maintaining optimal remuneration parameters

Many of these actions require consensus.

In the present proposal we are mostly concerned with the consensus involved in
the running of existing feeds. We hope that the underlying techniques can be
directly extended to the other tasks contexts.

The running of the existing collectors and feeds is synonymous with the task of
producing statements.

A statement is the interface between Orcfax and its consumers. Consumers are the
users of Cardano who are in some way implicated in transactions that consume
Orcfax data. For example, an individual who took a collateralized loan via a
dApp that integrated Orcfax price feeds, and that was liquidated when exchange
rates changed is a consumer.

A statement is _published_ on the L1 only if it has a valid signature. Currently
a single signer is used. It is envisioned that a FROST-like method would allow
the transition from a single signer to a multi-signature (k of n) arrangement.
However, Orcfax is not committed to this design and this aspect can be change if
it is deemed desirable.

## Participants behaving badly

An aim of Consensus is to uphold the health of the network in spite of
participants behaving badly: either by incompetence or with malicious intent.

It may be helpful to bear in mind _What actions are deemed bad for the health of
the network?_

**An unresponsive participant.** We cannot know if this is due to malice,
incompetence, or wider network failure beyond what is reasonable to expect is
within the participants control.

**Collectors reporting spurious information.** For example, were it possible for
a collector to simply copy and repeat anther collectors' output then they add no
additional integrity to the network.

**Biased aggregators**. Depending on the design, there may be the possibility of
an aggregator perform selection bias, ignoring collectors with unfavourable
outputs.

**Copycat participants**. Depending on the design, it maybe possible for a
participant to simply copy the results of another participant. This makes the
network less robust and reliable.

**Half arsed Participants**. Suppose there is no check on publishing the data
trace then a participant will begin ignoring a stipulation.

We don't yet have complete answers to how the design addresses these issues.
Such design decisions will likely have a bearing on who is reaching consensus
how and over precisely what.
