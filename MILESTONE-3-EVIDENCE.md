# Catalyst Milestone 3 Requirements

This repo has been made publicly available as part of a [Catalyst project][m-1] completed by [Orcfax][m-2]. The following provides a mapping to completion requirements as established in the [milestone][m-3].

[m-1]: https://projectcatalyst.io/funds/12/cardano-use-cases-concept/orcfax-validators-reaching-l2-consensus
[m-2]: https://orcfax.io/
[m-3]: https://milestones.projectcatalyst.io/projects/1200180/milestones/3

## Important Notes

**The objective of this Catalyst project was to produce software which would serve as a Proof of Concept (PoC)** and which could be built on top of. The PoC has been developed within an Orcfax specific context, which is presented in the [context document](./context.md), and was preceded by an exploratory [literature review](./literature-review.md) aimed at analyzing available solutions.

To this end, the repository makes reference to on-going research and development which is beyond the scope of the Catalyst Project; reviewers of milestone deliverables must limit their review to the elements explicitly required. The table presented below clearly identifies the aspects of the PoC which are under review-- all other elements acknowledged or in development within this repository are outside the scope of Catalyst.

## Deliverables

| Approved requirement               | Implementation                                                           | Test                                                                                                                            | Evidence                                                           |
| ---------------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Receive external source data       | FX polling service and local cache                                       | A test validator must be able to receive data from an external source                                                           | [Video walkthrough][evid-1] demonstrates successful tests          |
| Apply validation logic             | Freshness, feed identity, timestamp and deviation checks                 | A test validator must be able to validate received data                                                                         | [Video walkthrough][evid-1] demonstrates successful tests          |
| Compare results with other nodes   | Each receiving node compares the proposed value with its own local value | A test validator must be able to compare its validation results with other test validators; nodes with matching values endorse. | [Video walkthrough][evid-1] demonstrates successful tests          |
| Arrive at consensus on datum value | Signed endorsements are counted against a two-of-three quorum            | Test validators must be able to reach consensus; quorum succeeds with 2/3 endorsements.                                         | [Video walkthrough][evid-1] demonstrates successful tests          |
| Publish code and test cases        | Public tagged repository release                                         | Recorded software walkthrough test                                                                                              | The publication of this repository and [Video walkthrough][evid-1] |

[evid-1]: https://youtu.be/2P3_upweQys
