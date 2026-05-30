---
schema_version: "cacg.v0"
id: "mt-funding-liquidity-fire-sales"
title: "Funding Liquidity, Fire Sales, and Limits to Arbitrage"
reading_id: "14_microstructure_and_trading"
summary: "Funding-constrained intermediaries forced to liquidate (fire sales) push prices below fundamentals; because arbitrageurs share those constraints, mispricing can persist and amplify through a funding/market-liquidity feedback."
tags: ["microstructure", "funding-liquidity", "fire-sales", "limits-to-arbitrage", "liquidity-spiral"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p304:0462"
    chunk_hash: "e9f70683006cbcef9eb288b7e8dff5cb512e6209c01d3a83401ae5f4eae837fb"
    page_range: [305, 305]
    quote: "be forced to liquidate positions in financial assets to deal with an unexpected funding need"
    edge_type: "defines"
---
# Funding Liquidity, Fire Sales, and Limits to Arbitrage

## Intuition

Classroom arbitrage assumes a trader who spots two claims to the same payoff trading at
different prices can buy the cheap one, sell the dear one, and pocket the gap risk-free as
the prices converge. The textbook objection is that real arbitrageurs do not have infinite,
patient capital: they borrow to finance positions and answer to outside investors who can
pull funding. A position that is right "in the long run" can be marked against the
arbitrageur in the short run, and the financiers — unable to tell a temporary widening of
the gap from a genuine loss — may cut the line. The arbitrageur is then *forced to
liquidate* at the worst moment. This is "performance-based arbitrage": the willingness of
others to fund you depends on your interim P&L, not on the ultimate truth of your bet.

Now layer in *who else is selling*. Mutual funds, insurers, and levered intermediaries hit
by redemptions, prudential caps, or margin calls dump assets regardless of value — a **fire
sale**. Fire sales depress prices; depressed prices shrink the collateral value of everyone's
book; shrunken collateral tightens funding; tighter funding forces still more liquidation.
Market liquidity and funding liquidity feed on each other, and in a crisis the loop runs
downhill — a *liquidity spiral*.

```
   noise-trader / forced supply shock
              |
              v
        price of A falls below V  ---------------------+
              |                                         |
              v                                         |
   arbitrageur collateral / mark-to-market drops        |  amplification
              |                                         |  feedback loop
              v                                         |
   financiers cut funding (prob phi)                    |
              |                                         |
              v                                         |
   forced liquidation: arbitrageur SELLS into the gap --+
              |
              v
        mispricing M1 persists / widens, not corrected
```

The punchline: the very agents who *should* correct mispricing are the ones whose selling
*deepens* it once funding is constrained, so the law of one price can fail for long stretches.

**Source:** Foucault, Pagano & Röell (2013) ch.9 §9.1, §9.4 pp.284, 305.

## Definition

Setup (single-mispricing convergence trade). Two bonds A and B pay the same value V with
certainty at date t = 2. At dates t = 0 and t = 1 bond A is undervalued by a *mispricing*
M_t > 0 (its price is V − M_t), while B is priced at V. The convergence trade is long A,
short B; it is built at most once, at date 0 (I_0 = 1) or date 1 (I_1 = 1), with position size
capped at one unit — the cap encodes the funding constraint.

- **Funding liquidity** — the ease with which a trader can finance/maintain a position
  without being forced to unwind. **Market liquidity** — the ease of trading an asset
  without moving its price. The two reinforce each other.
- **Performance-based arbitrage** — outside funding is withdrawn when interim performance
  is poor; if the arbitrageur intervenes at date 0 and the mispricing *widens* (prob κ),
  forced liquidation occurs with probability φ.
- **Fire sale** — a forced asset liquidation, e.g. an investor who must
  "be forced to liquidate positions in financial assets to deal with an unexpected funding
  need" (redemptions, creditor repayment, prudential rules), selling regardless of
  fundamental value and pushing price below V.
- **Limits to arbitrage** — the inability of arbitrage capital to fully correct mispricing
  because that capital is finite and itself subject to forced-liquidation risk.

**Source:** Foucault, Pagano & Röell (2013) ch.9 §9.4.1–§9.4.2 pp.302-306.

## Mathematical Reasoning

*Intervene at date 0 vs. wait.* Intervening at date 0 captures M_0 immediately. With
probability κ the mispricing persists and worsens to M_1; conditional on that, forced
liquidation (prob φ) realizes a loss of −M_1. Hence expected profit from date-0 entry is

```
  Pi_0(phi) = M_0  -  kappa * phi * M_1.
```

Waiting until date 1 forgoes M_0 but avoids liquidation risk: the trade is set up only if the
mispricing persists (prob κ), yielding

```
  Pi_1 = kappa * M_1.
```

Restricting attention to the interesting case κM_1 < M_0 (mispricing expected to shrink),
waiting is preferred iff Pi_1 > Pi_0, i.e. κM_1 > M_0 − κφM_1, which rearranges to the
threshold liquidation risk

```
  phi  >  phi_hat  =  (M_0 - kappa*M_1) / (kappa*M_1).        (indifference)
```

So even when mispricing is expected to fall, a high enough sensitivity-of-funding φ pushes
arbitrageurs to *defer* deployment. φ̂ is decreasing in future mispricing M_1.

*Endogenous mispricing and the feedback.* With a continuum of arbitrageurs indexed by
liquidation probability φ(i) = i uniform on [0,1], a fraction φ̂ deploys at date 0 and 1 − φ̂
remains for date 1. The date-1 market clears noise-trader supply y(P_A1) = 1 − δM_1 plus
forced arbitrageur supply ∫_0^{φ̂} φ(i) di against the remaining buy capacity 1 − φ̂.
Clearing gives

```
  M_1  =  (1/delta) * ( (1/2) phi_hat^2 + phi_hat ),          (clearing)
```

so M_1 is *increasing* in φ̂: the more capital committed early, the less is left to lean
against the date-1 fire sale, and the deeper the undervaluation.

The two relations close the loop. The indifference curve makes φ̂ *decreasing* in M_1; the
clearing curve makes M_1 *increasing* in φ̂. Equilibrium (φ̂*, M_1*) jointly solves both.
Comparative statics: a larger date-0 mispricing M_0 shifts the indifference curve, drawing
more capital to date 0, which by the clearing relation raises the equilibrium date-1
mispricing — the formal expression of "fire sales beget persistent mispricing."

**Source:** Foucault, Pagano & Röell (2013) ch.9 §9.4.1–§9.4.2 eqs.(9.19)–(9.24) pp.301-307.

## Boundary Notes

- *Assumptions.* Convergence is *certain* at t = 2 (no fundamental risk on the spread);
  the position cap of one unit and the binary date-0/date-1 choice are the entire funding
  friction. Relaxing certain convergence adds true divergence risk that strengthens limits
  to arbitrage further.
- *When it bites.* The mechanism is load-bearing precisely when κM_1 < M_0 (mispricing
  expected to shrink) yet φ is high — i.e. when liquidation risk, not the bet's merit,
  governs deployment. If liquidation risk is negligible (φ → 0) or convergence is fast,
  arbitrage capital corrects mispricing immediately and the law of one price holds.
- *Contrast.* This is distinct from inventory- or adverse-selection spreads in dealer
  models: here the friction is on the *arbitrageur's* balance sheet, not the dealer's
  quote-setting. It is also broader than a one-shot supply shock — the funding/market
  feedback (the *liquidity spiral*) is what makes a transient shock self-amplifying.
- *Empirical anchor.* The text grounds fire-sale price pressure and reversal in evidence
  (Coval & Stafford; Campbell-Giglio-Pathak; Ellul-Jotikasthira-Lundblad), but those are
  illustrations, not derivations of the closed-form above.

**Source:** Foucault, Pagano & Röell (2013) ch.9 §9.4.2–§9.4.3 pp.305-309.

## See Also

- [`mt-three-dimensions-liquidity`](./mt-three-dimensions-liquidity.md) -- market liquidity (depth/tightness/resiliency) is one leg of the funding/market feedback.
- [`mt-liquidity-adjusted-capm`](./mt-liquidity-adjusted-capm.md) -- why illiquidity and liquidity risk command a premium that arbitrage cannot fully erode.
- [`mt-bubbles-crashes-circuit-breakers`](./mt-bubbles-crashes-circuit-breakers.md) -- liquidity spirals as the engine of crashes and the policy response.

## Escalate to Raw When

The card sketches the equilibrium as the intersection of the downward-sloping indifference
curve (9.23) and the upward-sloping clearing curve (9.24); for the full fixed-point
existence/uniqueness argument, figures 9.4–9.7, the exact derivation of noise-trader
supply y(P_A1) = 1 − δM_1, and the integration of arbitrageur forced-supply, re-read
Foucault, Pagano & Röell (2013) §9.4.1–§9.4.2 (printed pp.329-334; PDF pp.302-307). For
the funding↔market liquidity feedback formalization and the crisis/liquidity-spiral
discussion, re-read §9.4.3.
