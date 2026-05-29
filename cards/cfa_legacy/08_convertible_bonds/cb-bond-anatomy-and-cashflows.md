---
schema_version: "cacg.v0"
id: "cb-bond-anatomy-and-cashflows"
title: "Convertible Bond Anatomy and Cashflows"
reading_id: "08_convertible_bonds"
summary: "A convertible bond is a corporate-debt instrument whose holder owns an embedded right to surrender the bond for a fixed number of issuer shares; until conversion or maturity it pays coupons like an ordinary corporate bond, with conversion price K_c = F / q derived from face F and conversion ratio q, and call/put provisions modifying the bond-plus-call decomposition."
tags: ["convertible-bonds", "bond-anatomy"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p045:0050"
    chunk_hash: "36d0e915b9efae7a3e8aa407ef024af6169687bca4eb32a53e0778e7e98c95bf"
    page_range: [45, 46]
    quote: "The bond floor or the investment value is the value of the convertible if it were to be stripped of the possibility to convert into the underlying shares."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p021:0019"
    chunk_hash: "b929674cbb30422015f513c673dbb0e571e39c6b721962595dcff309e5b0ae49"
    page_range: [21, 22]
    quote: "Convertible securities are hybrid issues that have fixed-income and equity characteristics."
    edge_type: "supports"
  - source_id: "cb_zubulake_convertible_securities_worldwide"
    chunk_id: "cb_zubulake_convertible_securities_worldwide:p027:0019"
    chunk_hash: "93f3fc0e57e36b56d0410720e08fb6617dff2f9f473ecab0bf4635bd4594d416"
    page_range: [27, 28]
    quote: "Convertible bonds are issued in either domestic, foreign, or Eurobond form."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2820:4213"
    chunk_hash: "7790a3dd4fc3cf226f70a56f5e4df653bab79a296b6a978db913c2103e297d08"
    page_range: [2820, 2821]
    quote: "For example, options can be combined with bonds to form either callable bonds or convertible bonds."
    edge_type: "supports"
card_hash: "4b80e7813ffd1c508662b98019a0446fb8611b91a78c8da36c495c7820fe82e1"
---
# Convertible Bond Anatomy and Cashflows

## Intuition

A convertible bond (CB) is a corporate-debt instrument whose holder also owns
an embedded right to surrender the bond in exchange for a fixed number of the
issuer's shares. Until conversion or maturity, it pays coupons like an
ordinary corporate bond; if the holder converts, the bond ceases to exist and
the holder receives shares instead.
**Source:** DeSpiegeleer et al. (2014) §2.1-§2.2 pp.21-30; Calamos (2003) §1
pp.3-12.

```
issuance
   |
   |  face F, coupon c, maturity T,
   v  conversion ratio q (shares per face F)
+-----+      coupon c    +---------+   coupon c   +---------+
|     |--------------> | holder    |------------> | holder  |
| t=0 |  pay F to        | t = 1Y    |   ...        | t = T-1 |
+-----+  issuer          +---------+              +---------+
                              |                         |
                              |hold (no conv.)          | redeem F
                              v                         | OR convert q shares
                          continue                      v
                                                    maturity
```

## Definition

A convertible bond is the bundle `(F, c, T, q, S, callability, putability,
seniority)` where `F` is face value, `c` is the coupon rate, `T` is legal
maturity, `q` is the conversion ratio, `S` is the issuer's underlying common
share, and the remaining fields enumerate optional issuer-call and holder-put
rights and the bond's seniority in the capital structure.
**Source:** DeSpiegeleer et al. (2014) §2.2 pp.25-30.

The conversion price `K_c = F / q` is implied by `(F, q)`; markets often quote
`K_c` directly even though `q` is the prospectus primitive. **Source:**
DeSpiegeleer et al. (2014) §2.2 pp.27-29.

## Mathematical Reasoning

The pre-conversion cash-flow stream of an ordinary CB is identical to the
straight-bond stream of the same `(F, c, T)`: coupons `c · F` paid at the
contractual coupon dates and face `F` paid at maturity if the bond is not
converted. The convertible adds the embedded right at any (or specific)
conversion-eligible date `t_k ∈ T_conv ⊆ [0, T]` to exchange the remaining
stream for `q · S(t_k)` worth of equity. **Source:** DeSpiegeleer et al.
(2014) §2.4 pp.30-40.

At any conversion-eligible date `t ∈ T_conv`, no-arbitrage forces
`V(t) ≥ q · S(t)` because the holder could otherwise convert immediately for
risk-free profit, and `V(t) ≥ B(t)` (the credit-risky straight-bond floor; see
the [bond-floor card](./cb-bond-floor-investment-value.md#definition)),
yielding `V(t) ≥ max(q · S(t), B(t))`; the inequality is in general strict by
the unexercised early-conversion option's time value. **Source:**
DeSpiegeleer et al. (2014) §2.3-§2.4 pp.30-40; Calamos (2003) §1 pp.3-12.

The CB's coupon `c` is typically lower than the issuer's straight-bond
coupon `c_straight` for the same seniority and tenor; the gap is the price
the holder pays for the embedded conversion right (see the
[conversion-mechanics card](./cb-conversion-feature-mechanics.md#definition)).
**Source:** DeSpiegeleer et al. (2014) §2.3 pp.30-32.

### Cash-Flow Ladder

The contractual cash-flow stream — face `F` paid by the holder at
issuance, periodic coupon `c` paid by the issuer, and either redemption
of `F` at maturity or holder conversion into `q` shares — plots as a
ladder over time. **Source:** DeSpiegeleer et al. (2014) §2.1-§2.2
pp.21-30; Calamos (2003) §1 pp.3-12.

```
<!-- primitive: cb-cashflow-ladder source: _diagram_primitives.md -->
issuance
   |
   |  face F, coupon c, maturity T,
   v  conversion ratio q
+-----+    coupon c   +-------+    coupon c    +-------+
|     |------------> | holder |--------------> | holder|
| t=0 | pay F to       | t=1Y    |    ...           | t=T-1 |
+-----+ issuer         +-------+                  +-------+
                            |                          |
                            |hold (no conv.)           | redeem F
                            v                          | OR convert
                       continue                        v
                                                  maturity
```

## See Also

- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — how the embedded right is exercised
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — `B(t)` as the credit-risky lower bound
- [`cb-parity-and-conversion-value.md`](cb-parity-and-conversion-value.md) — `q · S(t)` as the other lower bound

## Escalate to Raw When

Open the prospectus or DeSpiegeleer §2.2 pp.25-30 directly when any of the
criteria below applies. **Source:** DeSpiegeleer et al. (2014) §2.2
pp.25-30.

- The bond has non-standard features (PIK coupons, step-up coupons,
  cross-default triggers, change-of-control puts, tax events).
  **Source:** DeSpiegeleer et al. (2014) §2.2 pp.25-30.
- Coupon-frequency or day-count conventions matter for accrued-interest
  calculations beyond the level of this card. **Source:** DeSpiegeleer et al.
  (2014) §2.2 pp.25-30.
- The issuer is regulated and the bond may be a contingent convertible (CoCo)
  with capital-trigger conversion mechanics — those follow a different
  decomposition than the holder-discretion case. **Source:** DeSpiegeleer
  et al. (2014) §1.5 pp.16-20.
