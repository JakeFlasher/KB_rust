---
schema_version: "cacg.v0"
id: "be-limits-of-arbitrage"
title: "Limits of Arbitrage"
reading_id: "10_behavioral_finance"
summary: "Limits of Arbitrage: framing why rational arbitrageurs cannot always correct sentiment-driven mispricing — the four activation conditions (identifiability, convergence horizon, capital + agency constraints, transaction frictions) that the limits-of-arbitrage gate enforces and the resulting bounded-deviation equilibrium that Shleifer establishes against the textbook frictionless-arbitrage assump..."
tags: ["behavioral-finance", "arbitrage", "mispricing"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p098:0109"
    chunk_hash: "07c806da83ef5920df6c0bcfcdac87ec184a133f7efc67584cf833dc4a4cda11"
    page_range: [98, 99]
    quote: "arbitrageurs can become most constrained precisely when they have the best opportunities"
    edge_type: "supports"
card_hash: "037b5535c52f6b940647a54135d0e3f2dbabb4f93c9b3964609b8e5e6d0b851f"
---
# Limits of Arbitrage

## Intuition

Textbook finance ASSERTS that rational arbitrageurs eliminate mispricing: whenever the market price `P` deviates from fundamental value `V`, profit-seeking traders trade against the gap until `P → V`. Shleifer's limits-of-arbitrage theory rules out this conclusion under any of four widely-binding real-world conditions, leaving sentiment-driven mispricing as a stable equilibrium feature rather than a transient deviation. **Source:** Shleifer (2000) Ch.1 pp.1-27 + Ch.4 pp.89-111.

The four activation conditions form a sequential gate: arbitrage activates only if (i) the mispricing is identifiable (the arbitrageur has a credible estimate of `V` separate from `P`), (ii) the convergence horizon is bounded relative to the arbitrageur's holding capacity, (iii) capital and agency constraints permit the position to be carried through interim drawdowns, and (iv) transaction frictions are below the expected profit. Any one condition that fails leaves the mispricing in place — the arbitrage-activation gate diagram below shows the conjunction structure. **Source:** Shleifer (2000) Ch.4 pp.89-111.

```
<!-- primitive: arbitrage-activation-gate source: _diagram_primitives.md -->
   mispricing observed (P deviates from V_fundamental)
                       |
                       v
   +-----------------------------------+
   | Is mispricing identifiable?       |
   | (model risk / known fundamental)  |---No---> mispricing persists
   +-----------------+-----------------+         (fundamental risk)
                     | Yes
                     v
   +-----------------------------------+
   | Is convergence horizon bounded?   |
   | (vs noise-trader-risk hold)       |---No---> mispricing persists
   +-----------------+-----------------+         (noise-trader risk)
                     | Yes
                     v
   +-----------------------------------+
   | Capital + agency constraints OK?  |
   | (margin, drawdown, redemption)    |---No---> mispricing persists
   +-----------------+-----------------+         (agency / capital cost)
                     | Yes
                     v
   +-----------------------------------+
   | Trading frictions below profit?   |
   | (transaction + tax cost)          |---No---> mispricing persists
   +-----------------+-----------------+         (transaction cost)
                     | Yes
                     v
              arbitrage activates: P -> V_fundamental
```

Shleifer's central methodological move EXPLAINS the joint failure mode: arbitrage capital is concentrated in specialised intermediaries (hedge funds, prop desks, dealer balance sheets) whose own agency structure couples interim performance to capital availability. When mispricing widens, the same drawdown that opens the larger arbitrage opportunity simultaneously triggers redemptions or margin calls that shrink the arbitrageur's capital — so arbitrage capacity is endogenously contracting when it would be most valuable. **Source:** Shleifer (2000) Ch.4 pp.100-111.

## Definition

**Fundamental risk** is the risk that the arbitrageur's estimate of `V` is wrong (model risk) or that `V` itself shifts before convergence (cash-flow news risk). Shleifer ASSERTS that fundamental risk is largely irreducible: even a perfectly-rational arbitrageur with the best available information cannot rule out the possibility that the observed mispricing reflects private information the arbitrageur lacks. The textbook frictionless-arbitrage assumption presumes fundamental risk is zero by construction. **Source:** Shleifer (2000) Ch.4 pp.89-95.

**Noise-trader risk** is the risk that sentiment-driven demand shifts widen the mispricing further before convergence, forcing the arbitrageur to absorb a mark-to-market loss even when the position will eventually converge. Shleifer EXPLAINS that bounded-horizon arbitrageurs face a real cost from this short-term divergence: the mark-to-market hit can trigger forced unwinding (margin call, redemption, performance-based termination) before the convergence window opens, converting an eventual-winner trade into a realised loss. **Source:** Shleifer (2000) Ch.5 pp.112-130.

**Capital constraint** is the wedge between the arbitrageur's notional risk budget and the gross position size needed to enforce convergence at scale. The wedge widens with margin requirements, prime-broker haircut schedules, and redemption-driven liquidity drain. Shleifer ASSERTS that this wedge is materially binding for the universe of mispricings observed empirically: even when the mispricing-vs-volatility ratio favours the trade in expectation, the position size needed is often larger than the capital available. **Source:** Shleifer (2000) Ch.4 pp.95-100.

**Agency cost** is the principal-agent friction between the arbitrageur (manager) and the capital provider (outside investor, prime broker, fund LP). Shleifer EXPLAINS that the outside capital provider observes only realised P&L, not the manager's signal quality, so a drawdown that the manager knows is consistent with eventual convergence still looks indistinguishable to the outside provider from genuine skill failure — triggering redemption / margin call / position trimming on the drawdown rather than at convergence. The agency cost is therefore not a fixed friction but a state-contingent one that bites hardest when the trade is most valuable. **Source:** Shleifer (2000) Ch.4 pp.100-111 + Ch.5 pp.130-145.

**Transaction cost** is the residual frictional wedge from bid-ask spreads, brokerage, market-impact cost, short-borrow fees, and tax drag. Shleifer ASSERTS that for most observed mispricings transaction cost alone is below the expected profit; the binding constraints in practice are noise-trader risk, capital constraint, and agency cost rather than transaction friction. The transaction-cost gate is still listed in the activation sequence because some mispricings (small-cap, illiquid, hard-to-borrow names) fail at this gate even when the other three pass. **Source:** Shleifer (2000) Ch.4 pp.105-111.

## Mathematical Reasoning

The bounded-deviation arbitrage equilibrium EXPLAINS the resulting structural relationship between mispricing magnitude and arbitrage capacity: at the equilibrium, the maximum sustained price-value deviation `|P(t) − V(t)|` is bounded by a multiplicative factor `K` on the reciprocal of the arbitrageur's bounded horizon `H`. Symbolically, `|P(t) − V(t)| ≤ K · (1 / H)` where `K` aggregates noise-trader-risk variance, agency-cost severity, and capital-constraint tightness. **Source:** Shleifer (2000) Ch.5 pp.112-130.

The inequality DOCUMENTS three structural facts: longer arbitrageur horizons reduce the sustained mispricing; tighter capital constraints / higher agency frictions widen the sustained mispricing through a larger `K`; and the equilibrium permits non-zero mispricing even when fundamental value is publicly known and arbitrageurs are rational. The first two follow from comparative statics on the bound; the third is the central structural deviation from frictionless-arbitrage theory. **Source:** Shleifer (2000) Ch.5 pp.112-153.

The convergence horizon `H` is endogenous to the arbitrageur's funding structure: a longer-locked-up fund (private equity, endowment, family office) sustains longer `H` than a daily-redemption mutual fund or a short-term-leveraged prop desk. The clientele-segmentation of arbitrage capital is therefore a determinant of which mispricings can be eliminated — markets where long-horizon arbitrage capital dominates (large-cap, indexed, well-followed) show smaller and shorter-lived deviations than markets where short-horizon arbitrage capital dominates (small-cap, distressed, narrative-driven). The clientele-tier framing in [`be-investor-clientele-segmentation.md`](./be-investor-clientele-segmentation.md#definition) returns to this point. **Source:** Shleifer (2000) Ch.4 pp.100-111 + Ch.5 pp.130-153.

The limits-of-arbitrage gate APPLIES across the Shleifer 2000 anomaly inventory: the value premium ([`be-value-anomaly.md`](./be-value-anomaly.md#mathematical-reasoning)), the momentum effect ([`be-momentum-anomaly.md`](./be-momentum-anomaly.md#mathematical-reasoning)), and the post-earnings-drift / overreaction patterns ([`be-investor-overreaction.md`](./be-investor-overreaction.md#mathematical-reasoning)) all survive in equilibrium because at least one gate condition binds. The cross-vertical analogue under the efficient-markets / factor-exposure framing lives in subcorpus 09 (`pm-efficient-markets-and-anomalies.md`), which interprets the same patterns as factor-priced rather than as bias-driven; the per-10 BOUNDARY-DISCIPLINE in `_style_guide.md` partitions ownership. **Source:** Shleifer (2000) Ch.5 pp.112-153 + Ch.6 pp.154-174.

## See Also

- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) — the noise-trader-risk equilibrium that quantifies the convergence-horizon bound used in the inequality above.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the divergence framing that visualises bounded-deviation equilibria.
- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the overreaction / underreaction taxonomy that classifies mispricing sources.
- [`be-regret-matching-foundations.md`](./be-regret-matching-foundations.md#definition) — the Hart+Mas-Colell adaptive-strategies bridge that anchors clientele-weight dynamics under bounded-arbitrage equilibria.
- [`fa-shleifer-vishny-limits-to-arbitrage`](../22_fund_level_arbitrage/fa-shleifer-vishny-limits-to-arbitrage.md) — cross-set: performance-sensitive-capital limits-to-arbitrage (reading-14 primary derivation; reading-22 fund creation/redemption application; reading-10 behavioral owner).
## Escalate to Raw When

Open Shleifer 2000 *Inefficient Markets* Ch.4 + Ch.5 directly when any of the criteria below applies. **Source:** Shleifer (2000) Ch.4 pp.89-111 + Ch.5 pp.112-153.

- The mispricing under analysis violates a specific gate condition in a way that requires the original derivation — e.g., a case where fundamental risk is non-trivial and the arbitrageur's `V`-estimate carries quantifiable model risk. **Source:** Shleifer (2000) Ch.4 pp.89-100.
- The agency-cost mechanism on a specific arbitrage capital pool (hedge-fund LP redemption terms, prime-broker margin haircuts, prop-desk drawdown limits) requires the original Shleifer-Vishny 1997 derivation for the principal-agent contract. **Source:** Shleifer (2000) Ch.4 pp.100-111 + Ch.5 pp.130-145.
- A cross-vertical synthesis card needs the bounded-deviation equilibrium derivation that this card paraphrases symbolically — escalate to the Ch.5 §5.x algebra for the full proof. **Source:** Shleifer (2000) Ch.5 pp.112-153.
