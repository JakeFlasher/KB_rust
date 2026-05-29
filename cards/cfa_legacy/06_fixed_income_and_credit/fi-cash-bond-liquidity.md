---
schema_version: "cacg.v0"
id: "fi-cash-bond-liquidity"
title: "Cash-Bond Liquidity"
reading_id: "06_fixed_income_and_credit"
summary: "Liquidity dimensions of cash-bond markets: bid-ask, dealer inventory, on-the-run / off-the-run benchmarks, and repo specialness as the liquidity-premium machinery for credit spreads."
tags: ["fixed-income", "cash-bond"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2406:3528"
    chunk_hash: "5ea89cd028cf18934eca1ece67c78a3bca26622a0ec41f07635cd41c2c37ba92"
    page_range: [2406, 2407]
    quote: "Problems with Libor first arose during the 2007–2009 global financial crisis, when the perceived default and liquidity risks of major international banks rose significantly."
    edge_type: "defines"
  - source_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding"
    chunk_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding:p068:0084"
    chunk_hash: "9db0028d106b97431b080151e74555800ce41f6764ecc6c525a12a7fbe051df3"
    page_range: [68, 70]
    quote: "Commonly proposed explanations for the corresponding spreads refer to a combined effect of credit risk and liquidity risk."
    edge_type: "supports"
card_hash: "5cf894da58849405cb53490d08605145573b331f92973f745c19d2d59e85d234"
---
# Cash-Bond Liquidity

## Intuition

Liquidity in cash-bond markets is not uniform: a current
on-the-run Treasury benchmark trades at tight bid-ask
spreads with deep dealer inventory; an off-the-run bond
of similar maturity trades wider. The liquidity premium
is the additional yield investors demand for holding
the less-liquid bond — it is one of the components of
the credit-spread risk premium from
[`fi-credit-risk-fundamentals.md`](./fi-credit-risk-fundamentals.md#mathematical-reasoning).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.50-100.

```
bid-ask spread (bps)
   ^
   |   off-the-run bond           illiquid corporate
   |    *                          bond (rarely traded)
   |     *  *                            *  *
   |       *  *  *                       *
   |              *  *  *  *
   |                       *  *  *
   |  on-the-run Treasury        *  *
   |     *
   +----------------------------------> liquidity
   tight bid-ask <--->                    wide bid-ask
```

## Definition

Bid-ask spread is the difference between the price a
dealer offers to buy (bid) and the price the dealer
offers to sell (ask). For Treasuries the spread can be
sub-basis-point on benchmark issues; for off-the-run
issues spreads widen; for high-yield corporate bonds
spreads can exceed 50 basis points. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.50-100.

On-the-run vs off-the-run: the on-the-run is the most
recently issued benchmark Treasury at a given maturity;
off-the-run is any earlier-issued Treasury of similar
remaining tenor. The on-the-run premium (price above
the off-the-run cohort) reflects the liquidity premium
investors pay for benchmark status. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.50-100.

Repo specialness: a Treasury that is in high demand for
short-covering trades commands a "special" repo rate
below general collateral. The specialness is the
implied repo discount; bonds that frequently trade
"special" benefit from a funding-cost subsidy that is
priced into their cash-bond yield. **Source:**
Crepey+Bielecki+Brigo (2014) §4 pp.150-200.

## Mathematical Reasoning

The liquidity premium contributes to the spread
decomposition: total spread = expected loss + credit
risk premium + liquidity premium + tax premium. The
liquidity component varies cyclically — it widens in
stressed markets when dealer balance sheets shrink and
buy-side de-leveraging forces fire-sales of less-liquid
bonds. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.50-100.

The credit-spread machinery from
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning)
absorbs liquidity into the wedge between physical and
risk-neutral hazard, but the absorption is qualitative.
Empirical decomposition of bond spreads into pure-credit
and pure-liquidity components requires reference to
CDS spreads (which carry less liquidity premium) and is
contested in the literature. **Source:** Crepey+
Bielecki+Brigo (2014) §4 pp.150-200.

Repo-special funding affects the cash bond's yield
because a holder who can lend the bond out at a
special rate effectively earns the specialness as
additional carry. The yield observed in the cash
market is therefore not the pure credit + tenor yield
but a composite that includes the funding-side
subsidy. **Source:** Crepey+Bielecki+Brigo (2014) §4
pp.150-200.

The bond anatomy of
[`fi-bond-anatomy-and-cashflows.md`](./fi-bond-anatomy-and-cashflows.md#mathematical-reasoning)
ignores liquidity effects; the contractual stream is
the same regardless of dealer inventory. Liquidity
premium is a property of the holder's market access,
not of the issuer's contract. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.50-100.

## See Also

- [`fi-credit-risk-fundamentals.md`](fi-credit-risk-fundamentals.md) — total spread decomposition that absorbs liquidity premium
- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — issuer-contract baseline that liquidity does not alter

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 42 or Crepey
Chapter 4 directly when any of the criteria below
applies. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.50-100; Crepey+Bielecki+Brigo (2014) §4
pp.150-200.

- A specific liquidity-premium decomposition (Roll
  bid-ask, Amihud illiquidity, dealer-inventory
  proxies) is required. **Source:** CFA L1 Curriculum
  (2022) Vol.5/pp.50-100.
- Cross-currency / cross-tenor liquidity premia are
  in scope; this card covers single-currency single-
  tenor cases. **Source:** Crepey+Bielecki+Brigo
  (2014) §4 pp.150-200.
- A specific repo-special trade or sovereign-issuance
  calendar effect requires desk-level micro-detail
  outside this card's scope. **Source:** CFA L1
  Curriculum (2022) Vol.5/pp.50-100.
