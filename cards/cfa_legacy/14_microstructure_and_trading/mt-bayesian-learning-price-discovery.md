---
schema_version: "cacg.v0"
id: "mt-bayesian-learning-price-discovery"
title: "Bayesian Learning and Price Discovery: Quotes Converge to Full-Information Value"
reading_id: "14_microstructure_and_trading"
summary: "In asymmetric-information microstructure, the market maker Bayes-updates beliefs from observed order flow, so quotes track conditional expectations and, over repeated trades, converge to the asset's full-information value."
tags: ["microstructure", "bayesian-learning", "price-discovery", "asymmetric-information", "market-efficiency"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p073:0098"
    chunk_hash: "5e64db8c6228c931fa9ade7d61a45092a69525545f010851e8945501c8a12557"
    page_range: [73, 74]
    quote: "in Bayesian learning models it will be the case that prices ultimately converge to the true value"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p091:0138"
    chunk_hash: "c22158b4cf3246a9c97d1d51a04f05fd2f07dadb3577afe630bc648c6d1e5941"
    page_range: [91, 91]
    quote: "the probability assigned to the high value by the dealers converges to 1 (they eventually discover the correct value)"
    edge_type: "supports"
---
# Bayesian Learning and Price Discovery: Quotes Converge to Full-Information Value

## Intuition

The central insight of information-based microstructure is that *trades are themselves
information*. When a counterparty insists on buying, the market maker cannot tell whether
that buyer holds favorable private news or is merely an uninformed liquidity trader. But
because informed agents buy on good news and sell on bad news, the *direction* of trade
shifts the probability the market maker assigns to high versus low value. The market maker
therefore protects himself by adjusting his beliefs about the asset's value conditional on
the type of trade observed, and these revised beliefs feed directly into the quotes he
posts. Price adjustment is thus *isomorphic* to belief revision: there is no separate
pricing rule beyond "quote the conditional expectation."

Run this forward and a price-discovery dynamic emerges. Each trade is a noisy signal of the
underlying value; the market maker treats his posterior from one trade as the prior for the
next and re-applies Bayes' Rule. When the news is good, buy orders predominate, the posterior
weight on the high value drifts up, and quotes ratchet upward — and symmetrically downward
on bad news. The order flow leaks the informed traders' information into prices.

```
   nature picks V (low V_L or high V_H, unknown to MM)
                       |
        informed trade WITH the truth; uninformed trade randomly
                       |
   order flow  ->  MM applies Bayes' Rule  ->  posterior belief  ->  quote = E[V | history]
        ^                                                                    |
        |____________________ posterior becomes next prior __________________|
                       |
   over many trades:  belief -> point mass on true V,  price -> full-information value
```

Because the limit point of the belief sequence is the true value, prices "discover" the
fundamental over time even though no single trade reveals it. The convergence is a property
of the Bayesian dynamics, not of any special quoting heuristic.

**Source:** O'Hara (1995) §3.2–§3.3 *Information-Based Models* pp.57–65.

## Definition

Let the asset's eventual value be a random variable `V` (in the binary illustration,
`V ∈ {V_L, V_H}`). The market maker's information set at the start of period `t` is `I_t`,
summarized by a prior belief over the value of `V`. Define the order-type events `B_t` (a
trader wishes to buy from the market maker) and `S_t` (a trader wishes to sell to the
market maker).

A competitive, risk-neutral market maker sets quotes equal to *conditional expectations of
the asset value given the observed trade direction*:

- Ask: `a_t = E[V | I_t, B_t]`
- Bid: `b_t = E[V | I_t, S_t]`

The bid–ask spread `a_t − b_t > 0` arises purely because a buy revises the value estimate
upward and a sell revises it downward — an adverse-selection spread, with no exogenous
transaction or inventory cost required.

The *learning problem* is: use the indirect evidence in order flow to infer the unknown
true value of `V`. The order-flow signal may be a single trade (sequential-trade models)
or a batched net demand (Kyle-style call markets); in either case the inference is an
application of Bayes' Rule.

**Source:** O'Hara (1995) §3.3 and Appendix §3.A pp.59–62, pp.77–78.

## Mathematical Reasoning

**Belief update (Bayes' Rule).** Treat each observed trade `Q_t ∈ {B, S}` as data. The
posterior probability that the value equals a candidate level `v` given the trade is

```
                 Pr{Q_t | V = v} · Pr{V = v}
Pr{V = v | Q_t} = ----------------------------------------------------
                 Σ_{v'} Pr{Q_t | V = v'} · Pr{V = v'}
```

i.e. `posterior ∝ prior × likelihood`, normalized by the marginal likelihood of the data.
The denominator is the *unconditional* probability of seeing that trade; crucially it is
**not** symmetric across buys and sells whenever quotes are away from full-information
levels — good news makes buys more likely, bad news makes sells more likely.

**Quotes as expectations.** In the binary case, writing the posterior on the high state as
`θ_t = Pr{V = V_H | I_t}`, the *pre-trade conditional value estimate* (the dealer's
reference price, conditioned on the public information set `I_t` before the next trade
arrives) is `p_t = θ_t V_H + (1 − θ_t) V_L = E[V | I_t]`. The bid and ask are the *same value
conditioned additionally on the trade direction* — i.e. the trade-conditional expectations:
`a_t = E[V | I_t, B_t]`, `b_t = E[V | I_t, S_t]`. Because informed buyers raise the
conditional weight on `V_H` (and informed sellers lower it), we have `a_t > p_t > b_t`; here
`p_t` is the reference value estimate, not necessarily the realized bid-ask midpoint.

**Recursion and convergence.** The period-`t` posterior becomes the period-`(t+1)` prior,
so beliefs follow a Bayesian martingale: `E[θ_{t+1} | I_t] = θ_t`. As trades accumulate,
the preponderance of informed trades on the correct side drives `θ_t → 1` if `V = V_H`
(and `θ_t → 0` if `V = V_L`), so the belief sequence converges to a point mass on the true
state and quotes converge to the full-information value. The price sequence is therefore a
martingale that becomes, in the limit, strong-form efficient.

**Comparative statics (qualitative, per O'Hara).** The spread widens with (i) a higher
proportion of informed traders, (ii) more dispersed / more informative underlying value
uncertainty, and (iii) lower uninformed (liquidity) trading elasticity. No worked
arithmetic is needed: each is a monotone consequence of how much a single trade moves the
posterior.

**Source:** O'Hara (1995) §3.3 and Appendix §3.A.1 *Bayes Rule* pp.59–65, pp.78–79.

## Boundary Notes

- **Assumptions.** Risk-neutral, competitive market maker with unlimited capital, no
  bankruptcy, and a short horizon; inventory is neutralized *by construction* so that the
  card isolates how information per se moves prices. Relaxing risk-neutrality or capital
  re-introduces inventory effects (sibling inventory models).
- **Convergence is asymptotic, and only along the realized path.** Prices converge to the
  true value in the limit; en route they are merely semi-strong-form efficient (they
  reflect the market maker's public information set, not the informed traders' private
  signal). Different order-flow realizations trace different adjustment paths — each a
  martingale — so "efficiency" is path-dependent and, as O'Hara notes (citing Black 1989;
  Easley–O'Hara 1992b), of limited diagnostic content in a dynamic setting.
- **Where it holds vs. breaks.** The pure adverse-selection spread requires that order
  direction genuinely carries information; if all traders are uninformed, buys and sells are
  equally likely, the posterior does not move, and the spread collapses to its
  transaction-cost floor. Short-sales constraints or inventory frictions slow (or distort)
  convergence relative to the frictionless benchmark.
- **Contrast with siblings.** Versus Copeland–Galai, the spread here is not a static
  balancing of expected gains and losses but a *belief-revision* spread. Versus Kyle, the
  signal is a discrete trade direction rather than a continuous batched net demand, but both
  are the same Bayesian learning problem with different observation structures.

**Source:** O'Hara (1995) §3.3 *The Glosten-Milgrom Model* pp.59–66.

## See Also

- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) — the
  sequential-trade model that instantiates this learning problem and yields the
  adverse-selection spread.
- [`mt-prices-martingale-information-process`](./mt-prices-martingale-information-process.md) —
  the martingale property of transaction prices that the Bayesian recursion produces.
- [`mt-market-viability-no-trade-breakdown`](./mt-market-viability-no-trade-breakdown.md) —
  conditions under which adverse selection is severe enough that the learning market fails
  to open.

## Escalate to Raw When

O'Hara *proves* the convergence and martingale properties this card only sketches; the OCR
garbles every displayed equation (e.g. eq. 3.3–3.5 quote formulas on PDF p.70, and the
Bayes-Rule derivations eq. 3.10–3.15 in the Appendix). For the exact conditional-expectation
quote formulas re-read PDF pp.69–70; for the formal Bayes-Rule statement and the
worked binary buy/sell example re-read Appendix §3.A.1 on PDF pp.85–87; for the general
normally-distributed-signal learning result and its convergence proof re-read the remainder
of Appendix §3.A on PDF pp.88+.
