---
schema_version: "cacg.v0"
id: "pa-fi-carry-rolldown-pulltopar-time-decomposition"
title: "Carry, Roll-Down, and Pull-to-Par Time Decomposition"
reading_id: "15_performance_and_attribution"
summary: "Decomposes the market-closed deterministic bond return into carry (yield times elapsed time), pull-to-par, and roll-down (riding the curve), with carry sub-split into risk-free/credit and running/pull-to-par components."
tags: ["fixed-income-attribution", "carry-return", "rolldown"]
citations:
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p175:0184"
    chunk_hash: "352925f520371ab3473911ab041bc29b29502ddf3c3889f2e0db5509d7ceef5a"
    page_range: [175, 175]
    quote: "However, rolldown return is seldom substantial, and will be overwhelmed by even quite small changes in the level or shape of the curve."
    edge_type: "defines"
---
# Carry, Roll-Down, and Pull-to-Par Time Decomposition

## Intuition

What is the return of a bond portfolio when the markets are closed? For equities the answer is zero — prices have not moved. For fixed income it is not, because a bond is engineered to throw off a trickle of return every day, weekends and holidays included: as the next coupon draws closer the bond's value rises, whether or not a single trade prints. This is the **carry** (also called yield, time, or coupon return), and it is the deterministic, market-closed component of fixed income return.

**Source:** Colin (2016) §9.1 PDF p.136 (printed p.113)

But two more sources of return arise even when the *market* (the yield curve and spreads) is held fixed. **Pull-to-par** is the force that drags a bond's price toward par as maturity approaches: a discount bond is pulled up, a premium bond pulled down, regardless of where rates sit. **Roll-down** is the gain (or loss) from a security ageing along a sloped curve — a 1-year bond becomes an 11-month bond a month later, and is repriced off a lower (on an upward-sloped curve) or higher yield point. Colin is emphatic that roll-down is its own animal: it is not market return (we assumed the curve was unchanged) and it is not pure elapsed-time carry (it is driven by the *shape* of the curve), so it must be reported on its own line.

**Source:** Colin (2016) §12.3 PDF p.175 (printed p.152)

## Definition

- **Carry return** over an interval `dt`: `r_carry = y * dt`, where `y` is the bond's yield to maturity and `dt` the elapsed fraction of a year. Using YTM is as accurate as repricing from first principles and far less work, since YTM is usually already available.
- **Pull-to-par return**: the return arising because the price must converge to par at maturity, present *even if the curve is flat*; it is driven by the gap between current price and eventual par.
- **Roll-down return**: the return from the security's cash flows ageing to a different point on an unchanged curve; driven *entirely* by the curve's shape. Roll-down change in yield is `dy = Y_t(m1) - Y_t(m2)` with `m2 < m1`, and the return is `r_rolldown = MD * dy_rolldown` (computed the same way as a yield-driven curve return, via modified duration). Also called *riding the yield curve*.

**Source:** Colin (2016) §9.4 (PDF p.139 / printed p.116), §12.3 (PDF p.176 / printed p.153)

Carry itself admits two reportable sub-splits, both built from decomposing the YTM:

- **Running yield vs. pull-to-par**: `y_YTM = y_running_yield + y_pull_to_par`. Running yield is coupon over clean price (income only); pull-to-par captures the capital convergence YTM additionally embeds.
- **Risk-free vs. credit carry**: `y_YTM = y_risk_free_yield + y_credit_yield`. The credit carry is the extra yield a lower-rated bond carries over its treasury equivalent, and it generates carry return whether or not spreads move.

**Source:** Colin (2016) §9.6, §9.8 PDF pp.140-141 (printed pp.117-118)

## Mathematical Reasoning

The market-closed return identity layers three deterministic, curve-unchanged sources, each splitting further down the tree:

```
        Deterministic (market-closed) return
        +---------------+---------------+--------------+
     CARRY            PULL-TO-PAR     ROLL-DOWN
   r = y*dt        price -> par      MD * dy_rolldown
        |           (flat-curve OK)  (needs sloped curve)
   +----+----+
 split (a)   split (b)
 running     risk-free carry
 + pull-to-  + credit carry
   par        y = y_rf + y_credit
 y = y_run
   + y_p2p
```

**Source:** Colin (2016) §9.6, §9.8 (PDF pp.140-141 / printed pp.117-118), §12.3 (PDF p.176 / printed p.153)

Two limiting cases sharpen the distinction between pull-to-par and roll-down. Colin asserts: *pull-to-par return is generated even if the yield curve is flat*, since it is driven by the price-versus-par gap; *in contrast, roll-down return is driven entirely by the shape of the yield curve* — a flat curve yields no roll-down because `Y_t(m1) = Y_t(m2)`. The two also vanish under different degeneracies: a perpetual bond (no maturity date) generates no pull-to-par return, while a discount-instrument such as a bank bill generates no running-yield carry. These are stated by the source, which does not formally prove them beyond the structural argument given; the card asserts to the same depth.

**Source:** Colin (2016) §9.9 (PDF p.143 / printed p.120), §12.3 (PDF p.176 / printed p.153)

The reportable re-aggregation. Although carry, treasury (curve), and credit are measured separately, the market convention is to *roll them back up* into a small, trader-legible effect tree. In Colin's treatment the canonical re-aggregation is the Campisi model, described as "perhaps the simplest possible security-level" model for fixed income attribution: total return splits into an income return (which "is actually running yield, as it omits any pull-to-par effects"), a treasury effect (`-MD*dy_treasury`, combining parallel and non-parallel curve moves), a spread effect (`-MD*dy_spread`), and a residual-free selection effect (total minus the prior three, with no residual term). The income / treasury / spread / selection tree is the standard reporting face that the finer carry sub-splits described above feed into; see the sibling perturbational-equation card for the `r = c - MD*dy` engine underneath it. (Worked Campisi/duration-model numbers and the full effect tree live beyond this card's page range — escalate to the raw for them.)

**Source:** Colin (2016) §23.1 PDF pp.278-279 (printed pp.255-256)

## See Also

- [`pa-fi-perturbational-attribution-equation.md`](pa-fi-perturbational-attribution-equation.md) — the `r = c - MD*dy` engine that turns each yield change into a return component.
- [`pa-fi-shift-twist-butterfly-and-krd.md`](pa-fi-shift-twist-butterfly-and-krd.md) — decomposes the market-*open* curve return that roll-down is explicitly held distinct from.
- [`pa-fi-parametric-vs-nonparametric-curve-models.md`](pa-fi-parametric-vs-nonparametric-curve-models.md) — the curve representation whose slope drives roll-down.

The Campisi income/treasury/spread/selection tree referenced above is the reportable re-aggregation face; the duration model and van Breukelen attribution extend it. Carry-versus-active framing connects to pm-* active-return decomposition, while the credit-carry sub-split feeds rm-* credit-spread risk attribution.

## Escalate to Raw When

- You need the worked carry/pull-to-par/roll-down arithmetic (e.g. the 7/365 carry example, the 105 bp pull-to-par worked figure, or the roll-down yield-pickup example) — Colin (2016) §9.4 and §12.3.
- You need the full Campisi / duration-model effect-tree formulas and the reportable income-treasury-spread-selection breakdown with numbers — Colin (2016) Ch.23 (beyond this card's page range).
- You need the optionality, inflation, paydown, or liquidity carry sub-categories (yield-to-call, real-yield carry, MBS paydown return `r_p = (100-P)/P * df`) — Colin (2016) §9.8.4-§9.8.6, §12.1, §12.4.
