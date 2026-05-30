---
schema_version: "cacg.v0"
id: "mt-market-viability-no-trade-breakdown"
title: "Market Viability: When Adverse Selection Causes Market Breakdown"
reading_id: "14_microstructure_and_trading"
summary: "A securities market is viable only if enough uninformed (liquidity) trade is present; if the threat of informed trading is too severe, no break-even spread exists, uninformed traders exit, and the market can shut down — the microstructure analog of Akerlof's lemons problem."
tags: ["microstructure", "adverse-selection", "market-breakdown", "no-trade", "lemons-problem", "market-maker"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p186:0241"
    chunk_hash: "6c877dd139340ee5b71f8f7c14a7c6aeb01128a03f53c3c0c563eae73291b177"
    page_range: [186, 187]
    quote: "may induce many or even all uninformed traders to leave the market"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p088:0131"
    chunk_hash: "cd4037431a3dd28b819641814a0c8b4e15151ce9feb8580b7980a58516201a28"
    page_range: [88, 88]
    quote: "this situation can lead to a market breakdown, where the spread is so wide that no trade occurs at all."
    edge_type: "supports"
---
# Market Viability: When Adverse Selection Causes Market Breakdown

## Intuition

In the adverse-selection models of microstructure, a quoted spread exists because the
market maker cannot tell whether the trader on the other side knows something he does
not. Every quote bundles a "premium" that protects the dealer against the chance the
counterparty is informed. So far so manageable — but O'Hara asks the sharper question:
*does a break-even quote always exist?* The answer is no. If the threat of
information-based trading is high enough, there may be **no** spread wide enough to let
the market maker break even, and trading simply halts.

The mechanism is the **market-for-lemons** logic of Akerlof, transplanted into a trading
venue. The dealer must widen the spread to cover the informed-trading risk. But a wider
spread is paid by the *uninformed* (liquidity) traders, who get nothing in return for it.
As the spread widens, more uninformed traders find it not worth transacting and withdraw.
With fewer uninformed traders left, each trade the dealer sees is *more* likely to be
informed, which forces the spread wider still, which drives out yet more uninformed
traders. The feedback can run to its terminal point: everyone uninformed has left, every
remaining trade is a sure loss to the dealer, and the only sensible quote is one at which
nobody trades.

```
   informed-trading threat HIGH
            |
            v
   dealer widens spread to break even
            |
            v
   uninformed traders find spread too costly -> exit
            |
            v
   pool of remaining traders is MORE adverse  --+
            |                                     |
            v                                     | feedback
   dealer must widen spread further  -------------+
            |
            v
   no break-even price exists  ==>  TRADING HALT / market shuts down
```

The corollary is the load-bearing fact for liquidity: a market is **viable only because
enough uninformed trade is present** to dilute the adverse-selection problem. Liquidity
trade is not noise to be filtered out — it is the precondition for the market existing at
all.

**Source:** O'Hara (1995) ch.7 §7.1 "Information and Market Viability" pp.186-187.

## Definition

**Setting.** A risk-neutral, competitive market maker posts prices to a stream of
incoming orders. A trade may originate from an *informed* trader (who holds a private
signal of the asset payoff) or from an *uninformed / liquidity* trader (who trades for
portfolio or endowment reasons unrelated to the asset's value). Let the asset payoff be a
random value `V` with prior mean `m`.

**Competitive (zero-profit) pricing.** The competitive market maker sets the price of any
trade equal to the conditional expected value of the asset given that trade. For a buy of
size `Q`, `Ask(Q) = E[V | buy of size Q]`; symmetrically for sells. Expected profit on
**every** trade is therefore zero by construction.

**Viability.** The market is **viable** if there exists a price schedule satisfying the
zero-profit (break-even) condition at which uninformed traders are still willing to
transact. The market **breaks down** (a *no-trade outcome* / trading halt) when no such
break-even price exists — i.e., the adverse-selection-induced spread is wider than the
spread any uninformed trader will accept.

**Lemons analogy.** This is a variant of Akerlof's (1970) market-for-lemons problem: the
informed side's private information degrades the average quality of the counterparty the
dealer faces, and beyond a threshold the only equilibrium is no trade.

**Source:** O'Hara (1995) ch.7 §7.1 pp.186-188 (Glosten 1989 adverse-selection / viability
framing; Glosten and Milgrom 1985 origin).

## Mathematical Reasoning

*(Notation reconstructed in clean form; O'Hara presents the formal Glosten 1989 model,
whose equations the OCR garbles — see Escalate.)*

**Conditional-expectation pricing.** With normal signals and Bayesian updating, the
posterior mean of the payoff given a noisy signal `S = V + ε` is the precision-weighted
average of prior and signal,

```
        E[V | S] = (π_V · m + π_S · S) / (π_V + π_S),
```

where `π_V`, `π_S` are the precisions (inverse variances) of prior and signal. The
competitive dealer cannot observe `S` directly; he infers it from order size `Q`, so his
quote is `P(Q) = E[V | Q]`. Observing `Q` therefore acts as a noisy signal of `V`.

**Upward-sloping schedule.** Glosten shows any differentiable competitive schedule is
**increasing in trade size**: larger orders transact at worse prices, because larger
orders are more likely informed. Write it schematically as

```
        P(Q) = m + (adverse-selection loading) · g(Q),    g increasing in |Q|.
```

If there were *no* informational trading the schedule would be flat (`g ≡ 0`) and
liquidity traders could rebalance costlessly; the positive slope is precisely the welfare
cost of asymmetric information. O'Hara notes ex-ante trader utility is **strictly lower**
with asymmetric than with symmetric information.

**Existence threshold.** A differentiable competitive equilibrium exists only if a
parameter `α` — a function of trader risk aversion and the precisions — satisfies

```
        α > 1/2.
```

Intuitively this requires that **the risk of informed trading is not so large as to
overwhelm the dealer's ability to set break-even prices.** If `α ≤ 1/2`, no
market-clearing price exists, trading is halted, and the market shuts down. Comparative
statics: anything that raises the informed-trading intensity or signal precision relative
to liquidity demand pushes `α` toward and past the threshold — i.e., more adverse
selection ⇒ wider required spread ⇒ closer to (and ultimately through) breakdown.

**Discrete elastic-demand illustration (logic, not arithmetic).** Suppose competitive
break-even pricing initially sustains both a large-trade and a small-trade market. If
large uninformed traders are *price-sensitive*, each successive widening of the
large-trade price drives one more uninformed trader out; their exit forces a further
widening, until all uninformed large traders have left. Past that point the informed
trader optimally migrates to the small-trade market, contaminating it too; if small
uninformed traders are likewise elastic, **there is no price at which trades can clear and
the market fails.** Note the key asymmetry: simply quoting large trades very high cannot
fix this, because the informed trader just switches to the profit-maximizing quantity —
informed order flow is present in *any* equilibrium.

**Source:** O'Hara (1995) ch.7 §7.1 pp.187-191 (Glosten 1989 competitive solution,
existence condition `α > 1/2`, discrete elastic-demand example).

## Boundary Notes

- **Assumptions that drive the result.** Competitive (zero-profit, trade-by-trade) pricing
  + *elastic* uninformed demand. With perfectly *inelastic* liquidity demand (as in the
  baseline Glosten-Milgrom sequential model) the spread can stay finite and the market
  always clears; breakdown is what appears once liquidity traders are allowed to withdraw
  in response to a wide spread.
- **When it holds vs. breaks.** Breakdown bites when the informed-trading threat is large
  relative to liquidity-trading volume (existence parameter through its threshold). It does
  **not** bite when uninformed order flow is plentiful enough to dilute adverse selection —
  the empirical norm, which is why halts are real but infrequent.
- **Mechanism-design escape routes (siblings, not this card).** O'Hara shows the breakdown
  is *not* inevitable given the right trading mechanism. (i) A **monopolist specialist** can
  cross-subsidize — lose on large (informed-laden) trades, profit on frequent small trades —
  averaging across trade sizes to keep the market open; under breakdown conditions, welfare
  can be *higher* under monopoly than competition. (ii) A **call auction** (Madhavan 1992)
  can also restore viability. These mechanism remedies are distinct cards; this card is
  scoped to the breakdown condition itself.
- **Contrast with inventory models.** Here the spread and the breakdown are driven purely
  by *adverse selection / asymmetric information*, not by dealer inventory risk. The
  no-trade outcome is an information phenomenon, not an inventory-management one.
- **Lineage.** The no-trade outcome connects to the rational-expectations no-trade theorems
  (Milgrom-Stokey 1982) and the Walrasian breakdown characterization (Bhattacharya-Spiegel
  1991); O'Hara cites these as the same difficulty in a different framework.

**Source:** O'Hara (1995) ch.7 §7.1 pp.186-193; Foucault, Pagano & Röell (2013) §3.3.2
p.88 (elastic liquidity demand ⇒ complete market freeze).

## See Also

- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) — the
  sequential-trade model whose break-even quote, taken to its limit under elastic liquidity
  demand, produces this no-trade breakdown.
- [`mt-liquidity-nature-provision-return`](./mt-liquidity-nature-provision-return.md) — why
  uninformed (liquidity) trade is the precondition for market viability rather than mere
  noise.

## Escalate to Raw When

The OCR garbles Glosten's (1989) formal solution: equations (7.1)–(7.10) — the certainty-
equivalent objective, the first-order condition, the differentiable competitive pricing
schedule, and the constants `N`, `α`, `γ` defining the `α > 1/2` existence threshold — are
unreadable in the extracted text and have been reconstructed here only schematically.
Re-read **O'Hara (1995) ch.7 §7.1, pp.187–191** for the full algebra of the competitive
schedule and the precise definition of `α`, and **pp.191–193** for the monopolist-specialist
incentive-compatibility program (eqs. 7.9–7.10) and the discrete numerical example, to
confirm any quantitative claim before relying on it.
