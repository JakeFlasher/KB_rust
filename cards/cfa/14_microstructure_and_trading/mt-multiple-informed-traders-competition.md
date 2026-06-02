---
schema_version: "cacg.v0"
id: "mt-multiple-informed-traders-competition"
title: "Multiple Informed Traders and Competition Among the Informed"
reading_id: "14_microstructure_and_trading"
summary: "When several informed traders share the same signal, strategic restraint erodes: each trades more aggressively, prices reveal information faster, and the informed traders' collective rent shrinks below the monopolist case."
tags: ["microstructure", "informed-trading", "kyle-model", "price-discovery", "imperfect-competition"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p116:0150"
    chunk_hash: "bd4f8d7174f3feeab01921a345b7204f0528e7bc16bcc34eafabd9e489be66e9"
    page_range: [116, 117]
    quote: "the informed traders compete among themselves for the available profits and, in so doing, reduce the total available rents to be shared"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p123:0199"
    chunk_hash: "813d7ee7fc5908aaa91e6adf637ca70d15725da69fa1014c7d4d6a1e746327f9"
    page_range: [124, 124]
    quote: "an increase in the fraction of informed traders accelerates price discovery"
    edge_type: "supports"
card_hash: "1db163056fd4fef971cdbe54e7dc26dde3657ba5e534de380aa27f7c48ee4b90"
---
# Multiple Informed Traders and Competition Among the Informed

## Intuition

A single informed trader in the Kyle model is an *information monopolist*: she rations her
private signal across trading rounds precisely because she is the only one exploiting it.
Trading too aggressively would move the price against her before she finishes accumulating
her position, so she hides inside the noise-trader order flow and lets information leak out
slowly. Now drop a second, third, or M-th trader holding the *same* signal into the same
batch auction. Each one fears that the others will trade on the news first and capture the
gain, so the monopolist's discipline collapses into a race. Each trader front-loads,
information floods into the order flow earlier, and the price converges to fundamental value
faster than it ever would under a monopolist.

The competitive externality is purely strategic, not informational: every trader holds an
identical signal, so no one *learns* anything new from a rival. What changes is the *timing*
incentive. Because rivals will impound the signal whether or not I hold back, holding back
only forfeits my share of the rent — so I stop holding back. The faster information enters
the price, the smaller the residual mispricing left to harvest, and the smaller the total
pie all the informed traders divide.

```
   ONE INFORMED TRADER (monopolist)        MULTIPLE INFORMED TRADERS (competitors)
   price                                    price
   v |               .--- v (final)        v |        .------------- v (final)
     |          .---'                        |    .--'   <- impounded EARLY
     |     .---'   slow, rationed leak       |  .'         (aggressive front-loading)
   p0|---'                                  p0|-'
     +----------------------> time            +----------------------> time
   rent to informed: LARGE                   rent to informed (collective): SMALLER
   λ roughly constant over rounds            λ high early, then collapses toward 0
```

**Source:** O'Hara (1995) §4.2 "Price Behavior and Multiple Informed Traders" pp.106-111.

## Definition

Take the multi-period Kyle [1985] batch-auction structure (see
`mt-kyle-strategic-informed-trader-lambda`) and replace the single insider with M informed
traders who each observe the same liquidation value v and each know the number M of
competitors. Holden and Subrahmanyam (HS, as restated by O'Hara) retain the original Kyle
difference-equation structure but solve it for M ≥ 1.

- **Identical informed agents.** All M traders observe the same signal and are identical in
  every respect, so in equilibrium each conjectures the others behave identically. This
  symmetry reduces the M-player game to a single representative trader's decision problem.
- **Imperfect competition.** With M finite, each informed trader internalizes that her own
  order moves the price, but she is no longer the sole controller of the information flow.
  She is a strategic player, not a price taker (that would be the competitive
  rational-expectations limit) and not a monopolist (M = 1).
- **Endogenous entry.** Because informed trading earns positive expected profit, entry can be
  made endogenous: M adjusts until the per-trader expected profit equals the cost of becoming
  informed.

**Source:** O'Hara (1995) §4.2 pp.106-110.

## Mathematical Reasoning

Use round index n = 1,…,N and write the per-round market-maker pricing rule and aggregate
informed demand in clean notation (reconstructed; O'Hara's printed equations are OCR-garbled):

- Pricing: Δp_n = λ_n (Δx_n + Δu_n), where Δx_n is total informed order flow and Δu_n is
  noise-trader order flow in round n.
- Each informed trader's order is linear in the gap between value and price:
  Δx_n^(individual) = β_n (v − p_{n-1}) Δt_n, and total informed flow is M times this.
- Σ_n = Var(v | order-flow history through round n) measures residual price inefficiency.

Two comparative-statics channels drive the result as M rises:

1. **Rent-sharing (direct) channel.** A larger M splits the surplus from the private signal
   across more traders, mechanically reducing each trader's per-capita profit.
2. **Behavioral (strategic) channel.** Each trader best-responds to the conjectured
   aggressiveness of the others. Under risk neutrality this raises *aggregate* informed
   trading: the total scale of information-based trading increases with M, so prices become
   more informative sooner.

Key qualitative results (risk-neutral case):

- Unlike the monopolist case where λ_n is held roughly constant and Σ_n declines smoothly,
  with competing informed traders **λ_n is larger in early rounds and then falls rapidly**,
  and Σ_n collapses sharply toward zero. The greater M is, the faster both λ_n and Σ_n fall.
- **Continuous-time-of-trading limit (N → ∞, M fixed):** for the last auction before any
  calendar cutoff t, Σ → 0 and λ → 0; information is revealed in an arbitrarily small window,
  so it is essentially impounded immediately. In the *first* round, market depth is tiny
  (λ very large) and the expected informed quantity → 0, because any first-round trade moves
  the price so much it destroys the trader's own informational rent.
- **Many-trader limit (M → ∞, N fixed):** at the first auction Σ_1 → 0, λ_1 → 0, expected
  informed quantity → ∞, and p_1 → v. Full revelation in the first interval — the competitive
  rational-expectations outcome re-emerges inside a strategic Kyle framework.

Net effect on collective rent: faster revelation shrinks residual mispricing, so the *total*
profit available to the informed shrinks relative to the M = 1 monopolist benchmark — the
informed "compete among themselves… and… reduce the total available rents to be shared."

**Source:** O'Hara (1995) §4.2 pp.107-112 (Holden–Subrahmanyam [1992] results as restated).

## Boundary Notes

- **Risk neutrality is load-bearing.** All the "more traders ⇒ faster revelation ⇒ more
  liquidity" results assume risk-neutral informed traders and market makers, so only mean
  effects matter. Under O'Hara's exposition of Subrahmanyam [1991b] using the Kyle [1984]
  model, risk-averse informed traders flip the conclusion: increasing M can *decrease*
  liquidity, with λ unimodal in M (λ first rises — risk-averse traders trade less
  aggressively — then falls as aggregate risk tolerance grows). Competition among informed
  traders therefore does *not* necessarily improve information revelation.
- **Public vs. private information.** With M fixed, more public information unambiguously
  makes prices more efficient and erodes the informed's advantage. With M endogenous, better
  public signals also induce some informed traders to exit; O'Hara reports Kyle's result that
  the public-signal gain still dominates, so prices end up more informative overall.
- **Noise-trading comparative static depends on endogeneity.** With M fixed, more noise
  trading raises informed profits without changing price informativeness (the informed scale
  up to keep their relative footprint). With M endogenous, extra noise induces entry, raising
  aggregate informed trading and making prices *more* informative.
- **Contrast with siblings.** The single-trader monopolist benchmark and the constant-λ,
  martingale-price property live in `mt-kyle-strategic-informed-trader-lambda` and
  `mt-prices-martingale-information-process`; this card is specifically the *M > 1* departure
  from those baselines. The Foster–Viswanathan [1993] elliptical-distribution extension is
  noted by O'Hara but not derived here.

**Source:** O'Hara (1995) §4.2 pp.108-112.

## See Also

- [`mt-kyle-strategic-informed-trader-lambda`](./mt-kyle-strategic-informed-trader-lambda.md) — the M = 1 monopolist baseline whose constant-λ rationing this card relaxes.
- [`mt-prices-martingale-information-process`](./mt-prices-martingale-information-process.md) — how information impounds into prices; competition accelerates that process.

## Escalate to Raw When

You need the explicit Holden–Subrahmanyam linear-equilibrium constants (a_n, β_n, λ_n, Σ_n),
the difference-equation system, or the formal limit derivations (Eqs. 4.33–4.36 in O'Hara):
the OCR garbles every one of these equations and the algebra behind the λ-collapse and the
M → ∞ / N → ∞ limit results. Re-read O'Hara (1995) §4.2 pp.110-112 for the exact equilibrium
characterization, and pp.108-109 for the risk-averse (Subrahmanyam [1991b]) unimodal-λ proof.
